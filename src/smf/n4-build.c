/*
 * Copyright (C) 2019 by Sukchan Lee <acetcom@gmail.com>
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

#include "context.h"
#include "n4-build.h"

ogs_pkbuf_t *smf_n4_build_session_establishment_request(
        uint8_t type, smf_sess_t *sess)
{
    ogs_pfcp_message_t pfcp_message;
    ogs_pfcp_session_establishment_request_t *req = NULL;
    ogs_pkbuf_t *pkbuf = NULL;

    ogs_pfcp_pdr_t *pdr = NULL;
    ogs_pfcp_far_t *far = NULL;
    ogs_pfcp_urr_t *urr = NULL;
    ogs_pfcp_qer_t *qer = NULL;
    int i, rv;

    ogs_pfcp_node_id_t node_id;
    ogs_pfcp_f_seid_t f_seid;
    int len;

    ogs_debug("Session Establishment Request");
    ogs_assert(sess);

    req = &pfcp_message.pfcp_session_establishment_request;
    memset(&pfcp_message, 0, sizeof(ogs_pfcp_message_t));

    /* Node ID */
    rv = ogs_pfcp_sockaddr_to_node_id(
            ogs_pfcp_self()->pfcp_addr, ogs_pfcp_self()->pfcp_addr6,
            ogs_app()->parameter.prefer_ipv4,
            &node_id, &len);
    ogs_expect_or_return_val(rv == OGS_OK, NULL);
    req->node_id.presence = 1;
    req->node_id.data = &node_id;
    req->node_id.len = len;

    /* F-SEID */
    rv = ogs_pfcp_sockaddr_to_f_seid(
            ogs_pfcp_self()->pfcp_addr, ogs_pfcp_self()->pfcp_addr6,
            &f_seid, &len);
    ogs_expect_or_return_val(rv == OGS_OK, NULL);
    f_seid.seid = htobe64(sess->smf_n4_seid);
    req->cp_f_seid.presence = 1;
    req->cp_f_seid.data = &f_seid;
    req->cp_f_seid.len = len;

    ogs_pfcp_pdrbuf_init();

    /* Create PDR */
    i = 0;
    ogs_list_for_each(&sess->pfcp.pdr_list, pdr) {
        ogs_pfcp_build_create_pdr(&req->create_pdr[i], i, pdr);
        i++;
    }

    /* Create FAR */
    i = 0;
    ogs_list_for_each(&sess->pfcp.far_list, far) {
        ogs_pfcp_build_create_far(&req->create_far[i], i, far);
        i++;
    }

    /* Create URR */
    i = 0;
    ogs_list_for_each(&sess->pfcp.urr_list, urr) {
        ogs_pfcp_build_create_urr(&req->create_urr[i], i, urr);
        i++;
    }

    /* Create QER */
    i = 0;
    ogs_list_for_each(&sess->pfcp.qer_list, qer) {
        ogs_pfcp_build_create_qer(&req->create_qer[i], i, qer);
        i++;
    }

    /* Create BAR */
    if (sess->pfcp.bar) {
        ogs_pfcp_build_create_bar(&req->create_bar, sess->pfcp.bar);
    }

    /* PDN Type */
    req->pdn_type.presence = 1;
    req->pdn_type.u8 = sess->session.paa.session_type;

    pfcp_message.h.type = type;
    pkbuf = ogs_pfcp_build_msg(&pfcp_message);

    ogs_pfcp_pdrbuf_clear();

    return pkbuf;
}

