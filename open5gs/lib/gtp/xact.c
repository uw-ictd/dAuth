/*
 * Copyright (C) 2019 by Sukchan Lee <acetcom@gmail.com>
 * Copyright (C) 2022 by sysmocom - s.f.m.c. GmbH <info@sysmocom.de>
 *
 * This file is part of Open5GS.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#include "ogs-gtp.h"
#include "ogs-app.h"

typedef enum {
    GTP_XACT_UNKNOWN_STAGE,
    GTP_XACT_INITIAL_STAGE,
    GTP_XACT_INTERMEDIATE_STAGE,
    GTP_XACT_FINAL_STAGE,
} ogs_gtp_xact_stage_t;

static int ogs_gtp_xact_initialized = 0;
static uint32_t g_xact_id = 0;

static OGS_POOL(pool, ogs_gtp_xact_t);

static ogs_gtp_xact_t *ogs_gtp_xact_remote_create(ogs_gtp_node_t *gnode, uint8_t gtp_version, uint32_t sqn);
static ogs_gtp_xact_stage_t ogs_gtp_xact_get_stage(uint8_t type, uint32_t sqn);
static ogs_gtp_xact_stage_t ogs_gtp1_xact_get_stage(uint8_t type, uint32_t sqn);
static int ogs_gtp_xact_delete(ogs_gtp_xact_t *xact);
static int ogs_gtp_xact_update_rx(ogs_gtp_xact_t *xact, uint8_t type);
static ogs_gtp_xact_t *ogs_gtp_xact_find_by_xid(
        ogs_gtp_node_t *gnode, uint8_t type, uint8_t gtp_version, uint32_t xid);

static void response_timeout(void *data);
static void holding_timeout(void *data);

int ogs_gtp_xact_init(void)
{
    ogs_assert(ogs_gtp_xact_initialized == 0);

    ogs_pool_init(&pool, ogs_app()->pool.gtp_xact);

    g_xact_id = 0;

    ogs_gtp_xact_initialized = 1;

    return OGS_OK;
}

void ogs_gtp_xact_final(void)
{
    ogs_assert(ogs_gtp_xact_initialized == 1);

    ogs_pool_final(&pool);

    ogs_gtp_xact_initialized = 0;
}

ogs_gtp_xact_t *ogs_gtp1_xact_local_create(ogs_gtp_node_t *gnode,
        ogs_gtp1_header_t *hdesc, ogs_pkbuf_t *pkbuf,
        void (*cb)(ogs_gtp_xact_t *xact, void *data), void *data)
{
    int rv;
    char buf[OGS_ADDRSTRLEN];
    ogs_gtp_xact_t *xact = NULL;

    ogs_assert(gnode);
    ogs_assert(hdesc);

    ogs_pool_alloc(&pool, &xact);
    ogs_assert(xact);
    memset(xact, 0, sizeof *xact);
    xact->index = ogs_pool_index(&pool, xact);

    xact->gtp_version = 1;
    xact->org = OGS_GTP_LOCAL_ORIGINATOR;
    xact->xid = OGS_NEXT_ID(g_xact_id,
            OGS_GTP1_MIN_XACT_ID, OGS_GTP1_MAX_XACT_ID);
    xact->gnode = gnode;
    xact->cb = cb;
    xact->data = data;

    xact->tm_response = ogs_timer_add(
            ogs_app()->timer_mgr, response_timeout, xact);
    ogs_assert(xact->tm_response);
    xact->response_rcount = ogs_app()->time.message.gtp.n3_response_rcount,

    xact->tm_holding = ogs_timer_add(
            ogs_app()->timer_mgr, holding_timeout, xact);
    ogs_assert(xact->tm_holding);
    xact->holding_rcount = ogs_app()->time.message.gtp.n3_holding_rcount,

    ogs_list_add(xact->org == OGS_GTP_LOCAL_ORIGINATOR ?
            &xact->gnode->local_list : &xact->gnode->remote_list, xact);

    rv = ogs_gtp1_xact_update_tx(xact, hdesc, pkbuf);
    if (rv != OGS_OK) {
        ogs_error("ogs_gtp_xact_update_tx(rv=%d) failed", (int)rv);
        ogs_gtp_xact_delete(xact);
        return NULL;
    }

    ogs_debug("[%d] %s Create  peer [%s]:%d",
            xact->xid,
            xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
            OGS_ADDR(&gnode->addr, buf),
            OGS_PORT(&gnode->addr));

    return xact;
}

ogs_gtp_xact_t *ogs_gtp_xact_local_create(ogs_gtp_node_t *gnode,
        ogs_gtp2_header_t *hdesc, ogs_pkbuf_t *pkbuf,
        void (*cb)(ogs_gtp_xact_t *xact, void *data), void *data)
{
    int rv;
    char buf[OGS_ADDRSTRLEN];
    ogs_gtp_xact_t *xact = NULL;

    ogs_assert(gnode);
    ogs_assert(hdesc);

    ogs_pool_alloc(&pool, &xact);
    ogs_assert(xact);
    memset(xact, 0, sizeof *xact);
    xact->index = ogs_pool_index(&pool, xact);

    xact->gtp_version = 2;
    xact->org = OGS_GTP_LOCAL_ORIGINATOR;
    xact->xid = OGS_NEXT_ID(g_xact_id,
            OGS_GTP_MIN_XACT_ID, OGS_GTP_CMD_XACT_ID);
    if (hdesc->type == OGS_GTP2_MODIFY_BEARER_COMMAND_TYPE ||
        hdesc->type == OGS_GTP2_DELETE_BEARER_COMMAND_TYPE ||
        hdesc->type == OGS_GTP2_BEARER_RESOURCE_COMMAND_TYPE) {
        xact->xid |= OGS_GTP_CMD_XACT_ID;
    }
    xact->gnode = gnode;
    xact->cb = cb;
    xact->data = data;

    xact->tm_response = ogs_timer_add(
            ogs_app()->timer_mgr, response_timeout, xact);
    ogs_assert(xact->tm_response);
    xact->response_rcount = ogs_app()->time.message.gtp.n3_response_rcount,

    xact->tm_holding = ogs_timer_add(
            ogs_app()->timer_mgr, holding_timeout, xact);
    ogs_assert(xact->tm_holding);
    xact->holding_rcount = ogs_app()->time.message.gtp.n3_holding_rcount,

    ogs_list_add(xact->org == OGS_GTP_LOCAL_ORIGINATOR ?
            &xact->gnode->local_list : &xact->gnode->remote_list, xact);

    rv = ogs_gtp_xact_update_tx(xact, hdesc, pkbuf);
    if (rv != OGS_OK) {
        ogs_error("ogs_gtp_xact_update_tx(rv=%d) failed", (int)rv);
        ogs_gtp_xact_delete(xact);
        return NULL;
    }

    ogs_debug("[%d] %s Create  peer [%s]:%d",
            xact->xid,
            xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
            OGS_ADDR(&gnode->addr, buf),
            OGS_PORT(&gnode->addr));

    return xact;
}

static ogs_gtp_xact_t *ogs_gtp_xact_remote_create(ogs_gtp_node_t *gnode, uint8_t gtp_version, uint32_t sqn)
{
    char buf[OGS_ADDRSTRLEN];
    ogs_gtp_xact_t *xact = NULL;

    ogs_assert(gnode);

    ogs_pool_alloc(&pool, &xact);
    ogs_assert(xact);
    memset(xact, 0, sizeof *xact);
    xact->index = ogs_pool_index(&pool, xact);

    xact->gtp_version = gtp_version;
    xact->org = OGS_GTP_REMOTE_ORIGINATOR;
    xact->xid = (gtp_version == 1) ? OGS_GTP1_SQN_TO_XID(sqn) : OGS_GTP2_SQN_TO_XID(sqn);
    xact->gnode = gnode;

    xact->tm_response = ogs_timer_add(
            ogs_app()->timer_mgr, response_timeout, xact);
    ogs_assert(xact->tm_response);
    xact->response_rcount = ogs_app()->time.message.gtp.n3_response_rcount,

    xact->tm_holding = ogs_timer_add(
            ogs_app()->timer_mgr, holding_timeout, xact);
    ogs_assert(xact->tm_holding);
    xact->holding_rcount = ogs_app()->time.message.gtp.n3_holding_rcount,

    ogs_list_add(xact->org == OGS_GTP_LOCAL_ORIGINATOR ?
            &xact->gnode->local_list : &xact->gnode->remote_list, xact);

    ogs_debug("[%d] %s Create  peer [%s]:%d",
            xact->xid,
            xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
            OGS_ADDR(&gnode->addr, buf),
            OGS_PORT(&gnode->addr));

    return xact;
}

ogs_gtp_xact_t *ogs_gtp_xact_cycle(ogs_gtp_xact_t *xact)
{
    return ogs_pool_cycle(&pool, xact);
}

void ogs_gtp_xact_delete_all(ogs_gtp_node_t *gnode)
{
    ogs_gtp_xact_t *xact = NULL, *next_xact = NULL;

    ogs_list_for_each_safe(&gnode->local_list, next_xact, xact)
        ogs_gtp_xact_delete(xact);
    ogs_list_for_each_safe(&gnode->remote_list, next_xact, xact)
        ogs_gtp_xact_delete(xact);
}

int ogs_gtp1_xact_update_tx(ogs_gtp_xact_t *xact,
        ogs_gtp1_header_t *hdesc, ogs_pkbuf_t *pkbuf)
{
    char buf[OGS_ADDRSTRLEN];
    ogs_gtp_xact_stage_t stage;
    ogs_gtp1_header_t *h = NULL;
    int gtp_hlen = 0;

    ogs_assert(xact);
    ogs_assert(xact->gnode);
    ogs_assert(hdesc);
    ogs_assert(pkbuf);

    ogs_debug("[%d] %s UPD TX-%d  peer [%s]:%d",
            xact->xid,
            xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
            hdesc->type,
            OGS_ADDR(&xact->gnode->addr, buf),
            OGS_PORT(&xact->gnode->addr));

    stage = ogs_gtp1_xact_get_stage(hdesc->type, xact->xid);
    if (xact->org == OGS_GTP_LOCAL_ORIGINATOR) {
        switch (stage) {
        case GTP_XACT_INITIAL_STAGE:
            if (xact->step != 0) {
                ogs_error("invalid step[%d]", xact->step);
                ogs_pkbuf_free(pkbuf);
                return OGS_ERROR;
            }
            break;

        case GTP_XACT_INTERMEDIATE_STAGE:
            ogs_expect(0);
            ogs_pkbuf_free(pkbuf);
            return OGS_ERROR;

        case GTP_XACT_FINAL_STAGE:
            if (xact->step != 2) {
                ogs_error("invalid step[%d]", xact->step);
                ogs_pkbuf_free(pkbuf);
                return OGS_ERROR;
            }
            break;

        default:
            ogs_assert_if_reached();
            break;
        }
    } else if (xact->org == OGS_GTP_REMOTE_ORIGINATOR) {
        switch (stage) {
        case GTP_XACT_INITIAL_STAGE:
            ogs_expect(0);
            ogs_pkbuf_free(pkbuf);
            return OGS_ERROR;

        case GTP_XACT_INTERMEDIATE_STAGE:
        case GTP_XACT_FINAL_STAGE:
            if (xact->step != 1) {
                ogs_error("invalid step[%d]", xact->step);
                ogs_pkbuf_free(pkbuf);
                return OGS_ERROR;
            }
            break;

        default:
            ogs_error("invalid stage[%d]", stage);
            ogs_pkbuf_free(pkbuf);
            return OGS_ERROR;
        }
    } else {
        ogs_error("invalid org[%d]", xact->org);
        ogs_pkbuf_free(pkbuf);
        return OGS_ERROR;
    }

    gtp_hlen = OGS_GTPV1C_HEADER_LEN;


    ogs_pkbuf_push(pkbuf, gtp_hlen);
    h = (ogs_gtp1_header_t *)pkbuf->data;
    memset(h, 0, gtp_hlen);

    h->version = 1;
    h->type = hdesc->type;
    h->pt = 1; /* GTP */
    h->teid = htobe32(hdesc->teid);

    h->s = 1;
    h->sqn = OGS_GTP1_XID_TO_SQN(xact->xid);
    h->length = htobe16(pkbuf->len - 8);

    /* Save Message type and packet of this step */
    xact->seq[xact->step].type = h->type;
    xact->seq[xact->step].pkbuf = pkbuf;

    /* Step */
    xact->step++;

    return OGS_OK;
}

