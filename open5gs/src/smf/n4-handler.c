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
#include "timer.h"
#include "pfcp-path.h"
#include "gtp-path.h"
#include "n4-handler.h"
#include "binding.h"
#include "sbi-path.h"
#include "ngap-path.h"

static uint8_t gtp_cause_from_pfcp(uint8_t pfcp_cause)
{
    switch (pfcp_cause) {
    case OGS_PFCP_CAUSE_REQUEST_ACCEPTED:
        return OGS_GTP_CAUSE_REQUEST_ACCEPTED;
    case OGS_PFCP_CAUSE_REQUEST_REJECTED:
        return OGS_GTP_CAUSE_REQUEST_REJECTED_REASON_NOT_SPECIFIED;
    case OGS_PFCP_CAUSE_SESSION_CONTEXT_NOT_FOUND:
        return OGS_GTP_CAUSE_CONTEXT_NOT_FOUND;
    case OGS_PFCP_CAUSE_MANDATORY_IE_MISSING:
        return OGS_GTP_CAUSE_MANDATORY_IE_MISSING;
    case OGS_PFCP_CAUSE_CONDITIONAL_IE_MISSING:
        return OGS_GTP_CAUSE_CONDITIONAL_IE_MISSING;
    case OGS_PFCP_CAUSE_INVALID_LENGTH:
        return OGS_GTP_CAUSE_INVALID_LENGTH;
    case OGS_PFCP_CAUSE_MANDATORY_IE_INCORRECT:
        return OGS_GTP_CAUSE_MANDATORY_IE_INCORRECT;
    case OGS_PFCP_CAUSE_INVALID_FORWARDING_POLICY:
    case OGS_PFCP_CAUSE_INVALID_F_TEID_ALLOCATION_OPTION:
        return OGS_GTP_CAUSE_INVALID_MESSAGE_FORMAT;
    case OGS_PFCP_CAUSE_NO_ESTABLISHED_PFCP_ASSOCIATION:
        return OGS_GTP_CAUSE_REMOTE_PEER_NOT_RESPONDING;
    case OGS_PFCP_CAUSE_RULE_CREATION_MODIFICATION_FAILURE:
        return OGS_GTP_CAUSE_SEMANTIC_ERROR_IN_THE_TFT_OPERATION;
    case OGS_PFCP_CAUSE_PFCP_ENTITY_IN_CONGESTION:
        return OGS_GTP_CAUSE_GTP_C_ENTITY_CONGESTION;
    case OGS_PFCP_CAUSE_NO_RESOURCES_AVAILABLE:
        return OGS_GTP_CAUSE_NO_RESOURCES_AVAILABLE;
    case OGS_PFCP_CAUSE_SERVICE_NOT_SUPPORTED:
        return OGS_GTP_CAUSE_SERVICE_NOT_SUPPORTED;
    case OGS_PFCP_CAUSE_SYSTEM_FAILURE:
        return OGS_GTP_CAUSE_SYSTEM_FAILURE;
    case OGS_PFCP_CAUSE_ALL_DYNAMIC_ADDRESS_ARE_OCCUPIED:
        return OGS_GTP_CAUSE_ALL_DYNAMIC_ADDRESSES_ARE_OCCUPIED;
    default:
        return OGS_GTP_CAUSE_SYSTEM_FAILURE;
    }

    return OGS_GTP_CAUSE_SYSTEM_FAILURE;
}

static int sbi_status_from_pfcp(uint8_t pfcp_cause)
{
    switch (pfcp_cause) {
    case OGS_PFCP_CAUSE_REQUEST_ACCEPTED:
        return OGS_SBI_HTTP_STATUS_OK;
    case OGS_PFCP_CAUSE_REQUEST_REJECTED:
        return OGS_SBI_HTTP_STATUS_FORBIDDEN;
    case OGS_PFCP_CAUSE_SESSION_CONTEXT_NOT_FOUND:
        return OGS_SBI_HTTP_STATUS_NOT_FOUND;
    case OGS_PFCP_CAUSE_MANDATORY_IE_MISSING:
    case OGS_PFCP_CAUSE_CONDITIONAL_IE_MISSING:
    case OGS_PFCP_CAUSE_INVALID_LENGTH:
    case OGS_PFCP_CAUSE_MANDATORY_IE_INCORRECT:
    case OGS_PFCP_CAUSE_INVALID_FORWARDING_POLICY:
    case OGS_PFCP_CAUSE_INVALID_F_TEID_ALLOCATION_OPTION:
    case OGS_PFCP_CAUSE_RULE_CREATION_MODIFICATION_FAILURE:
    case OGS_PFCP_CAUSE_PFCP_ENTITY_IN_CONGESTION:
    case OGS_PFCP_CAUSE_NO_RESOURCES_AVAILABLE:
        return OGS_SBI_HTTP_STATUS_BAD_REQUEST;
    case OGS_PFCP_CAUSE_NO_ESTABLISHED_PFCP_ASSOCIATION:
        return OGS_SBI_HTTP_STATUS_GATEWAY_TIMEOUT;
    case OGS_PFCP_CAUSE_SERVICE_NOT_SUPPORTED:
        return OGS_SBI_HTTP_STATUS_SERVICE_UNAVAILABLE;
    case OGS_PFCP_CAUSE_SYSTEM_FAILURE:
        return OGS_SBI_HTTP_STATUS_INTERNAL_SERVER_ERROR;
    default:
        return OGS_SBI_HTTP_STATUS_INTERNAL_SERVER_ERROR;
    }

    return OGS_SBI_HTTP_STATUS_INTERNAL_SERVER_ERROR;
}

