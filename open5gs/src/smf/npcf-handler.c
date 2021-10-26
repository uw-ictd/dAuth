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

#include "npcf-handler.h"

#include "sbi-path.h"
#include "pfcp-path.h"
#include "nas-path.h"

bool smf_npcf_smpolicycontrol_handle_create(
        smf_sess_t *sess, ogs_sbi_stream_t *stream, int state,
        ogs_sbi_message_t *recvmsg)
{
    int rv;
    char buf1[OGS_ADDRSTRLEN];
    char buf2[OGS_ADDRSTRLEN];

    uint64_t supported_features;

    char *strerror = NULL;
    smf_ue_t *smf_ue = NULL;

    smf_bearer_t *qos_flow = NULL;
    ogs_pfcp_pdr_t *dl_pdr = NULL;
    ogs_pfcp_pdr_t *ul_pdr = NULL;
    ogs_pfcp_pdr_t *cp2up_pdr = NULL;
    ogs_pfcp_pdr_t *up2cp_pdr = NULL;
    ogs_pfcp_far_t *up2cp_far = NULL;
    ogs_pfcp_qer_t *qer = NULL;

    OpenAPI_sm_policy_decision_t *SmPolicyDecision = NULL;
    OpenAPI_lnode_t *node = NULL, *node2 = NULL;

#define MAX_TRIGGER_ID 128
    bool trigger_results[MAX_TRIGGER_ID];

    ogs_sbi_message_t message;
    ogs_sbi_header_t header;

    ogs_assert(sess);
    ogs_assert(stream);
    smf_ue = sess->smf_ue;
    ogs_assert(smf_ue);

    ogs_assert(recvmsg);

    if (!recvmsg->http.location) {
        strerror = ogs_msprintf("[%s:%d] No http.location",
                smf_ue->supi, sess->psi);
        goto cleanup;
    }

    SmPolicyDecision = recvmsg->SmPolicyDecision;
    if (!SmPolicyDecision) {
        strerror = ogs_msprintf("[%s:%d] No SmPolicyDecision",
                smf_ue->supi, sess->psi);
        goto cleanup;
    }

    memset(&header, 0, sizeof(header));
    header.uri = recvmsg->http.location;

    rv = ogs_sbi_parse_header(&message, &header);
    if (rv != OGS_OK) {
        strerror = ogs_msprintf("[%s:%d] Cannot parse http.location [%s]",
                smf_ue->supi, sess->psi, recvmsg->http.location);
        goto cleanup;
    }

    if (!message.h.resource.component[1]) {
        strerror = ogs_msprintf("[%s:%d] No Assocication ID [%s]",
                smf_ue->supi, sess->psi, recvmsg->http.location);

        ogs_sbi_header_free(&header);
        goto cleanup;
    }

    if (sess->policy_association_id)
        ogs_free(sess->policy_association_id);
    sess->policy_association_id = ogs_strdup(message.h.resource.component[1]);
    ogs_assert(sess->policy_association_id);

    ogs_sbi_header_free(&header);

    /* SBI Features */
    if (SmPolicyDecision->supp_feat) {
        supported_features =
            ogs_uint64_from_string(SmPolicyDecision->supp_feat);
        sess->smpolicycontrol_features &= supported_features;
    } else {
        sess->smpolicycontrol_features = 0;
    }

    /*********************************************************************
     * Handle Policy Control Request Triggers
     *********************************************************************/

    /* Get policy control request triggers */
    memset(&trigger_results, 0, sizeof(trigger_results));
    OpenAPI_list_for_each(SmPolicyDecision->policy_ctrl_req_triggers, node) {
        if (node->data) {
            OpenAPI_policy_control_request_trigger_e trigger_id =
                (intptr_t)node->data;

            ogs_assert(trigger_id < MAX_TRIGGER_ID);
            trigger_results[trigger_id] = true;
        }
    }

    /* Update authorized session-AMBR */
    if (SmPolicyDecision->sess_rules) {
        OpenAPI_map_t *SessRuleMap = NULL;
        OpenAPI_session_rule_t *SessionRule = NULL;

        OpenAPI_ambr_t *AuthSessAmbr = NULL;
        OpenAPI_authorized_default_qos_t *AuthDefQos = NULL;

        OpenAPI_list_for_each(SmPolicyDecision->sess_rules, node) {
            SessRuleMap = node->data;
            if (!SessRuleMap) {
                ogs_error("No SessRuleMap");
                continue;
            }

            SessionRule = SessRuleMap->value;
            if (!SessionRule) {
                ogs_error("No SessionRule");
                continue;
            }


            AuthSessAmbr = SessionRule->auth_sess_ambr;
            if (AuthSessAmbr && trigger_results[
                OpenAPI_policy_control_request_trigger_SE_AMBR_CH] == true) {
                if (AuthSessAmbr->uplink)
                    sess->session.ambr.uplink =
                        ogs_sbi_bitrate_from_string(AuthSessAmbr->uplink);
                if (AuthSessAmbr->downlink)
                    sess->session.ambr.downlink =
                        ogs_sbi_bitrate_from_string(AuthSessAmbr->downlink);
            }

            AuthDefQos = SessionRule->auth_def_qos;
            if (AuthDefQos && trigger_results[
                OpenAPI_policy_control_request_trigger_DEF_QOS_CH] == true) {
                sess->session.qos.index = AuthDefQos->_5qi;
                sess->session.qos.arp.priority_level =
                    AuthDefQos->priority_level;
                if (AuthDefQos->arp) {
                    sess->session.qos.arp.priority_level =
                            AuthDefQos->arp->priority_level;
                    if (AuthDefQos->arp->preempt_cap ==
                        OpenAPI_preemption_capability_NOT_PREEMPT)
                        sess->session.qos.arp.pre_emption_capability =
                            OGS_5GC_PRE_EMPTION_DISABLED;
                    else if (AuthDefQos->arp->preempt_cap ==
                        OpenAPI_preemption_capability_MAY_PREEMPT)
                        sess->session.qos.arp.pre_emption_capability =
                            OGS_5GC_PRE_EMPTION_ENABLED;
                    ogs_assert(sess->session.qos.arp.pre_emption_capability);

                    if (AuthDefQos->arp->preempt_vuln ==
                        OpenAPI_preemption_vulnerability_NOT_PREEMPTABLE)
                        sess->session.qos.arp.pre_emption_vulnerability =
                            OGS_5GC_PRE_EMPTION_DISABLED;
                    else if (AuthDefQos->arp->preempt_vuln ==
                        OpenAPI_preemption_vulnerability_PREEMPTABLE)
                        sess->session.qos.arp.pre_emption_vulnerability =
                            OGS_5GC_PRE_EMPTION_ENABLED;
                    ogs_assert(sess->session.qos.arp.pre_emption_vulnerability);
                }
            }
        }
    }

    /* Update authorized PCC rule & QoS */
    if (SmPolicyDecision->pcc_rules) {
        OpenAPI_map_t *PccRuleMap = NULL;
        OpenAPI_pcc_rule_t *PccRule = NULL;
        OpenAPI_flow_information_t *FlowInformation = NULL;
        OpenAPI_qos_data_t *QosData = NULL;
        char *QosId = NULL;

        ogs_assert(sess->num_of_pcc_rule == 0);
        OpenAPI_list_for_each(SmPolicyDecision->pcc_rules, node) {
            ogs_pcc_rule_t *pcc_rule = &sess->pcc_rule[sess->num_of_pcc_rule];

            ogs_assert(pcc_rule);

            PccRuleMap = node->data;
            if (!PccRuleMap) {
                ogs_error("No PccRuleMap");
                continue;
            }

            PccRule = PccRuleMap->value;
            if (!PccRule) {
                ogs_error("No PccRule");
                continue;
            }

            if (!PccRule->ref_qos_data) {
                ogs_error("No RefQosData");
                continue;
            }

            if (!PccRule->ref_qos_data->first) {
                ogs_error("No RefQosData->first");
                continue;
            }

            QosId = PccRule->ref_qos_data->first->data;
            if (!QosId) {
                ogs_error("no QosId");
                continue;
            }

            if (SmPolicyDecision->qos_decs) {
                OpenAPI_map_t *QosDecisionMap = NULL;
                OpenAPI_qos_data_t *QosDataIter = NULL;

                OpenAPI_list_for_each(SmPolicyDecision->qos_decs, node2) {
                    QosDecisionMap = node2->data;
                    if (!QosDecisionMap) {
                        ogs_error("No QosDecisionMap");
                        continue;
                    }

                    QosDataIter = QosDecisionMap->value;
                    if (!QosDataIter) {
                        ogs_error("No QosData");
                        continue;
                    }

                    if (!QosDataIter->qos_id) {
                        ogs_error("No QosId");
                        continue;

                    }

                    if (strcmp(QosId, QosDataIter->qos_id) == 0) {
                        QosData = QosDataIter;
                        break;
                    }
                }
            }

            if (!QosData) {
                ogs_error("no qosData");
                continue;
            }

            pcc_rule->type = OGS_PCC_RULE_TYPE_INSTALL;
            pcc_rule->id = ogs_strdup(PccRule->pcc_rule_id);
            ogs_assert(pcc_rule->id);
            pcc_rule->precedence = PccRule->precedence;

            if (PccRule->flow_infos) {
                ogs_assert(pcc_rule->num_of_flow == 0);
                OpenAPI_list_for_each(PccRule->flow_infos, node2) {
                    ogs_flow_t *flow = &pcc_rule->flow[pcc_rule->num_of_flow];

                    ogs_assert(flow);

                    FlowInformation = node2->data;
                    if (!FlowInformation) {
                        ogs_error("No FlowInformation");
                        continue;
                    }

                    if (FlowInformation->flow_direction ==
                        OpenAPI_flow_direction_UPLINK)
                        flow->direction = OGS_FLOW_UPLINK_ONLY;
                    else if (FlowInformation->flow_direction ==
                        OpenAPI_flow_direction_DOWNLINK)
                        flow->direction = OGS_FLOW_DOWNLINK_ONLY;
                    else {
                        ogs_fatal("Unsupported direction [%d]",
                                FlowInformation->flow_direction);
                        ogs_assert_if_reached();
                    }

                    flow->description =
                        ogs_strdup(FlowInformation->flow_description);
                    ogs_assert(flow->description);

                    pcc_rule->num_of_flow++;
                }
            }

            pcc_rule->qos.index = QosData->_5qi;
            pcc_rule->qos.arp.priority_level = QosData->priority_level;

            if (QosData->arp) {
                pcc_rule->qos.arp.priority_level = QosData->arp->priority_level;
                if (QosData->arp->preempt_cap ==
                    OpenAPI_preemption_capability_NOT_PREEMPT)
                    pcc_rule->qos.arp.pre_emption_capability =
                        OGS_5GC_PRE_EMPTION_DISABLED;
                else if (QosData->arp->preempt_cap ==
                    OpenAPI_preemption_capability_MAY_PREEMPT)
                    pcc_rule->qos.arp.pre_emption_capability =
                        OGS_5GC_PRE_EMPTION_ENABLED;
                ogs_assert(pcc_rule->qos.arp.pre_emption_capability);

                if (QosData->arp->preempt_vuln ==
                    OpenAPI_preemption_vulnerability_NOT_PREEMPTABLE)
                    pcc_rule->qos.arp.pre_emption_vulnerability =
                        OGS_5GC_PRE_EMPTION_DISABLED;
                else if (QosData->arp->preempt_vuln ==
                    OpenAPI_preemption_vulnerability_PREEMPTABLE)
                    pcc_rule->qos.arp.pre_emption_vulnerability =
                        OGS_5GC_PRE_EMPTION_ENABLED;
                ogs_assert(pcc_rule->qos.arp.pre_emption_vulnerability);
            }

            if (QosData->maxbr_ul)
                pcc_rule->qos.mbr.uplink =
                    ogs_sbi_bitrate_from_string(QosData->maxbr_ul);
            if (QosData->maxbr_dl)
                pcc_rule->qos.mbr.downlink =
                    ogs_sbi_bitrate_from_string(QosData->maxbr_dl);

            if (QosData->gbr_ul)
                pcc_rule->qos.gbr.uplink =
                    ogs_sbi_bitrate_from_string(QosData->gbr_ul);
            if (QosData->gbr_dl)
                pcc_rule->qos.gbr.downlink =
                    ogs_sbi_bitrate_from_string(QosData->gbr_dl);

            if (pcc_rule->qos.mbr.downlink || pcc_rule->qos.mbr.uplink ||
                pcc_rule->qos.gbr.downlink || pcc_rule->qos.gbr.uplink) {
                if (pcc_rule->qos.mbr.downlink == 0)
                    pcc_rule->qos.mbr.downlink = MAX_BIT_RATE;
                if (pcc_rule->qos.mbr.uplink == 0)
                    pcc_rule->qos.mbr.uplink = MAX_BIT_RATE;
                if (pcc_rule->qos.gbr.downlink == 0)
                    pcc_rule->qos.gbr.downlink = MAX_BIT_RATE;
                if (pcc_rule->qos.gbr.uplink == 0)
                    pcc_rule->qos.gbr.uplink = MAX_BIT_RATE;
            }

            sess->num_of_pcc_rule++;
        }
    }

    /*********************************************************************
     * Send PFCP Session Establiashment Request to the UPF
     *********************************************************************/

    /* Select UPF based on UE Location Information */
    smf_sess_select_upf(sess);

    /* Check if selected UPF is associated with SMF */
    ogs_assert(sess->pfcp_node);
    if (!OGS_FSM_CHECK(&sess->pfcp_node->sm, smf_pfcp_state_associated)) {
        ogs_error("[%s] No associated UPF", smf_ue->supi);
        return false;
    }

    /* Remove all previous QoS flow */
    smf_bearer_remove_all(sess);

    /* Setup Default QoS flow */
    qos_flow = smf_qos_flow_add(sess);
    ogs_assert(qos_flow);

    /* Setup CP/UP Data Forwarding PDR/FAR */
    smf_sess_create_cp_up_data_forwarding(sess);

    /* Copy Session QoS information to Default QoS Flow */
    memcpy(&qos_flow->qos, &sess->session.qos, sizeof(ogs_qos_t));

    /* Setup QER */
    qer = qos_flow->qer;
    ogs_assert(qer);
    qer->mbr.uplink = sess->session.ambr.uplink;
    qer->mbr.downlink = sess->session.ambr.downlink;

    /* Setup PDR */
    dl_pdr = qos_flow->dl_pdr;
    ogs_assert(dl_pdr);
    ul_pdr = qos_flow->ul_pdr;
    ogs_assert(ul_pdr);
    cp2up_pdr = sess->cp2up_pdr;
    ogs_assert(cp2up_pdr);
    up2cp_pdr = sess->up2cp_pdr;
    ogs_assert(up2cp_pdr);

    /* Setup FAR */
    up2cp_far = sess->up2cp_far;
    ogs_assert(up2cp_far);

    ogs_assert(OGS_OK ==
        ogs_pfcp_paa_to_ue_ip_addr(&sess->session.paa,
            &dl_pdr->ue_ip_addr, &dl_pdr->ue_ip_addr_len));
    dl_pdr->ue_ip_addr.sd = OGS_PFCP_UE_IP_DST;

    ogs_info("UE SUPI[%s] DNN[%s] IPv4[%s] IPv6[%s]",
	    smf_ue->supi, sess->session.name,
        sess->ipv4 ? OGS_INET_NTOP(&sess->ipv4->addr, buf1) : "",
        sess->ipv6 ? OGS_INET6_NTOP(&sess->ipv6->addr, buf2) : "");

    /* Set UE-to-CP Flow-Description and Outer-Header-Creation */
    up2cp_pdr->flow_description[up2cp_pdr->num_of_flow++] =
        (char *)"permit out 58 from ff02::2/128 to assigned";
    ogs_assert(OGS_OK ==
        ogs_pfcp_ip_to_outer_header_creation(
            &ogs_gtp_self()->gtpu_ip,
            &up2cp_far->outer_header_creation,
            &up2cp_far->outer_header_creation_len));
    up2cp_far->outer_header_creation.teid = sess->index;

    /* Set UPF-N3 TEID & ADDR to the Default UL PDR */
    ogs_assert(sess->pfcp_node);
    if (sess->pfcp_node->up_function_features.ftup) {
        ul_pdr->f_teid.ch = 1;
        ul_pdr->f_teid.chid = 1;
        ul_pdr->f_teid.choose_id = OGS_PFCP_DEFAULT_CHOOSE_ID;
        ul_pdr->f_teid_len = 2;

        cp2up_pdr->f_teid.ch = 1;
        cp2up_pdr->f_teid_len = 1;

        up2cp_pdr->f_teid.ch = 1;
        up2cp_pdr->f_teid.chid = 1;
        up2cp_pdr->f_teid.choose_id = OGS_PFCP_DEFAULT_CHOOSE_ID;
        up2cp_pdr->f_teid_len = 2;
    } else {
        char buf[OGS_ADDRSTRLEN];
        ogs_gtpu_resource_t *resource = NULL;
        ogs_sockaddr_t *addr = sess->pfcp_node->sa_list;
        ogs_assert(addr);

        ogs_error("F-TEID allocation/release not supported with peer [%s]:%d",
                OGS_ADDR(addr, buf), OGS_PORT(addr));

        resource = ogs_pfcp_find_gtpu_resource(
                &sess->pfcp_node->gtpu_resource_list,
                sess->session.name, OGS_PFCP_INTERFACE_ACCESS);
        if (resource) {
            ogs_user_plane_ip_resource_info_to_sockaddr(&resource->info,
                &sess->upf_n3_addr, &sess->upf_n3_addr6);
            if (resource->info.teidri)
                sess->upf_n3_teid = OGS_PFCP_GTPU_INDEX_TO_TEID(
                        sess->index, resource->info.teidri,
                        resource->info.teid_range);
            else
                sess->upf_n3_teid = sess->index;
        } else {
            if (sess->pfcp_node->addr.ogs_sa_family == AF_INET)
                ogs_assert(OGS_OK ==
                    ogs_copyaddrinfo(
                        &sess->upf_n3_addr, &sess->pfcp_node->addr));
            else if (sess->pfcp_node->addr.ogs_sa_family == AF_INET6)
                ogs_assert(OGS_OK ==
                    ogs_copyaddrinfo(
                        &sess->upf_n3_addr6, &sess->pfcp_node->addr));
            else
                ogs_assert_if_reached();

            sess->upf_n3_teid = sess->index;
        }

        ogs_assert(OGS_OK ==
            ogs_pfcp_sockaddr_to_f_teid(
                sess->upf_n3_addr, sess->upf_n3_addr6,
                &ul_pdr->f_teid, &ul_pdr->f_teid_len));
        ul_pdr->f_teid.teid = sess->upf_n3_teid;

        ogs_assert(OGS_OK ==
            ogs_pfcp_sockaddr_to_f_teid(
                ogs_gtp_self()->gtpu_addr, ogs_gtp_self()->gtpu_addr6,
                &cp2up_pdr->f_teid, &cp2up_pdr->f_teid_len));
        cp2up_pdr->f_teid.teid = sess->index;

        ogs_assert(OGS_OK ==
            ogs_pfcp_sockaddr_to_f_teid(
                sess->upf_n3_addr, sess->upf_n3_addr6,
                &up2cp_pdr->f_teid, &up2cp_pdr->f_teid_len));
        up2cp_pdr->f_teid.teid = sess->upf_n3_teid;
    }

    dl_pdr->precedence = OGS_PFCP_DEFAULT_PDR_PRECEDENCE;
    ul_pdr->precedence = OGS_PFCP_DEFAULT_PDR_PRECEDENCE;

    cp2up_pdr->precedence = OGS_PFCP_CP2UP_PDR_PRECEDENCE;
    up2cp_pdr->precedence = OGS_PFCP_UP2CP_PDR_PRECEDENCE;

    ogs_assert(OGS_OK ==
        smf_5gc_pfcp_send_session_establishment_request(sess, stream));

    return true;

cleanup:
    ogs_assert(strerror);

    ogs_error("%s", strerror);
    ogs_assert(true ==
        ogs_sbi_server_send_error(stream, OGS_SBI_HTTP_STATUS_BAD_REQUEST,
            recvmsg, strerror, NULL));
    ogs_free(strerror);

    return false;
}

bool smf_npcf_smpolicycontrol_handle_delete(
        smf_sess_t *sess, ogs_sbi_stream_t *stream, int state,
        ogs_sbi_message_t *recvmsg)
{
    int trigger = state;

    ogs_assert(trigger);

    ogs_assert(OGS_OK ==
        smf_5gc_pfcp_send_session_deletion_request(sess, stream, trigger));

    return true;
}