int ogs_gtp_xact_update_tx(ogs_gtp_xact_t *xact,
        ogs_gtp2_header_t *hdesc, ogs_pkbuf_t *pkbuf)
{
    char buf[OGS_ADDRSTRLEN];
    ogs_gtp_xact_stage_t stage;
    ogs_gtp2_header_t *h = NULL;
    int gtp_hlen = 0;

    ogs_assert(xact);
    ogs_assert(xact->gnode);
    ogs_assert(hdesc);
    ogs_assert(pkbuf);

    ogs_debug("[%d] %s UPD TX-%d  peer [%s]:%d",
            xact->xid,
            xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
            hdesc->type,
            OGS_ADDR(&xact->gnode->addr, buf),
            OGS_PORT(&xact->gnode->addr));

    stage = ogs_gtp_xact_get_stage(hdesc->type, xact->xid);
    if (xact->org == OGS_GTP_LOCAL_ORIGINATOR) {
        switch (stage) {
        case GTP_XACT_INITIAL_STAGE:
            if (xact->step != 0) {
                ogs_error("invalid step[%d]", xact->step);
                ogs_pkbuf_free(pkbuf);
                return OGS_ERROR;
            }
            break;

        case GTP_XACT_INTERMEDIATE_STAGE:
            ogs_expect(0);
            ogs_pkbuf_free(pkbuf);
            return OGS_ERROR;

        case GTP_XACT_FINAL_STAGE:
            if (xact->step != 2) {
                ogs_error("invalid step[%d]", xact->step);
                ogs_pkbuf_free(pkbuf);
                return OGS_ERROR;
            }
            break;

        default:
            ogs_assert_if_reached();
            break;
        }
    } else if (xact->org == OGS_GTP_REMOTE_ORIGINATOR) {
        switch (stage) {
        case GTP_XACT_INITIAL_STAGE:
            ogs_expect(0);
            ogs_pkbuf_free(pkbuf);
            return OGS_ERROR;

        case GTP_XACT_INTERMEDIATE_STAGE:
        case GTP_XACT_FINAL_STAGE:
            if (xact->step != 1) {
                ogs_error("invalid step[%d]", xact->step);
                ogs_pkbuf_free(pkbuf);
                return OGS_ERROR;
            }
            break;

        default:
            ogs_error("invalid stage[%d]", stage);
            ogs_pkbuf_free(pkbuf);
            return OGS_ERROR;
        }
    } else {
        ogs_error("invalid org[%d]", xact->org);
        ogs_pkbuf_free(pkbuf);
        return OGS_ERROR;
    }

    if (hdesc->type > OGS_GTP2_VERSION_NOT_SUPPORTED_INDICATION_TYPE) {
        gtp_hlen = OGS_GTPV2C_HEADER_LEN;
    } else {
        gtp_hlen = OGS_GTPV2C_HEADER_LEN - OGS_GTP2_TEID_LEN;
    }

    ogs_pkbuf_push(pkbuf, gtp_hlen);
    h = (ogs_gtp2_header_t *)pkbuf->data;
    memset(h, 0, gtp_hlen);

    h->version = 2;
    h->type = hdesc->type;

    if (hdesc->type > OGS_GTP2_VERSION_NOT_SUPPORTED_INDICATION_TYPE) {
        h->teid_presence = 1;
        h->teid = htobe32(hdesc->teid);
        h->sqn = OGS_GTP2_XID_TO_SQN(xact->xid);
    } else {
        h->teid_presence = 0;
        h->sqn_only = OGS_GTP2_XID_TO_SQN(xact->xid);
    }
    h->length = htobe16(pkbuf->len - 4);

    /* Save Message type and packet of this step */
    xact->seq[xact->step].type = h->type;
    xact->seq[xact->step].pkbuf = pkbuf;

    /* Step */
    xact->step++;

    return OGS_OK;
}