void smf_5gc_n4_handle_session_establishment_response(
        smf_sess_t *sess, ogs_pfcp_xact_t *xact,
        ogs_pfcp_session_establishment_response_t *rsp)
{
    int i;

    smf_n1_n2_message_transfer_param_t param;
    ogs_sbi_stream_t *stream = NULL;

    uint8_t pfcp_cause_value = OGS_PFCP_CAUSE_REQUEST_ACCEPTED;
    uint8_t offending_ie_value = 0;

    ogs_pfcp_f_seid_t *up_f_seid = NULL;

    ogs_assert(xact);
    ogs_assert(rsp);

    stream = xact->assoc_stream;
    ogs_assert(stream);

    ogs_pfcp_xact_commit(xact);

    if (!sess) {
        ogs_warn("No Context");
        return;
    }

    if (rsp->up_f_seid.presence == 0) {
        ogs_error("No UP F-SEID");
        return;
    }

    if (rsp->created_pdr[0].presence == 0) {
        ogs_error("No Created PDR");
        return;
    }

    if (rsp->cause.presence) {
        if (rsp->cause.u8 != OGS_PFCP_CAUSE_REQUEST_ACCEPTED) {
            ogs_error("PFCP Cause [%d] : Not Accepted", rsp->cause.u8);
            return;
        }
    } else {
        ogs_error("No Cause");
        return;
    }

    ogs_assert(sess);

    pfcp_cause_value = OGS_PFCP_CAUSE_REQUEST_ACCEPTED;
    for (i = 0; i < OGS_MAX_NUM_OF_PDR; i++) {
        ogs_pfcp_pdr_t *pdr = NULL;
        ogs_pfcp_far_t *far = NULL;

        pdr = ogs_pfcp_handle_created_pdr(
                &sess->pfcp, &rsp->created_pdr[i],
                &pfcp_cause_value, &offending_ie_value);

        if (!pdr)
            break;

        far = pdr->far;
        ogs_assert(far);

        if (pdr->src_if == OGS_PFCP_INTERFACE_ACCESS) {
            if (far->dst_if == OGS_PFCP_INTERFACE_CP_FUNCTION)
                ogs_pfcp_far_teid_hash_set(far);

            ogs_assert(sess->pfcp_node);
            if (sess->pfcp_node->up_function_features.ftup &&
                pdr->f_teid_len) {
                if (sess->upf_n3_addr)
                    ogs_freeaddrinfo(sess->upf_n3_addr);
                if (sess->upf_n3_addr6)
                    ogs_freeaddrinfo(sess->upf_n3_addr6);

                ogs_assert(OGS_OK ==
                    ogs_pfcp_f_teid_to_sockaddr(
                        &pdr->f_teid, pdr->f_teid_len,
                        &sess->upf_n3_addr, &sess->upf_n3_addr6));
                sess->upf_n3_teid = pdr->f_teid.teid;
            }
        } else if (pdr->src_if == OGS_PFCP_INTERFACE_CP_FUNCTION) {
            ogs_assert(OGS_ERROR != ogs_pfcp_setup_pdr_gtpu_node(pdr));
        }
    }

    if (pfcp_cause_value != OGS_PFCP_CAUSE_REQUEST_ACCEPTED) {
        ogs_error("PFCP Cause [%d] : Not Accepted", pfcp_cause_value);
        return;
    }

    if (sess->upf_n3_addr == NULL && sess->upf_n3_addr6 == NULL) {
        ogs_error("No UP F-TEID");
        return;
    }

    /* UP F-SEID */
    up_f_seid = rsp->up_f_seid.data;
    ogs_assert(up_f_seid);
    sess->upf_n4_seid = be64toh(up_f_seid->seid);

    memset(&param, 0, sizeof(param));
    param.state = SMF_UE_REQUESTED_PDU_SESSION_ESTABLISHMENT;
    param.n1smbuf = gsm_build_pdu_session_establishment_accept(sess);
    ogs_assert(param.n1smbuf);
    param.n2smbuf = ngap_build_pdu_session_resource_setup_request_transfer(
                        sess);
    ogs_assert(param.n2smbuf);

    smf_namf_comm_send_n1_n2_message_transfer(sess, &param);
}