ogs_pkbuf_t *smf_n4_build_session_modification_request(
        uint8_t type, smf_sess_t *sess, uint64_t modify_flags)
{
    ogs_pfcp_pdr_t *pdr = NULL;

    ogs_pfcp_message_t pfcp_message;
    ogs_pfcp_session_modification_request_t *req = NULL;
    ogs_pkbuf_t *pkbuf = NULL;

    int num_of_remove_pdr = 0;
    int num_of_remove_far = 0;
    int num_of_create_pdr = 0;
    int num_of_create_far = 0;
    int num_of_update_far = 0;

    ogs_debug("Session Modification Request");
    ogs_assert(sess);
    ogs_assert(modify_flags);

    req = &pfcp_message.pfcp_session_modification_request;
    memset(&pfcp_message, 0, sizeof(ogs_pfcp_message_t));

    ogs_list_for_each(&sess->pfcp.pdr_list, pdr) {
        ogs_pfcp_far_t *far = pdr->far;
        ogs_assert(far);

        if (((modify_flags &
              (OGS_PFCP_MODIFY_DL_ONLY|
               OGS_PFCP_MODIFY_UL_ONLY|
               OGS_PFCP_MODIFY_INDIRECT)) == 0) ||

            ((modify_flags & OGS_PFCP_MODIFY_DL_ONLY) &&
             (pdr->src_if == OGS_PFCP_INTERFACE_CORE) &&
             (far->dst_if == OGS_PFCP_INTERFACE_ACCESS)) ||

            ((modify_flags & OGS_PFCP_MODIFY_UL_ONLY) &&
             (pdr->src_if == OGS_PFCP_INTERFACE_ACCESS) &&
             (far->dst_if == OGS_PFCP_INTERFACE_CORE)) ||

            ((modify_flags & OGS_PFCP_MODIFY_INDIRECT) &&
             (pdr->src_if == OGS_PFCP_INTERFACE_ACCESS) &&
             (far->dst_if == OGS_PFCP_INTERFACE_ACCESS))) {

            if (modify_flags & OGS_PFCP_MODIFY_REMOVE) {
                ogs_pfcp_tlv_remove_pdr_t *remove_pdr = NULL;
                ogs_pfcp_tlv_remove_far_t *remove_far = NULL;

                remove_pdr = &req->remove_pdr[num_of_remove_pdr];
                remove_pdr->presence = 1;
                remove_pdr->pdr_id.presence = 1;
                remove_pdr->pdr_id.u16 = pdr->id;
                num_of_remove_pdr++;

                remove_far = &req->remove_far[num_of_remove_far];
                remove_far->presence = 1;
                remove_far->far_id.presence = 1;
                remove_far->far_id.u32 = far->id;
                num_of_remove_far++;
            } else if (modify_flags & OGS_PFCP_MODIFY_CREATE) {
                ogs_pfcp_build_create_pdr(
                        &req->create_pdr[num_of_create_pdr],
                        num_of_create_pdr, pdr);

                num_of_create_pdr++;

                ogs_pfcp_build_create_far(
                        &req->create_far[num_of_create_far],
                        num_of_create_far, far);

                num_of_create_far++;
            } else if (modify_flags & OGS_PFCP_MODIFY_ACTIVATE) {
                if (far->apply_action & OGS_PFCP_APPLY_ACTION_FORW) {

                    if (modify_flags & OGS_PFCP_MODIFY_END_MARKER) {
                        far->smreq_flags.send_end_marker_packets = 1;
                    }

                    ogs_pfcp_build_update_far_activate(
                            &req->update_far[num_of_update_far],
                            num_of_update_far, far);

                    /* Clear all FAR flags */
                    far->smreq_flags.value = 0;

                    num_of_update_far++;
                }
            } else if (modify_flags & OGS_PFCP_MODIFY_DEACTIVATE) {
                ogs_pfcp_build_update_far_deactivate(
                        &req->update_far[num_of_update_far],
                        num_of_update_far, far);
                num_of_update_far++;
            } else {
                ogs_fatal("Invalid modify_flags = %lld",
                        (long long)modify_flags);
                ogs_assert_if_reached();
            }
        }

    }

    pfcp_message.h.type = type;
    pkbuf = ogs_pfcp_build_msg(&pfcp_message);

    return pkbuf;
}