static int ogs_gtp_xact_update_rx(ogs_gtp_xact_t *xact, uint8_t type)
{
    int rv = OGS_OK;
    char buf[OGS_ADDRSTRLEN];
    ogs_gtp_xact_stage_t stage;

    ogs_debug("[%d] %s UPD RX-%d  peer [%s]:%d",
            xact->xid,
            xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
            type,
            OGS_ADDR(&xact->gnode->addr, buf),
            OGS_PORT(&xact->gnode->addr));

    if (xact->gtp_version == 1)
        stage = ogs_gtp1_xact_get_stage(type, xact->xid);
    else
        stage = ogs_gtp_xact_get_stage(type, xact->xid);

    if (xact->org == OGS_GTP_LOCAL_ORIGINATOR) {
        switch (stage) {
        case GTP_XACT_INITIAL_STAGE:
            ogs_expect(0);
            return OGS_ERROR;

        case GTP_XACT_INTERMEDIATE_STAGE:
            if (xact->seq[1].type == type) {
                ogs_pkbuf_t *pkbuf = NULL;

                if (xact->step != 2 && xact->step != 3) {
                    ogs_error("invalid step[%d]", xact->step);
                    ogs_pkbuf_free(pkbuf);
                    return OGS_ERROR;
                }

                pkbuf = xact->seq[2].pkbuf;
                if (pkbuf) {
                    if (xact->tm_holding)
                        ogs_timer_start(xact->tm_holding,
                                ogs_app()->time.message.
                                    gtp.t3_holding_duration);

                    ogs_warn("[%d] %s Request Duplicated. Retransmit!"
                            " for step %d type %d peer [%s]:%d",
                            xact->xid,
                            xact->org == OGS_GTP_LOCAL_ORIGINATOR ?
                                "LOCAL " : "REMOTE",
                            xact->step, type,
                            OGS_ADDR(&xact->gnode->addr,
                                buf),
                            OGS_PORT(&xact->gnode->addr));
                    rv = ogs_gtp_sendto(xact->gnode, pkbuf);
                    ogs_expect(rv == OGS_OK);
                } else {
                    ogs_warn("[%d] %s Request Duplicated. Discard!"
                            " for step %d type %d peer [%s]:%d",
                            xact->xid,
                            xact->org == OGS_GTP_LOCAL_ORIGINATOR ?
                                "LOCAL " : "REMOTE",
                            xact->step, type,
                            OGS_ADDR(&xact->gnode->addr,
                                buf),
                            OGS_PORT(&xact->gnode->addr));
                }

                return OGS_RETRY;
            }

            if (xact->step != 1) {
                ogs_error("invalid step[%d]", xact->step);
                return OGS_ERROR;
            }

            if (xact->tm_holding)
                ogs_timer_start(xact->tm_holding,
                        ogs_app()->time.message.gtp.t3_holding_duration);

            break;

        case GTP_XACT_FINAL_STAGE:
            if (xact->step != 1) {
                ogs_error("invalid step[%d]", xact->step);
                return OGS_ERROR;
            }
            break;

        default:
            ogs_error("invalid stage[%d]", stage);
            return OGS_ERROR;
        }
    } else if (xact->org == OGS_GTP_REMOTE_ORIGINATOR) {
        switch (stage) {
        case GTP_XACT_INITIAL_STAGE:
            if (xact->seq[0].type == type) {
                ogs_pkbuf_t *pkbuf = NULL;

                if (xact->step != 1 && xact->step != 2) {
                    ogs_error("invalid step[%d]", xact->step);
                    return OGS_ERROR;
                }

                pkbuf = xact->seq[1].pkbuf;
                if (pkbuf) {
                    if (xact->tm_holding)
                        ogs_timer_start(xact->tm_holding,
                                ogs_app()->time.message.
                                    gtp.t3_holding_duration);

                    ogs_warn("[%d] %s Request Duplicated. Retransmit!"
                            " for step %d type %d peer [%s]:%d",
                            xact->xid,
                            xact->org == OGS_GTP_LOCAL_ORIGINATOR ?
                                "LOCAL " : "REMOTE",
                            xact->step, type,
                            OGS_ADDR(&xact->gnode->addr,
                                buf),
                            OGS_PORT(&xact->gnode->addr));
                    rv = ogs_gtp_sendto(xact->gnode, pkbuf);
                    ogs_expect(rv == OGS_OK);
                } else {
                    ogs_warn("[%d] %s Request Duplicated. Discard!"
                            " for step %d type %d peer [%s]:%d",
                            xact->xid,
                            xact->org == OGS_GTP_LOCAL_ORIGINATOR ?
                                "LOCAL " : "REMOTE",
                            xact->step, type,
                            OGS_ADDR(&xact->gnode->addr,
                                buf),
                            OGS_PORT(&xact->gnode->addr));
                }

                return OGS_RETRY;
            }

            if (xact->step != 0) {
                ogs_error("invalid step[%d]", xact->step);
                return OGS_ERROR;
            }
            if (xact->tm_holding)
                ogs_timer_start(xact->tm_holding,
                        ogs_app()->time.message.gtp.t3_holding_duration);

            break;

        case GTP_XACT_INTERMEDIATE_STAGE:
            ogs_expect(0);
            return OGS_ERROR;

        case GTP_XACT_FINAL_STAGE:
            if (xact->step != 2) {
                ogs_error("invalid step[%d]", xact->step);
                return OGS_ERROR;
            }

            /* continue */
            break;

        default:
            ogs_error("invalid stage[%d]", stage);
            return OGS_ERROR;
        }
    } else {
        ogs_error("invalid org[%d]", xact->org);
        return OGS_ERROR;
    }

    if (xact->tm_response)
        ogs_timer_stop(xact->tm_response);

    /* Save Message type of this step */
    xact->seq[xact->step].type = type;

    /* Step */
    xact->step++;

    return OGS_OK;
}