void smf_5gc_n4_handle_session_modification_response(
        smf_sess_t *sess, ogs_pfcp_xact_t *xact,
        ogs_pfcp_session_modification_response_t *rsp)
{
    int status = 0;
    uint64_t flags = 0;
    ogs_sbi_stream_t *stream = NULL;
    smf_bearer_t *qos_flow = NULL;

    ogs_assert(xact);
    ogs_assert(rsp);

    flags = xact->modify_flags;
    ogs_assert(flags);

    /* 'stream' could be NULL in smf_qos_flow_binding() */
    stream = xact->assoc_stream;

    /* If smf_5gc_pfcp_send_qos_flow_modification_request() is called */
    qos_flow = xact->data;

    ogs_pfcp_xact_commit(xact);

    status = OGS_SBI_HTTP_STATUS_OK;

    if (!sess) {
        ogs_warn("No Context");
        status = OGS_SBI_HTTP_STATUS_NOT_FOUND;
    }

    if (rsp->cause.presence) {
        if (rsp->cause.u8 != OGS_PFCP_CAUSE_REQUEST_ACCEPTED) {
            ogs_warn("PFCP Cause [%d] : Not Accepted", rsp->cause.u8);
            status = sbi_status_from_pfcp(rsp->cause.u8);
        }
    } else {
        ogs_error("No Cause");
        status = OGS_SBI_HTTP_STATUS_BAD_REQUEST;
    }

    if (status == OGS_SBI_HTTP_STATUS_OK) {
        int i;

        uint8_t pfcp_cause_value = OGS_PFCP_CAUSE_REQUEST_ACCEPTED;
        uint8_t offending_ie_value = 0;

        ogs_assert(sess);
        for (i = 0; i < OGS_MAX_NUM_OF_PDR; i++) {
            ogs_pfcp_pdr_t *pdr = NULL;
            ogs_pfcp_far_t *far = NULL;

            pdr = ogs_pfcp_handle_created_pdr(
                    &sess->pfcp, &rsp->created_pdr[i],
                    &pfcp_cause_value, &offending_ie_value);

            if (!pdr)
                break;

            far = pdr->far;
            ogs_assert(far);

            if (pdr->src_if == OGS_PFCP_INTERFACE_ACCESS) {
                if (far->dst_if == OGS_PFCP_INTERFACE_CP_FUNCTION)
                    ogs_pfcp_far_teid_hash_set(far);

                ogs_assert(sess->pfcp_node);
                if (sess->pfcp_node->up_function_features.ftup &&
                    pdr->f_teid_len) {

                    if (far->dst_if == OGS_PFCP_INTERFACE_CORE) {
                        if (sess->upf_n3_addr)
                            ogs_freeaddrinfo(sess->upf_n3_addr);
                        if (sess->upf_n3_addr6)
                            ogs_freeaddrinfo(sess->upf_n3_addr6);

                        ogs_assert(OGS_OK ==
                            ogs_pfcp_f_teid_to_sockaddr(
                                &pdr->f_teid, pdr->f_teid_len,
                                &sess->upf_n3_addr, &sess->upf_n3_addr6));
                        sess->upf_n3_teid = pdr->f_teid.teid;
                    } else if (far->dst_if == OGS_PFCP_INTERFACE_ACCESS) {
                        if (sess->handover.upf_dl_addr)
                            ogs_freeaddrinfo(sess->handover.upf_dl_addr);
                        if (sess->handover.upf_dl_addr6)
                            ogs_freeaddrinfo(sess->handover.upf_dl_addr6);

                        ogs_assert(OGS_OK ==
                            ogs_pfcp_f_teid_to_sockaddr(
                                &pdr->f_teid, pdr->f_teid_len,
                                &sess->handover.upf_dl_addr,
                                &sess->handover.upf_dl_addr6));
                        sess->handover.upf_dl_teid = pdr->f_teid.teid;
                    }
                }
            } else if (pdr->src_if == OGS_PFCP_INTERFACE_CP_FUNCTION) {
                ogs_assert(OGS_ERROR != ogs_pfcp_setup_pdr_gtpu_node(pdr));
            }
        }

        status = sbi_status_from_pfcp(pfcp_cause_value);
    }

    if (status != OGS_SBI_HTTP_STATUS_OK) {
        char *strerror = ogs_msprintf(
                "PFCP Cause [%d] : Not Accepted", rsp->cause.u8);
        if (stream)
            smf_sbi_send_sm_context_update_error(
                    stream, status, strerror, NULL, NULL, NULL);
        ogs_error("%s", strerror);
        ogs_free(strerror);
        return;
    }

    ogs_assert(sess);

    if (sess->upf_n3_addr == NULL && sess->upf_n3_addr6 == NULL) {
        if (stream)
            smf_sbi_send_sm_context_update_error(
                    stream, status, "No UP F_TEID", NULL, NULL, NULL);
        return;
    }

    if (flags & OGS_PFCP_MODIFY_ACTIVATE) {
        if (flags & OGS_PFCP_MODIFY_XN_HANDOVER) {
            ogs_pkbuf_t *n2smbuf =
                ngap_build_path_switch_request_ack_transfer(sess);
            ogs_assert(n2smbuf);

            smf_sbi_send_sm_context_updated_data_n2smbuf(sess, stream,
                OpenAPI_n2_sm_info_type_PATH_SWITCH_REQ_ACK, n2smbuf);
        } else if (flags & OGS_PFCP_MODIFY_N2_HANDOVER) {

            if (smf_sess_have_indirect_data_forwarding(sess) == true) {
                ogs_assert(OGS_OK ==
                    smf_5gc_pfcp_send_session_modification_request(
                        sess, stream,
                        OGS_PFCP_MODIFY_INDIRECT|OGS_PFCP_MODIFY_REMOVE,
                        ogs_app()->time.handover.duration));
            }

            smf_sbi_send_sm_context_updated_data_ho_state(
                    sess, stream, OpenAPI_ho_state_COMPLETED);

        } else {
            sess->paging.ue_requested_pdu_session_establishment_done = true;
            ogs_assert(true == ogs_sbi_send_http_status_no_content(stream));
        }

    } else if (flags & OGS_PFCP_MODIFY_DEACTIVATE) {
        if (flags & OGS_PFCP_MODIFY_ERROR_INDICATION) {
            smf_n1_n2_message_transfer_param_t param;

            memset(&param, 0, sizeof(param));
            param.state = SMF_ERROR_INDICATON_RECEIVED_FROM_5G_AN;
            param.n2smbuf =
                ngap_build_pdu_session_resource_release_command_transfer(
                    sess, SMF_NGAP_STATE_ERROR_INDICATION_RECEIVED_FROM_5G_AN,
                    NGAP_Cause_PR_nas, NGAP_CauseNas_normal_release);
            ogs_assert(param.n2smbuf);

            param.skip_ind = true;

            smf_namf_comm_send_n1_n2_message_transfer(sess, &param);
        } else {
            smf_sbi_send_sm_context_updated_data_up_cnx_state(
                    sess, stream, OpenAPI_up_cnx_state_DEACTIVATED);
        }
    /*
     * You should not change the following order to support
     * OGS_PFCP_MODIFY_REMOVE|OGS_PFCP_MODIFY_CREATE.
     *
     * 1. if (flags & OGS_PFCP_MODIFY_REMOVE) {
     * 2. } else if (flags & OGS_PFCP_MODIFY_CREATE) {
     *    }
     */
    } else if (flags & OGS_PFCP_MODIFY_REMOVE) {
        if (flags & OGS_PFCP_MODIFY_INDIRECT) {

            smf_sess_delete_indirect_data_forwarding(sess);

            /*
             * OGS_PFCP_MODIFY_CREATE remains.
             * So now we do some extra work to create an indirect tunnel.
             */
            if (flags & OGS_PFCP_MODIFY_CREATE) {
                smf_sess_create_indirect_data_forwarding(sess);

                ogs_assert(OGS_OK ==
                    smf_5gc_pfcp_send_session_modification_request(
                        sess, stream,
                        OGS_PFCP_MODIFY_INDIRECT|OGS_PFCP_MODIFY_CREATE,
                        0));
            } else if (flags & OGS_PFCP_MODIFY_HANDOVER_CANCEL) {
                smf_sbi_send_sm_context_updated_data_ho_state(
                        sess, stream, OpenAPI_ho_state_CANCELLED);
            }
        }
    } else if (flags & OGS_PFCP_MODIFY_CREATE) {
        if (flags & OGS_PFCP_MODIFY_INDIRECT) {
            ogs_pkbuf_t *n2smbuf = ngap_build_handover_command_transfer(sess);
            ogs_assert(n2smbuf);

            smf_sbi_send_sm_context_updated_data(
                sess, stream, 0, OpenAPI_ho_state_PREPARED,
                NULL, OpenAPI_n2_sm_info_type_HANDOVER_CMD, n2smbuf);

        } else {
            smf_n1_n2_message_transfer_param_t param;

            memset(&param, 0, sizeof(param));
            param.state = SMF_NETWORK_REQUESTED_QOS_FLOW_MODIFICATION;
            param.n1smbuf = gsm_build_qos_flow_modification_command(qos_flow,
                    OGS_NAS_PROCEDURE_TRANSACTION_IDENTITY_UNASSIGNED);
            ogs_assert(param.n1smbuf);
            param.n2smbuf =
                ngap_build_qos_flow_resource_modify_request_transfer(qos_flow);
            ogs_assert(param.n2smbuf);

            smf_namf_comm_send_n1_n2_message_transfer(sess, &param);
        }
    }
}

