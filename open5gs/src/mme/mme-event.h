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

#ifndef MME_EVENT_H
#define MME_EVENT_H

#include "ogs-core.h"

#ifdef __cplusplus
extern "C" {
#endif

/* forward declaration */
typedef enum {
    MME_EVT_BASE = OGS_FSM_USER_SIG,

    MME_EVT_S1AP_MESSAGE,
    MME_EVT_S1AP_TIMER,
    MME_EVT_S1AP_LO_ACCEPT,
    MME_EVT_S1AP_LO_SCTP_COMM_UP,
    MME_EVT_S1AP_LO_CONNREFUSED,

    MME_EVT_EMM_MESSAGE,
    MME_EVT_EMM_TIMER,
    MME_EVT_ESM_MESSAGE,
    MME_EVT_ESM_TIMER,
    MME_EVT_S11_MESSAGE,
    MME_EVT_S11_TIMER,
    MME_EVT_S6A_MESSAGE,
    MME_EVT_S6A_TIMER,

    MME_EVT_SGSAP_MESSAGE,
    MME_EVT_SGSAP_TIMER,
    MME_EVT_SGSAP_LO_SCTP_COMM_UP,
    MME_EVT_SGSAP_LO_CONNREFUSED,

    MME_EVT_TOP,

} mme_event_e;

typedef long S1AP_ProcedureCode_t;
typedef struct S1AP_S1AP_PDU ogs_s1ap_message_t;
typedef struct ogs_nas_eps_message_s ogs_nas_eps_message_t;
typedef struct mme_vlr_s mme_vlr_t;
typedef struct mme_enb_s mme_enb_t;
typedef struct enb_ue_s enb_ue_t;
typedef struct mme_ue_s mme_ue_t;
typedef struct mme_sess_s mme_sess_t;
typedef struct mme_bearer_s mme_bearer_t;
typedef struct ogs_gtp_node_s ogs_gtp_node_t;

typedef struct mme_event_s {
    int id;
    ogs_pkbuf_t *pkbuf;
    int timer_id;

    ogs_sock_t *sock;
    ogs_sockaddr_t *addr;

    uint16_t max_num_of_istreams;
    uint16_t max_num_of_ostreams;

    S1AP_ProcedureCode_t s1ap_code;
    ogs_s1ap_message_t *s1ap_message;

    ogs_gtp_node_t *gnode;

    uint8_t nas_type;
    ogs_nas_eps_message_t *nas_message;

    mme_vlr_t *vlr;
    mme_enb_t *enb;
    enb_ue_t *enb_ue;
    mme_ue_t *mme_ue;
    mme_sess_t *sess;
    mme_bearer_t *bearer;

    ogs_timer_t *timer;
} mme_event_t;

void mme_event_term(void);

mme_event_t *mme_event_new(mme_event_e id);
void mme_event_free(mme_event_t *e);

void mme_event_timeout(void *data);

const char *mme_event_get_name(mme_event_t *e);

void mme_sctp_event_push(mme_event_e id,
        void *sock, ogs_sockaddr_t *addr, ogs_pkbuf_t *pkbuf,
        uint16_t max_num_of_istreams, uint16_t max_num_of_ostreams);

#ifdef __cplusplus
}
#endif

#endif /* MME_EVENT_H */