int ogs_gtp_xact_commit(ogs_gtp_xact_t *xact)
{
    int rv;
    char buf[OGS_ADDRSTRLEN];

    uint8_t type;
    ogs_pkbuf_t *pkbuf = NULL;
    ogs_gtp_xact_stage_t stage;

    ogs_assert(xact);
    ogs_assert(xact->gnode);

    ogs_debug("[%d] %s Commit  peer [%s]:%d",
            xact->xid,
            xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
            OGS_ADDR(&xact->gnode->addr, buf),
            OGS_PORT(&xact->gnode->addr));

    type = xact->seq[xact->step-1].type;
    if (xact->gtp_version == 1)
        stage = ogs_gtp1_xact_get_stage(type, xact->xid);
    else
        stage = ogs_gtp_xact_get_stage(type, xact->xid);

    if (xact->org == OGS_GTP_LOCAL_ORIGINATOR) {
        switch (stage) {
        case GTP_XACT_INITIAL_STAGE:
            if (xact->step != 1) {
                ogs_error("invalid step[%d]", xact->step);
                ogs_gtp_xact_delete(xact);
                return OGS_ERROR;
            }

            if (xact->tm_response)
                ogs_timer_start(xact->tm_response,
                        ogs_app()->time.message.gtp.t3_response_duration);

            break;

        case GTP_XACT_INTERMEDIATE_STAGE:
            ogs_expect(0);
            ogs_gtp_xact_delete(xact);
            return OGS_ERROR;

        case GTP_XACT_FINAL_STAGE:
            if (xact->step != 2 && xact->step != 3) {
                ogs_error("invalid step[%d]", xact->step);
                ogs_gtp_xact_delete(xact);
                return OGS_ERROR;
            }
            if (xact->step == 2) {
                ogs_gtp_xact_delete(xact);
                return OGS_OK;
            }

            break;

        default:
            ogs_error("invalid stage[%d]", stage);
            ogs_gtp_xact_delete(xact);
            return OGS_ERROR;
        }
    } else if (xact->org == OGS_GTP_REMOTE_ORIGINATOR) {
        switch (stage) {
        case GTP_XACT_INITIAL_STAGE:
            ogs_expect(0);
            ogs_gtp_xact_delete(xact);
            return OGS_ERROR;

        case GTP_XACT_INTERMEDIATE_STAGE:
            if (xact->step != 2) {
                ogs_error("invalid step[%d]", xact->step);
                ogs_gtp_xact_delete(xact);
                return OGS_ERROR;
            }
            if (xact->tm_response)
                ogs_timer_start(xact->tm_response,
                        ogs_app()->time.message.gtp.t3_response_duration);

            break;

        case GTP_XACT_FINAL_STAGE:
            if (xact->step != 2 && xact->step != 3) {
                ogs_error("invalid step[%d]", xact->step);
                ogs_gtp_xact_delete(xact);
                return OGS_ERROR;
            }
            if (xact->step == 3) {
                ogs_gtp_xact_delete(xact);
                return OGS_OK;
            }

            break;

        default:
            ogs_error("invalid stage[%d]", stage);
            ogs_gtp_xact_delete(xact);
            return OGS_ERROR;
        }
    } else {
        ogs_error("invalid org[%d]", xact->org);
        ogs_gtp_xact_delete(xact);
        return OGS_ERROR;
    }

    pkbuf = xact->seq[xact->step-1].pkbuf;
    ogs_assert(pkbuf);

    rv = ogs_gtp_sendto(xact->gnode, pkbuf);
    ogs_expect(rv == OGS_OK);

    return rv;
}