void smf_5gc_n4_handle_session_deletion_response(
        smf_sess_t *sess, ogs_pfcp_xact_t *xact,
        ogs_pfcp_session_deletion_response_t *rsp)
{
    int status = 0;
    int trigger;

    ogs_sbi_stream_t *stream = NULL;

    ogs_sbi_message_t sendmsg;
    ogs_sbi_response_t *response = NULL;

    ogs_assert(xact);
    ogs_assert(rsp);

    stream = xact->assoc_stream;
    ogs_assert(stream);
    trigger = xact->delete_trigger;
    ogs_assert(trigger);

    ogs_pfcp_xact_commit(xact);

    status = OGS_SBI_HTTP_STATUS_OK;

    if (!sess) {
        ogs_warn("No Context");
        status = OGS_SBI_HTTP_STATUS_NOT_FOUND;
    }

    if (rsp->cause.presence) {
        if (rsp->cause.u8 != OGS_PFCP_CAUSE_REQUEST_ACCEPTED) {
            ogs_warn("PFCP Cause [%d] : Not Accepted", rsp->cause.u8);
            status = sbi_status_from_pfcp(rsp->cause.u8);
        }
    } else {
        ogs_error("No Cause");
        status = OGS_SBI_HTTP_STATUS_BAD_REQUEST;
    }

    if (status != OGS_SBI_HTTP_STATUS_OK) {
        char *strerror = ogs_msprintf(
                "PFCP Cause [%d] : Not Accepted", rsp->cause.u8);
        smf_sbi_send_sm_context_update_error(
                stream, status, strerror, NULL, NULL, NULL);
        ogs_error("%s", strerror);
        ogs_free(strerror);
        return;
    }

    ogs_assert(sess);

    if (trigger == OGS_PFCP_DELETE_TRIGGER_UE_REQUESTED) {
        ogs_pkbuf_t *n1smbuf = NULL, *n2smbuf = NULL;

        n1smbuf = gsm_build_pdu_session_release_command(
                sess, OGS_5GSM_CAUSE_REGULAR_DEACTIVATION);
        ogs_assert(n1smbuf);

        n2smbuf = ngap_build_pdu_session_resource_release_command_transfer(
                sess, SMF_NGAP_STATE_DELETE_TRIGGER_UE_REQUESTED,
                NGAP_Cause_PR_nas, NGAP_CauseNas_normal_release);
        ogs_assert(n2smbuf);

        smf_sbi_send_sm_context_updated_data_n1_n2_message(sess, stream,
                n1smbuf, OpenAPI_n2_sm_info_type_PDU_RES_REL_CMD, n2smbuf);
    } else {
        memset(&sendmsg, 0, sizeof(sendmsg));

        response = ogs_sbi_build_response(
                &sendmsg, OGS_SBI_HTTP_STATUS_NO_CONTENT);
        ogs_assert(response);
        ogs_assert(true == ogs_sbi_server_send_response(stream, response));

        SMF_SESS_CLEAR(sess);
    }
}