ogs_pkbuf_t *smf_n4_build_qos_flow_modification_request(
        uint8_t type, smf_bearer_t *qos_flow, uint64_t modify_flags)
{
    ogs_pfcp_message_t pfcp_message;
    ogs_pfcp_session_modification_request_t *req = NULL;
    ogs_pkbuf_t *pkbuf = NULL;
    int i;

    smf_sess_t *sess = NULL;

    ogs_debug("QoS Flow Modification Request");
    ogs_assert(qos_flow);
    sess = qos_flow->sess;
    ogs_assert(sess);
    ogs_assert(modify_flags);

    req = &pfcp_message.pfcp_session_modification_request;
    memset(&pfcp_message, 0, sizeof(ogs_pfcp_message_t));

    if (modify_flags & OGS_PFCP_MODIFY_REMOVE) {
        /* Remove PDR */
        i = 0;
        if (qos_flow->dl_pdr) {
            ogs_pfcp_tlv_remove_pdr_t *message = &req->remove_pdr[i];

            message->presence = 1;
            message->pdr_id.presence = 1;
            message->pdr_id.u16 = qos_flow->dl_pdr->id;
            i++;
        }
        if (qos_flow->ul_pdr) {
            ogs_pfcp_tlv_remove_pdr_t *message = &req->remove_pdr[i];

            message->presence = 1;
            message->pdr_id.presence = 1;
            message->pdr_id.u16 = qos_flow->ul_pdr->id;
            i++;
        }

        /* Remove FAR */
        i = 0;
        if (qos_flow->dl_far) {
            ogs_pfcp_tlv_remove_far_t *message = &req->remove_far[i];

            message->presence = 1;
            message->far_id.presence = 1;
            message->far_id.u32 = qos_flow->dl_far->id;
            i++;
        }
        if (qos_flow->ul_far) {
            ogs_pfcp_tlv_remove_far_t *message = &req->remove_far[i];

            message->presence = 1;
            message->far_id.presence = 1;
            message->far_id.u32 = qos_flow->ul_far->id;
            i++;
        }

        /* Remove QER */
        i = 0;
        if (qos_flow->qer) {
            ogs_pfcp_tlv_remove_qer_t *message = &req->remove_qer[i];

            message->presence = 1;
            message->qer_id.presence = 1;
            message->qer_id.u32 = qos_flow->qer->id;
            i++;
        }

    } else {
        if (modify_flags & OGS_PFCP_MODIFY_CREATE) {
            ogs_pfcp_pdrbuf_init();

            /* Create PDR */
            i = 0;
            if (qos_flow->dl_pdr) {
                ogs_pfcp_build_create_pdr(
                        &req->create_pdr[i], i, qos_flow->dl_pdr);
                i++;
            }
            if (qos_flow->ul_pdr) {
                ogs_pfcp_build_create_pdr(
                        &req->create_pdr[i], i, qos_flow->ul_pdr);
                i++;
            }

            /* Create FAR */
            i = 0;
            if (qos_flow->dl_far) {
                ogs_pfcp_build_create_far(
                        &req->create_far[i], i, qos_flow->dl_far);
                i++;
            }
            if (qos_flow->ul_far) {
                ogs_pfcp_build_create_far(
                        &req->create_far[i], i, qos_flow->ul_far);
                i++;
            }

            /* Create QER */
            i = 0;
            if (qos_flow->qer) {
                ogs_pfcp_build_create_qer(
                        &req->create_qer[i], i, qos_flow->qer);
                i++;
            }
        }
        if (modify_flags & OGS_PFCP_MODIFY_TFT_UPDATE) {
            ogs_pfcp_pdrbuf_init();

            /* Update PDR */
            i = 0;
            if (qos_flow->dl_pdr) {
                ogs_pfcp_build_update_pdr(
                        &req->update_pdr[i], i, qos_flow->dl_pdr);
                i++;
            }
            if (qos_flow->ul_pdr) {
                ogs_pfcp_build_update_pdr(
                        &req->update_pdr[i], i, qos_flow->ul_pdr);
                i++;
            }
        }
        if (modify_flags & OGS_PFCP_MODIFY_ACTIVATE
            || modify_flags & OGS_PFCP_MODIFY_UL_ONLY) {
            /* Update FAR - Only DL */
            i = 0;
            if (qos_flow->dl_far) {
                if (qos_flow->dl_far->apply_action &
                        OGS_PFCP_APPLY_ACTION_FORW) {

                    if (modify_flags & OGS_PFCP_MODIFY_END_MARKER) {
                        qos_flow->dl_far->
                            smreq_flags.send_end_marker_packets = 1;
                    }

                    ogs_pfcp_build_update_far_activate(
                            &req->update_far[i], i, qos_flow->dl_far);
                    i++;

                    /* Clear all FAR flags */
                    qos_flow->dl_far->smreq_flags.value = 0;
                }
            }
        } else if (modify_flags & OGS_PFCP_MODIFY_DEACTIVATE) {
            i = 0;
            if (qos_flow->dl_far) {
                ogs_pfcp_build_update_far_deactivate(
                        &req->update_far[i], i, qos_flow->dl_far);
                i++;
            }
        }
        if (modify_flags & OGS_PFCP_MODIFY_QOS_UPDATE) {
            /* Update QER */
            i = 0;
            if (qos_flow->qer) {
                ogs_pfcp_build_update_qer(
                        &req->update_qer[i], i, qos_flow->qer);
                i++;
            }
        }
    }

    pfcp_message.h.type = type;
    pkbuf = ogs_pfcp_build_msg(&pfcp_message);

    if (modify_flags & (OGS_PFCP_MODIFY_CREATE|OGS_PFCP_MODIFY_TFT_UPDATE)) {
        ogs_pfcp_pdrbuf_clear();
    }

    return pkbuf;
}

ogs_pkbuf_t *smf_n4_build_session_deletion_request(
        uint8_t type, smf_sess_t *sess)
{
    ogs_pfcp_message_t pfcp_message;

    ogs_debug("Session Deletion Request");
    ogs_assert(sess);

    pfcp_message.h.type = type;
    return ogs_pfcp_build_msg(&pfcp_message);
}