static void response_timeout(void *data)
{
    char buf[OGS_ADDRSTRLEN];
    ogs_gtp_xact_t *xact = data;

    ogs_assert(xact);
    ogs_assert(xact->gnode);

    ogs_debug("[%d] %s Response Timeout "
            "for step %d type %d peer [%s]:%d",
            xact->xid,
            xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
            xact->step, xact->seq[xact->step-1].type,
            OGS_ADDR(&xact->gnode->addr, buf),
            OGS_PORT(&xact->gnode->addr));

    if (--xact->response_rcount > 0) {
        ogs_pkbuf_t *pkbuf = NULL;

        if (xact->tm_response)
            ogs_timer_start(xact->tm_response,
                    ogs_app()->time.message.gtp.t3_response_duration);

        pkbuf = xact->seq[xact->step-1].pkbuf;
        ogs_assert(pkbuf);

        if (ogs_gtp_sendto(xact->gnode, pkbuf) != OGS_OK) {
            ogs_error("ogs_gtp_sendto() failed");
            goto out;
        }
    } else {
        ogs_warn("[%d] %s No Reponse. Give up! "
                "for step %d type %d peer [%s]:%d",
                xact->xid,
                xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
                xact->step, xact->seq[xact->step-1].type,
                OGS_ADDR(&xact->gnode->addr, buf),
                OGS_PORT(&xact->gnode->addr));

        if (xact->cb)
            xact->cb(xact, xact->data);

        ogs_gtp_xact_delete(xact);
    }

    return;

out:
    ogs_gtp_xact_delete(xact);
}