void smf_epc_n4_handle_session_establishment_response(
        smf_sess_t *sess, ogs_pfcp_xact_t *xact,
        ogs_pfcp_session_establishment_response_t *rsp)
{
    uint8_t cause_value = 0;

    smf_bearer_t *bearer = NULL;
    ogs_gtp_xact_t *gtp_xact = NULL;

    ogs_pfcp_f_seid_t *up_f_seid = NULL;

    ogs_assert(xact);
    ogs_assert(rsp);

    gtp_xact = xact->assoc_xact;
    ogs_assert(gtp_xact);

    ogs_pfcp_xact_commit(xact);

    cause_value = OGS_GTP_CAUSE_REQUEST_ACCEPTED;

    if (!sess) {
        ogs_warn("No Context");
        cause_value = OGS_GTP_CAUSE_CONTEXT_NOT_FOUND;
    }

    if (rsp->up_f_seid.presence == 0) {
        ogs_error("No UP F-SEID");
        cause_value = OGS_GTP_CAUSE_MANDATORY_IE_MISSING;
    }

    if (rsp->created_pdr[0].presence == 0) {
        ogs_error("No Created PDR");
        cause_value = OGS_GTP_CAUSE_MANDATORY_IE_MISSING;
    }

    if (rsp->cause.presence) {
        if (rsp->cause.u8 != OGS_PFCP_CAUSE_REQUEST_ACCEPTED) {
            ogs_warn("PFCP Cause [%d] : Not Accepted", rsp->cause.u8);
            cause_value = gtp_cause_from_pfcp(rsp->cause.u8);
        }
    } else {
        ogs_error("No Cause");
        cause_value = OGS_GTP_CAUSE_MANDATORY_IE_MISSING;
    }

    if (cause_value == OGS_GTP_CAUSE_REQUEST_ACCEPTED) {
        int i;

        uint8_t pfcp_cause_value = OGS_PFCP_CAUSE_REQUEST_ACCEPTED;
        uint8_t offending_ie_value = 0;

        ogs_assert(sess);
        for (i = 0; i < OGS_MAX_NUM_OF_PDR; i++) {
            ogs_pfcp_pdr_t *pdr = NULL;
            ogs_pfcp_far_t *far = NULL;

            pdr = ogs_pfcp_handle_created_pdr(
                    &sess->pfcp, &rsp->created_pdr[i],
                    &pfcp_cause_value, &offending_ie_value);

            if (!pdr)
                break;

            far = pdr->far;
            ogs_assert(far);

            if (pdr->src_if == OGS_PFCP_INTERFACE_ACCESS) {
                if (far->dst_if == OGS_PFCP_INTERFACE_CP_FUNCTION)
                    ogs_pfcp_far_teid_hash_set(far);

                bearer = smf_bearer_find_by_pdr_id(sess, pdr->id);
                if (bearer) {
                    ogs_assert(sess->pfcp_node);
                    if (sess->pfcp_node->up_function_features.ftup &&
                        pdr->f_teid_len) {
                        if (bearer->pgw_s5u_addr)
                            ogs_freeaddrinfo(bearer->pgw_s5u_addr);
                        if (bearer->pgw_s5u_addr)
                            ogs_freeaddrinfo(bearer->pgw_s5u_addr6);

                        ogs_assert(OGS_OK ==
                            ogs_pfcp_f_teid_to_sockaddr(
                                &pdr->f_teid, pdr->f_teid_len,
                                &bearer->pgw_s5u_addr, &bearer->pgw_s5u_addr6));
                        bearer->pgw_s5u_teid = pdr->f_teid.teid;
                    }
                }
            } else if (pdr->src_if == OGS_PFCP_INTERFACE_CP_FUNCTION) {
                ogs_assert(OGS_ERROR != ogs_pfcp_setup_pdr_gtpu_node(pdr));
            }
        }

        cause_value = gtp_cause_from_pfcp(pfcp_cause_value);
    }

    if (cause_value != OGS_GTP_CAUSE_REQUEST_ACCEPTED) {
        ogs_gtp_send_error_message(gtp_xact, sess ? sess->sgw_s5c_teid : 0,
                OGS_GTP_CREATE_SESSION_RESPONSE_TYPE, cause_value);
        return;
    }

    ogs_assert(sess);
    bearer = smf_default_bearer_in_sess(sess);
    ogs_assert(bearer);

    if (bearer->pgw_s5u_addr == NULL && bearer->pgw_s5u_addr6 == NULL) {
        ogs_error("No UP F-TEID");
        ogs_gtp_send_error_message(gtp_xact, sess ? sess->sgw_s5c_teid : 0,
                OGS_GTP_CREATE_SESSION_RESPONSE_TYPE,
                OGS_GTP_CAUSE_GRE_KEY_NOT_FOUND);
        return;
    }

    /* UP F-SEID */
    up_f_seid = rsp->up_f_seid.data;
    ogs_assert(up_f_seid);
    sess->upf_n4_seid = be64toh(up_f_seid->seid);

    ogs_assert(OGS_OK == smf_gtp_send_create_session_response(sess, gtp_xact));

    if (sess->gtp_rat_type == OGS_GTP_RAT_TYPE_WLAN) {
        smf_ue_t *smf_ue = NULL;
        smf_sess_t *eutran_sess = NULL;

        smf_ue = sess->smf_ue;
        ogs_assert(smf_ue);

        ogs_assert(sess->session.name);
        eutran_sess = smf_sess_find_by_apn(
                smf_ue, sess->session.name, OGS_GTP_RAT_TYPE_EUTRAN);
        if (eutran_sess) {
            smf_bearer_t *eutran_linked_bearer =
                ogs_list_first(&eutran_sess->bearer_list);
            ogs_assert(eutran_linked_bearer);

            ogs_assert(OGS_OK ==
                smf_gtp_send_delete_bearer_request(
                    eutran_linked_bearer,
                    OGS_NAS_PROCEDURE_TRANSACTION_IDENTITY_UNASSIGNED,
                    OGS_GTP_CAUSE_RAT_CHANGED_FROM_3GPP_TO_NON_3GPP));
        }
    }

    smf_bearer_binding(sess);
}