static void holding_timeout(void *data)
{
    char buf[OGS_ADDRSTRLEN];
    ogs_gtp_xact_t *xact = data;

    ogs_assert(xact);
    ogs_assert(xact->gnode);

    ogs_debug("[%d] %s Holding Timeout "
            "for step %d type %d peer [%s]:%d",
            xact->xid,
            xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
            xact->step, xact->seq[xact->step-1].type,
            OGS_ADDR(&xact->gnode->addr, buf),
            OGS_PORT(&xact->gnode->addr));

    if (--xact->holding_rcount > 0) {
        if (xact->tm_holding)
            ogs_timer_start(xact->tm_holding,
                    ogs_app()->time.message.gtp.t3_holding_duration);
    } else {
        ogs_debug("[%d] %s Delete Transaction "
                "for step %d type %d peer [%s]:%d",
                xact->xid,
                xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
                xact->step, xact->seq[xact->step-1].type,
                OGS_ADDR(&xact->gnode->addr, buf),
                OGS_PORT(&xact->gnode->addr));
        ogs_gtp_xact_delete(xact);
    }
}

int ogs_gtp1_xact_receive(
        ogs_gtp_node_t *gnode, ogs_gtp1_header_t *h, ogs_gtp_xact_t **xact)
{
    char buf[OGS_ADDRSTRLEN];
    int rv;
    ogs_gtp_xact_t *new = NULL;
    uint16_t sqn;

    ogs_assert(gnode);
    ogs_assert(h);

    if (!h->s) {
        ogs_error("ogs_gtp_xact_update_rx() failed, pkt has no SQN");
        return OGS_ERROR;
    }
    sqn = h->sqn;

    new = ogs_gtp_xact_find_by_xid(gnode, h->type, 1, OGS_GTP1_SQN_TO_XID(sqn));
    if (!new)
        new = ogs_gtp_xact_remote_create(gnode, 1, sqn);
    ogs_assert(new);

    ogs_debug("[%d] %s Receive peer [%s]:%d",
            new->xid,
            new->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
            OGS_ADDR(&gnode->addr, buf),
            OGS_PORT(&gnode->addr));

    rv = ogs_gtp_xact_update_rx(new, h->type);
    if (rv == OGS_ERROR) {
        ogs_error("ogs_gtp_xact_update_rx() failed");
        ogs_gtp_xact_delete(new);
        return rv;
    } else if (rv == OGS_RETRY) {
        return rv;
    }

    *xact = new;
    return rv;
}

int ogs_gtp_xact_receive(
        ogs_gtp_node_t *gnode, ogs_gtp2_header_t *h, ogs_gtp_xact_t **xact)
{
    char buf[OGS_ADDRSTRLEN];
    int rv;
    ogs_gtp_xact_t *new = NULL;
    uint32_t sqn;

    ogs_assert(gnode);
    ogs_assert(h);

    if (h->teid_presence) sqn = h->sqn;
    else sqn = h->sqn_only;

    new = ogs_gtp_xact_find_by_xid(gnode, h->type, 2, OGS_GTP2_SQN_TO_XID(sqn));
    if (!new)
        new = ogs_gtp_xact_remote_create(gnode, 2, sqn);
    ogs_assert(new);

    ogs_debug("[%d] %s Receive peer [%s]:%d",
            new->xid,
            new->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
            OGS_ADDR(&gnode->addr, buf),
            OGS_PORT(&gnode->addr));

    rv = ogs_gtp_xact_update_rx(new, h->type);
    if (rv == OGS_ERROR) {
        ogs_error("ogs_gtp_xact_update_rx() failed");
        ogs_gtp_xact_delete(new);
        return rv;
    } else if (rv == OGS_RETRY) {
        return rv;
    }

    *xact = new;
    return rv;
}