void smf_epc_n4_handle_session_modification_response(
        smf_sess_t *sess, ogs_pfcp_xact_t *xact,
        ogs_pfcp_session_modification_response_t *rsp)
{
    int i;

    smf_bearer_t *bearer = NULL;
    uint64_t flags = 0;

    uint8_t pfcp_cause_value = OGS_PFCP_CAUSE_REQUEST_ACCEPTED;
    uint8_t offending_ie_value = 0;

    ogs_assert(xact);
    ogs_assert(rsp);

    bearer = xact->data;
    ogs_assert(bearer);
    flags = xact->modify_flags;
    ogs_assert(flags);

    ogs_pfcp_xact_commit(xact);

    if (!sess) {
        ogs_error("No Context");
        return;
    }

    if (rsp->cause.presence) {
        if (rsp->cause.u8 != OGS_PFCP_CAUSE_REQUEST_ACCEPTED) {
            ogs_error("PFCP Cause [%d] : Not Accepted", rsp->cause.u8);
            return;
        }
    } else {
        ogs_error("No Cause");
        return;
    }

    ogs_assert(sess);

    pfcp_cause_value = OGS_PFCP_CAUSE_REQUEST_ACCEPTED;
    for (i = 0; i < OGS_MAX_NUM_OF_PDR; i++) {
        ogs_pfcp_pdr_t *pdr = NULL;
        ogs_pfcp_far_t *far = NULL;

        pdr = ogs_pfcp_handle_created_pdr(
                &sess->pfcp, &rsp->created_pdr[i],
                &pfcp_cause_value, &offending_ie_value);

        if (!pdr)
            break;

        far = pdr->far;
        ogs_assert(far);

        if (pdr->src_if == OGS_PFCP_INTERFACE_ACCESS) {
            if (far->dst_if == OGS_PFCP_INTERFACE_CP_FUNCTION)
                ogs_pfcp_far_teid_hash_set(far);

            ogs_assert(sess->pfcp_node);
            if (sess->pfcp_node->up_function_features.ftup &&
                pdr->f_teid_len) {
                if (bearer->pgw_s5u_addr)
                    ogs_freeaddrinfo(bearer->pgw_s5u_addr);
                if (bearer->pgw_s5u_addr)
                    ogs_freeaddrinfo(bearer->pgw_s5u_addr6);

                ogs_assert(OGS_OK ==
                    ogs_pfcp_f_teid_to_sockaddr(
                        &pdr->f_teid, pdr->f_teid_len,
                        &bearer->pgw_s5u_addr, &bearer->pgw_s5u_addr6));
                bearer->pgw_s5u_teid = pdr->f_teid.teid;
            }
        } else if (pdr->src_if == OGS_PFCP_INTERFACE_CP_FUNCTION) {
            ogs_assert(OGS_ERROR != ogs_pfcp_setup_pdr_gtpu_node(pdr));
        }
    }

    if (pfcp_cause_value != OGS_PFCP_CAUSE_REQUEST_ACCEPTED) {
        ogs_error("PFCP Cause [%d] : Not Accepted", pfcp_cause_value);
        return;
    }

    if (flags & OGS_PFCP_MODIFY_REMOVE) {
        smf_bearer_remove(bearer);

    } else if (flags & OGS_PFCP_MODIFY_CREATE) {
        ogs_assert(OGS_OK == smf_gtp_send_create_bearer_request(bearer));

    } else if (flags & OGS_PFCP_MODIFY_ACTIVATE) {
        /* Nothing */
    }
}