static ogs_gtp_xact_stage_t ogs_gtp1_xact_get_stage(uint8_t type, uint32_t xid)
{
    ogs_gtp_xact_stage_t stage = GTP_XACT_UNKNOWN_STAGE;

    switch (type) {
    case OGS_GTP1_ECHO_REQUEST_TYPE:
    case OGS_GTP1_NODE_ALIVE_REQUEST_TYPE:
    case OGS_GTP1_REDIRECTION_REQUEST_TYPE:
    case OGS_GTP1_CREATE_PDP_CONTEXT_REQUEST_TYPE:
    case OGS_GTP1_UPDATE_PDP_CONTEXT_REQUEST_TYPE:
    case OGS_GTP1_DELETE_PDP_CONTEXT_REQUEST_TYPE:
    case OGS_GTP1_INITIATE_PDP_CONTEXT_ACTIVATION_REQUEST_TYPE:
    case OGS_GTP1_PDU_NOTIFICATION_REQUEST_TYPE:
    case OGS_GTP1_PDU_NOTIFICATION_REJECT_REQUEST_TYPE:
    case OGS_GTP1_SEND_ROUTEING_INFORMATION_FOR_GPRS_REQUEST_TYPE:
    case OGS_GTP1_FAILURE_REPORT_REQUEST_TYPE:
    case OGS_GTP1_NOTE_MS_GPRS_PRESENT_REQUEST_TYPE:
    case OGS_GTP1_IDENTIFICATION_REQUEST_TYPE:
    case OGS_GTP1_SGSN_CONTEXT_REQUEST_TYPE:
    case OGS_GTP1_FORWARD_RELOCATION_REQUEST_TYPE:
    case OGS_GTP1_RELOCATION_CANCEL_REQUEST_TYPE:
    case OGS_GTP1_UE_REGISTRATION_QUERY_REQUEST_TYPE:
        stage = GTP_XACT_INITIAL_STAGE;
        break;
    case OGS_GTP1_ECHO_RESPONSE_TYPE:
    case OGS_GTP1_NODE_ALIVE_RESPONSE_TYPE:
    case OGS_GTP1_REDIRECTION_RESPONSE_TYPE:
    case OGS_GTP1_CREATE_PDP_CONTEXT_RESPONSE_TYPE:
    case OGS_GTP1_UPDATE_PDP_CONTEXT_RESPONSE_TYPE:
    case OGS_GTP1_DELETE_PDP_CONTEXT_RESPONSE_TYPE:
    case OGS_GTP1_INITIATE_PDP_CONTEXT_ACTIVATION_RESPONSE_TYPE:
    case OGS_GTP1_PDU_NOTIFICATION_RESPONSE_TYPE:
    case OGS_GTP1_PDU_NOTIFICATION_REJECT_RESPONSE_TYPE:
    case OGS_GTP1_SEND_ROUTEING_INFORMATION_FOR_GPRS_RESPONSE_TYPE:
    case OGS_GTP1_FAILURE_REPORT_RESPONSE_TYPE:
    case OGS_GTP1_NOTE_MS_GPRS_PRESENT_RESPONSE_TYPE:
    case OGS_GTP1_IDENTIFICATION_RESPONSE_TYPE:
    case OGS_GTP1_SGSN_CONTEXT_RESPONSE_TYPE:
    case OGS_GTP1_FORWARD_RELOCATION_RESPONSE_TYPE:
    case OGS_GTP1_RELOCATION_CANCEL_RESPONSE_TYPE:
    case OGS_GTP1_UE_REGISTRATION_QUERY_RESPONSE_TYPE:
        stage = GTP_XACT_FINAL_STAGE;
        break;

    default:
        ogs_error("Not implemented GTPv1 Message Type(%d)", type);
        break;
    }

    return stage;
}

static ogs_gtp_xact_stage_t ogs_gtp_xact_get_stage(uint8_t type, uint32_t xid)
{
    ogs_gtp_xact_stage_t stage = GTP_XACT_UNKNOWN_STAGE;

    switch (type) {
    case OGS_GTP2_CREATE_SESSION_REQUEST_TYPE:
    case OGS_GTP2_MODIFY_BEARER_REQUEST_TYPE:
    case OGS_GTP2_DELETE_SESSION_REQUEST_TYPE:
    case OGS_GTP2_MODIFY_BEARER_COMMAND_TYPE:
    case OGS_GTP2_DELETE_BEARER_COMMAND_TYPE:
    case OGS_GTP2_BEARER_RESOURCE_COMMAND_TYPE:
    case OGS_GTP2_RELEASE_ACCESS_BEARERS_REQUEST_TYPE:
    case OGS_GTP2_CREATE_INDIRECT_DATA_FORWARDING_TUNNEL_REQUEST_TYPE:
    case OGS_GTP2_DELETE_INDIRECT_DATA_FORWARDING_TUNNEL_REQUEST_TYPE:
    case OGS_GTP2_DOWNLINK_DATA_NOTIFICATION_TYPE:
    case OGS_GTP2_ECHO_REQUEST_TYPE:
        stage = GTP_XACT_INITIAL_STAGE;
        break;
    case OGS_GTP2_CREATE_BEARER_REQUEST_TYPE:
    case OGS_GTP2_UPDATE_BEARER_REQUEST_TYPE:
    case OGS_GTP2_DELETE_BEARER_REQUEST_TYPE:
        if (xid & OGS_GTP_CMD_XACT_ID)
            stage = GTP_XACT_INTERMEDIATE_STAGE;
        else
            stage = GTP_XACT_INITIAL_STAGE;
        break;
    case OGS_GTP2_CREATE_SESSION_RESPONSE_TYPE:
    case OGS_GTP2_MODIFY_BEARER_RESPONSE_TYPE:
    case OGS_GTP2_DELETE_SESSION_RESPONSE_TYPE:
    case OGS_GTP2_MODIFY_BEARER_FAILURE_INDICATION_TYPE:
    case OGS_GTP2_DELETE_BEARER_FAILURE_INDICATION_TYPE:
    case OGS_GTP2_BEARER_RESOURCE_FAILURE_INDICATION_TYPE:
    case OGS_GTP2_CREATE_BEARER_RESPONSE_TYPE:
    case OGS_GTP2_UPDATE_BEARER_RESPONSE_TYPE:
    case OGS_GTP2_DELETE_BEARER_RESPONSE_TYPE:
    case OGS_GTP2_RELEASE_ACCESS_BEARERS_RESPONSE_TYPE:
    case OGS_GTP2_CREATE_INDIRECT_DATA_FORWARDING_TUNNEL_RESPONSE_TYPE:
    case OGS_GTP2_DELETE_INDIRECT_DATA_FORWARDING_TUNNEL_RESPONSE_TYPE:
    case OGS_GTP2_DOWNLINK_DATA_NOTIFICATION_ACKNOWLEDGE_TYPE:
    case OGS_GTP2_ECHO_RESPONSE_TYPE:
        stage = GTP_XACT_FINAL_STAGE;
        break;

    default:
        ogs_error("Not implemented GTPv2 Message Type(%d)", type);
        break;
    }

    return stage;
}

static ogs_gtp_xact_t *ogs_gtp_xact_find_by_xid(
        ogs_gtp_node_t *gnode, uint8_t type, uint8_t gtp_version, uint32_t xid)
{
    char buf[OGS_ADDRSTRLEN];

    ogs_list_t *list = NULL;
    ogs_gtp_xact_t *xact = NULL;
    ogs_gtp_xact_stage_t stage;

    ogs_assert(gnode);

    if (gtp_version == 1)
        stage = ogs_gtp1_xact_get_stage(type, xid);
    else
        stage = ogs_gtp_xact_get_stage(type, xid);

    switch (stage) {
    case GTP_XACT_INITIAL_STAGE:
        list = &gnode->remote_list;
        break;
    case GTP_XACT_INTERMEDIATE_STAGE:
        list = &gnode->local_list;
        break;
    case GTP_XACT_FINAL_STAGE:
        switch (gtp_version) {
        case 1:
            list = &gnode->local_list; // FIXME: is this correct?
            break;
        case 2:
        default:
            if (xid & OGS_GTP_CMD_XACT_ID) {
                if (type == OGS_GTP2_MODIFY_BEARER_FAILURE_INDICATION_TYPE ||
                    type == OGS_GTP2_DELETE_BEARER_FAILURE_INDICATION_TYPE ||
                    type == OGS_GTP2_BEARER_RESOURCE_FAILURE_INDICATION_TYPE) {
                    list = &gnode->local_list;
                } else {
                    list = &gnode->remote_list;
                }
            } else {
                list = &gnode->local_list;
            }
            break;
        }
        break;
    default:
        ogs_assert_if_reached();
        break;
    }

    ogs_assert(list);
    ogs_list_for_each(list, xact) {
        if (xact->gtp_version == gtp_version && xact->xid == xid) {
            ogs_debug("[%d] %s Find GTPv%u peer [%s]:%d",
                    xact->xid,
                    xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
                    xact->gtp_version,
                    OGS_ADDR(&gnode->addr, buf),
                    OGS_PORT(&gnode->addr));
            break;
        }
    }

    return xact;
}

void ogs_gtp_xact_associate(ogs_gtp_xact_t *xact1, ogs_gtp_xact_t *xact2)
{
    ogs_assert(xact1);
    ogs_assert(xact2);

    ogs_assert(xact1->assoc_xact == NULL);
    ogs_assert(xact2->assoc_xact == NULL);

    xact1->assoc_xact = xact2;
    xact2->assoc_xact = xact1;
}

void ogs_gtp_xact_deassociate(ogs_gtp_xact_t *xact1, ogs_gtp_xact_t *xact2)
{
    ogs_assert(xact1);
    ogs_assert(xact2);

    ogs_assert(xact1->assoc_xact != NULL);
    ogs_assert(xact2->assoc_xact != NULL);

    xact1->assoc_xact = NULL;
    xact2->assoc_xact = NULL;
}

static int ogs_gtp_xact_delete(ogs_gtp_xact_t *xact)
{
    char buf[OGS_ADDRSTRLEN];

    ogs_assert(xact);
    ogs_assert(xact->gnode);

    ogs_debug("[%d] %s Delete  peer [%s]:%d",
            xact->xid,
            xact->org == OGS_GTP_LOCAL_ORIGINATOR ? "LOCAL " : "REMOTE",
            OGS_ADDR(&xact->gnode->addr, buf),
            OGS_PORT(&xact->gnode->addr));

    if (xact->seq[0].pkbuf)
        ogs_pkbuf_free(xact->seq[0].pkbuf);
    if (xact->seq[1].pkbuf)
        ogs_pkbuf_free(xact->seq[1].pkbuf);
    if (xact->seq[2].pkbuf)
        ogs_pkbuf_free(xact->seq[2].pkbuf);

    if (xact->tm_response)
        ogs_timer_delete(xact->tm_response);
    if (xact->tm_holding)
        ogs_timer_delete(xact->tm_holding);

    if (xact->assoc_xact)
        ogs_gtp_xact_deassociate(xact, xact->assoc_xact);

    ogs_list_remove(xact->org == OGS_GTP_LOCAL_ORIGINATOR ?
            &xact->gnode->local_list : &xact->gnode->remote_list, xact);
    ogs_pool_free(&pool, xact);

    return OGS_OK;
}