void smf_epc_n4_handle_session_deletion_response(
        smf_sess_t *sess, ogs_pfcp_xact_t *xact,
        ogs_pfcp_session_deletion_response_t *rsp)
{
    uint8_t cause_value = 0;
    ogs_gtp_xact_t *gtp_xact = NULL;

    ogs_assert(xact);
    ogs_assert(rsp);

    gtp_xact = xact->assoc_xact;

    ogs_pfcp_xact_commit(xact);

    cause_value = OGS_GTP_CAUSE_REQUEST_ACCEPTED;

    if (!sess) {
        ogs_warn("No Context");
        cause_value = OGS_GTP_CAUSE_CONTEXT_NOT_FOUND;
    }

    if (rsp->cause.presence) {
        if (rsp->cause.u8 != OGS_PFCP_CAUSE_REQUEST_ACCEPTED) {
            ogs_warn("PFCP Cause[%d] : Not Accepted", rsp->cause.u8);
            cause_value = gtp_cause_from_pfcp(rsp->cause.u8);
        }
    } else {
        ogs_error("No Cause");
        cause_value = OGS_GTP_CAUSE_MANDATORY_IE_MISSING;
    }

    if (cause_value != OGS_GTP_CAUSE_REQUEST_ACCEPTED) {
        if (gtp_xact)
            ogs_gtp_send_error_message(gtp_xact, sess ? sess->sgw_s5c_teid : 0,
                    OGS_GTP_DELETE_SESSION_RESPONSE_TYPE, cause_value);
        return;
    }

    ogs_assert(sess);

    if (gtp_xact) {
        /*
         * 1. MME sends Delete Session Request to SGW/SMF.
         * 2. SMF sends Delete Session Response to SGW/MME.
         */
        ogs_assert(OGS_OK ==
                smf_gtp_send_delete_session_response(sess, gtp_xact));
    } else {
        /*
         * 1. SMF sends Delete Bearer Request(DEFAULT BEARER) to SGW/MME.
         * 2. MME sends Delete Bearer Response to SGW/SMF.
         *
         * OR
         *
         * 1. SMF sends Delete Bearer Request(DEFAULT BEARER) to ePDG.
         * 2. ePDG sends Delete Bearer Response(DEFAULT BEARER) to SMF.
         *
         * Note that the following messages are not processed here.
         * - Bearer Resource Command
         * - Delete Bearer Request/Response with DEDICATED BEARER.
         */
    }

    SMF_SESS_CLEAR(sess);
}

void smf_n4_handle_session_report_request(
        smf_sess_t *sess, ogs_pfcp_xact_t *pfcp_xact,
        ogs_pfcp_session_report_request_t *pfcp_req)
{
    smf_bearer_t *qos_flow = NULL;
    ogs_pfcp_pdr_t *pdr = NULL;

    ogs_pfcp_report_type_t report_type;
    uint8_t cause_value = 0;
    uint16_t pdr_id = 0;

    ogs_assert(pfcp_xact);
    ogs_assert(pfcp_req);

    cause_value = OGS_GTP_CAUSE_REQUEST_ACCEPTED;

    if (!sess) {
        ogs_warn("No Context");
        cause_value = OGS_PFCP_CAUSE_SESSION_CONTEXT_NOT_FOUND;
    }

    if (pfcp_req->report_type.presence == 0) {
        ogs_error("No Report Type");
        cause_value = OGS_GTP_CAUSE_MANDATORY_IE_MISSING;
    }

    if (cause_value != OGS_GTP_CAUSE_REQUEST_ACCEPTED) {
        ogs_pfcp_send_error_message(pfcp_xact, 0,
                OGS_PFCP_SESSION_REPORT_RESPONSE_TYPE,
                cause_value, 0);
        return;
    }

    ogs_assert(sess);
    report_type.value = pfcp_req->report_type.u8;

    if (report_type.downlink_data_report) {
        ogs_pfcp_downlink_data_service_information_t *info = NULL;
        uint8_t paging_policy_indication_value = 0;
        uint8_t qfi = 0;

        if (pfcp_req->downlink_data_report.presence) {
            if (pfcp_req->downlink_data_report.
                    downlink_data_service_information.presence) {
                info = pfcp_req->downlink_data_report.
                    downlink_data_service_information.data;
                if (info) {
                    if (info->qfii && info->ppi) {
                        paging_policy_indication_value =
                            info->paging_policy_indication_value;
                        qfi = info->qfi;
                    } else if (info->qfii) {
                        qfi = info->qfi;
                    } else if (info->ppi) {
                        paging_policy_indication_value =
                            info->paging_policy_indication_value;
                    } else {
                        ogs_error("Invalid Downlink Data Service Information");
                    }

                    if (paging_policy_indication_value) {
                        ogs_warn("Not implement - "
                                "Paging Policy Indication Value");
                        ogs_pfcp_send_error_message(pfcp_xact, 0,
                                OGS_PFCP_SESSION_REPORT_RESPONSE_TYPE,
                                OGS_GTP_CAUSE_SERVICE_NOT_SUPPORTED, 0);
                        return;
                    }

                    if (qfi) {
                        qos_flow = smf_qos_flow_find_by_qfi(sess, qfi);
                        if (!qos_flow)
                            ogs_error("Cannot find the QoS Flow[%d]", qfi);
                    }
                } else {
                    ogs_error("No Info");
                }
            }

            if (pfcp_req->downlink_data_report.pdr_id.presence) {
                pdr = ogs_pfcp_pdr_find(&sess->pfcp,
                    pfcp_req->downlink_data_report.pdr_id.u16);
                if (!pdr)
                    ogs_error("Cannot find the PDR-ID[%d]", pdr_id);

            } else {
                ogs_error("No PDR-ID");
            }
        } else {
            ogs_error("No Downlink Data Report");
        }

        if (!pdr || !qos_flow) {
            ogs_error("No Context [%p:%p]", pdr, qos_flow);
            ogs_pfcp_send_error_message(pfcp_xact, 0,
                    OGS_PFCP_SESSION_REPORT_RESPONSE_TYPE,
                    cause_value, 0);
            return;
        }

        ogs_assert(OGS_OK ==
            smf_pfcp_send_session_report_response(
                pfcp_xact, sess, OGS_PFCP_CAUSE_REQUEST_ACCEPTED));

        if (sess->paging.ue_requested_pdu_session_establishment_done == true) {
            smf_n1_n2_message_transfer_param_t param;

            memset(&param, 0, sizeof(param));
            param.state = SMF_NETWORK_TRIGGERED_SERVICE_REQUEST;
            param.n2smbuf =
                ngap_build_pdu_session_resource_setup_request_transfer(sess);
            ogs_assert(param.n2smbuf);

            param.n1n2_failure_txf_notif_uri = true;

            smf_namf_comm_send_n1_n2_message_transfer(sess, &param);
        }

    } else if (report_type.error_indication_report) {
        smf_ue_t *smf_ue = sess->smf_ue;
        smf_sess_t *error_indication_session = NULL;
        ogs_assert(smf_ue);

        ogs_assert(OGS_OK ==
            smf_pfcp_send_session_report_response(
                pfcp_xact, sess, OGS_PFCP_CAUSE_REQUEST_ACCEPTED));

        error_indication_session = smf_sess_find_by_error_indication_report(
                smf_ue, &pfcp_req->error_indication_report);

        if (!error_indication_session) return;

        ogs_assert(OGS_OK ==
            smf_5gc_pfcp_send_session_modification_request(
                error_indication_session, NULL,
                OGS_PFCP_MODIFY_DL_ONLY|OGS_PFCP_MODIFY_DEACTIVATE|
                OGS_PFCP_MODIFY_ERROR_INDICATION,
                0));

    } else {
        ogs_error("Not supported Report Type[%d]", report_type.value);
        ogs_assert(OGS_OK ==
            smf_pfcp_send_session_report_response(
                pfcp_xact, sess, OGS_PFCP_CAUSE_SYSTEM_FAILURE));
    }
}
