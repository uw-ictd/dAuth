
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "vsmf_update_data.h"

OpenAPI_vsmf_update_data_t *OpenAPI_vsmf_update_data_create(
    OpenAPI_request_indication_e request_indication,
    OpenAPI_ambr_t *session_ambr,
    OpenAPI_list_t *qos_flows_add_mod_request_list,
    OpenAPI_list_t *qos_flows_rel_request_list,
    OpenAPI_list_t *eps_bearer_info,
    OpenAPI_list_t *assign_ebi_list,
    OpenAPI_list_t *revoke_ebi_list,
    OpenAPI_list_t *modified_ebi_list,
    bool is_pti,
    int pti,
    OpenAPI_ref_to_binary_data_t *n1_sm_info_to_ue,
    bool is_always_on_granted,
    int always_on_granted,
    char *hsmf_pdu_session_uri,
    char *supported_features,
    OpenAPI_cause_e cause,
    char *n1sm_cause,
    bool is_back_off_timer,
    int back_off_timer,
    OpenAPI_ma_release_indication_e ma_release_ind,
    bool is_ma_accepted_ind,
    int ma_accepted_ind,
    OpenAPI_tunnel_info_t *additional_cn_tunnel_info,
    OpenAPI_list_t *dnai_list,
    OpenAPI_n4_information_t *n4_info,
    OpenAPI_n4_information_t *n4_info_ext1,
    OpenAPI_n4_information_t *n4_info_ext2,
    bool is_small_data_rate_control_enabled,
    int small_data_rate_control_enabled,
    OpenAPI_qos_monitoring_info_t *qos_monitoring_info
)
{
    OpenAPI_vsmf_update_data_t *vsmf_update_data_local_var = ogs_malloc(sizeof(OpenAPI_vsmf_update_data_t));
    ogs_assert(vsmf_update_data_local_var);

    vsmf_update_data_local_var->request_indication = request_indication;
    vsmf_update_data_local_var->session_ambr = session_ambr;
    vsmf_update_data_local_var->qos_flows_add_mod_request_list = qos_flows_add_mod_request_list;
    vsmf_update_data_local_var->qos_flows_rel_request_list = qos_flows_rel_request_list;
    vsmf_update_data_local_var->eps_bearer_info = eps_bearer_info;
    vsmf_update_data_local_var->assign_ebi_list = assign_ebi_list;
    vsmf_update_data_local_var->revoke_ebi_list = revoke_ebi_list;
    vsmf_update_data_local_var->modified_ebi_list = modified_ebi_list;
    vsmf_update_data_local_var->is_pti = is_pti;
    vsmf_update_data_local_var->pti = pti;
    vsmf_update_data_local_var->n1_sm_info_to_ue = n1_sm_info_to_ue;
    vsmf_update_data_local_var->is_always_on_granted = is_always_on_granted;
    vsmf_update_data_local_var->always_on_granted = always_on_granted;
    vsmf_update_data_local_var->hsmf_pdu_session_uri = hsmf_pdu_session_uri;
    vsmf_update_data_local_var->supported_features = supported_features;
    vsmf_update_data_local_var->cause = cause;
    vsmf_update_data_local_var->n1sm_cause = n1sm_cause;
    vsmf_update_data_local_var->is_back_off_timer = is_back_off_timer;
    vsmf_update_data_local_var->back_off_timer = back_off_timer;
    vsmf_update_data_local_var->ma_release_ind = ma_release_ind;
    vsmf_update_data_local_var->is_ma_accepted_ind = is_ma_accepted_ind;
    vsmf_update_data_local_var->ma_accepted_ind = ma_accepted_ind;
    vsmf_update_data_local_var->additional_cn_tunnel_info = additional_cn_tunnel_info;
    vsmf_update_data_local_var->dnai_list = dnai_list;
    vsmf_update_data_local_var->n4_info = n4_info;
    vsmf_update_data_local_var->n4_info_ext1 = n4_info_ext1;
    vsmf_update_data_local_var->n4_info_ext2 = n4_info_ext2;
    vsmf_update_data_local_var->is_small_data_rate_control_enabled = is_small_data_rate_control_enabled;
    vsmf_update_data_local_var->small_data_rate_control_enabled = small_data_rate_control_enabled;
    vsmf_update_data_local_var->qos_monitoring_info = qos_monitoring_info;

    return vsmf_update_data_local_var;
}

void OpenAPI_vsmf_update_data_free(OpenAPI_vsmf_update_data_t *vsmf_update_data)
{
    if (NULL == vsmf_update_data) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_ambr_free(vsmf_update_data->session_ambr);
    OpenAPI_list_for_each(vsmf_update_data->qos_flows_add_mod_request_list, node) {
        OpenAPI_qos_flow_add_modify_request_item_free(node->data);
    }
    OpenAPI_list_free(vsmf_update_data->qos_flows_add_mod_request_list);
    OpenAPI_list_for_each(vsmf_update_data->qos_flows_rel_request_list, node) {
        OpenAPI_qos_flow_release_request_item_free(node->data);
    }
    OpenAPI_list_free(vsmf_update_data->qos_flows_rel_request_list);
    OpenAPI_list_for_each(vsmf_update_data->eps_bearer_info, node) {
        OpenAPI_eps_bearer_info_free(node->data);
    }
    OpenAPI_list_free(vsmf_update_data->eps_bearer_info);
    OpenAPI_list_for_each(vsmf_update_data->assign_ebi_list, node) {
        OpenAPI_arp_free(node->data);
    }
    OpenAPI_list_free(vsmf_update_data->assign_ebi_list);
    OpenAPI_list_for_each(vsmf_update_data->revoke_ebi_list, node) {
        ogs_free(node->data);
    }
    OpenAPI_list_free(vsmf_update_data->revoke_ebi_list);
    OpenAPI_list_for_each(vsmf_update_data->modified_ebi_list, node) {
        OpenAPI_ebi_arp_mapping_free(node->data);
    }
    OpenAPI_list_free(vsmf_update_data->modified_ebi_list);
    OpenAPI_ref_to_binary_data_free(vsmf_update_data->n1_sm_info_to_ue);
    ogs_free(vsmf_update_data->hsmf_pdu_session_uri);
    ogs_free(vsmf_update_data->supported_features);
    ogs_free(vsmf_update_data->n1sm_cause);
    OpenAPI_tunnel_info_free(vsmf_update_data->additional_cn_tunnel_info);
    OpenAPI_list_for_each(vsmf_update_data->dnai_list, node) {
        ogs_free(node->data);
    }
    OpenAPI_list_free(vsmf_update_data->dnai_list);
    OpenAPI_n4_information_free(vsmf_update_data->n4_info);
    OpenAPI_n4_information_free(vsmf_update_data->n4_info_ext1);
    OpenAPI_n4_information_free(vsmf_update_data->n4_info_ext2);
    OpenAPI_qos_monitoring_info_free(vsmf_update_data->qos_monitoring_info);
    ogs_free(vsmf_update_data);
}

cJSON *OpenAPI_vsmf_update_data_convertToJSON(OpenAPI_vsmf_update_data_t *vsmf_update_data)
{
    cJSON *item = NULL;

    if (vsmf_update_data == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [VsmfUpdateData]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (cJSON_AddStringToObject(item, "requestIndication", OpenAPI_request_indication_ToString(vsmf_update_data->request_indication)) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [request_indication]");
        goto end;
    }

    if (vsmf_update_data->session_ambr) {
    cJSON *session_ambr_local_JSON = OpenAPI_ambr_convertToJSON(vsmf_update_data->session_ambr);
    if (session_ambr_local_JSON == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [session_ambr]");
        goto end;
    }
    cJSON_AddItemToObject(item, "sessionAmbr", session_ambr_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [session_ambr]");
        goto end;
    }
    }

    if (vsmf_update_data->qos_flows_add_mod_request_list) {
    cJSON *qos_flows_add_mod_request_listList = cJSON_AddArrayToObject(item, "qosFlowsAddModRequestList");
    if (qos_flows_add_mod_request_listList == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [qos_flows_add_mod_request_list]");
        goto end;
    }

    OpenAPI_lnode_t *qos_flows_add_mod_request_list_node;
    if (vsmf_update_data->qos_flows_add_mod_request_list) {
        OpenAPI_list_for_each(vsmf_update_data->qos_flows_add_mod_request_list, qos_flows_add_mod_request_list_node) {
            cJSON *itemLocal = OpenAPI_qos_flow_add_modify_request_item_convertToJSON(qos_flows_add_mod_request_list_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [qos_flows_add_mod_request_list]");
                goto end;
            }
            cJSON_AddItemToArray(qos_flows_add_mod_request_listList, itemLocal);
        }
    }
    }

    if (vsmf_update_data->qos_flows_rel_request_list) {
    cJSON *qos_flows_rel_request_listList = cJSON_AddArrayToObject(item, "qosFlowsRelRequestList");
    if (qos_flows_rel_request_listList == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [qos_flows_rel_request_list]");
        goto end;
    }

    OpenAPI_lnode_t *qos_flows_rel_request_list_node;
    if (vsmf_update_data->qos_flows_rel_request_list) {
        OpenAPI_list_for_each(vsmf_update_data->qos_flows_rel_request_list, qos_flows_rel_request_list_node) {
            cJSON *itemLocal = OpenAPI_qos_flow_release_request_item_convertToJSON(qos_flows_rel_request_list_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [qos_flows_rel_request_list]");
                goto end;
            }
            cJSON_AddItemToArray(qos_flows_rel_request_listList, itemLocal);
        }
    }
    }

    if (vsmf_update_data->eps_bearer_info) {
    cJSON *eps_bearer_infoList = cJSON_AddArrayToObject(item, "epsBearerInfo");
    if (eps_bearer_infoList == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [eps_bearer_info]");
        goto end;
    }

    OpenAPI_lnode_t *eps_bearer_info_node;
    if (vsmf_update_data->eps_bearer_info) {
        OpenAPI_list_for_each(vsmf_update_data->eps_bearer_info, eps_bearer_info_node) {
            cJSON *itemLocal = OpenAPI_eps_bearer_info_convertToJSON(eps_bearer_info_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [eps_bearer_info]");
                goto end;
            }
            cJSON_AddItemToArray(eps_bearer_infoList, itemLocal);
        }
    }
    }

    if (vsmf_update_data->assign_ebi_list) {
    cJSON *assign_ebi_listList = cJSON_AddArrayToObject(item, "assignEbiList");
    if (assign_ebi_listList == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [assign_ebi_list]");
        goto end;
    }

    OpenAPI_lnode_t *assign_ebi_list_node;
    if (vsmf_update_data->assign_ebi_list) {
        OpenAPI_list_for_each(vsmf_update_data->assign_ebi_list, assign_ebi_list_node) {
            cJSON *itemLocal = OpenAPI_arp_convertToJSON(assign_ebi_list_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [assign_ebi_list]");
                goto end;
            }
            cJSON_AddItemToArray(assign_ebi_listList, itemLocal);
        }
    }
    }

    if (vsmf_update_data->revoke_ebi_list) {
    cJSON *revoke_ebi_list = cJSON_AddArrayToObject(item, "revokeEbiList");
    if (revoke_ebi_list == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [revoke_ebi_list]");
        goto end;
    }

    OpenAPI_lnode_t *revoke_ebi_list_node;
    OpenAPI_list_for_each(vsmf_update_data->revoke_ebi_list, revoke_ebi_list_node)  {
    if (cJSON_AddNumberToObject(revoke_ebi_list, "", *(double *)revoke_ebi_list_node->data) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [revoke_ebi_list]");
        goto end;
    }
                    }
    }

    if (vsmf_update_data->modified_ebi_list) {
    cJSON *modified_ebi_listList = cJSON_AddArrayToObject(item, "modifiedEbiList");
    if (modified_ebi_listList == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [modified_ebi_list]");
        goto end;
    }

    OpenAPI_lnode_t *modified_ebi_list_node;
    if (vsmf_update_data->modified_ebi_list) {
        OpenAPI_list_for_each(vsmf_update_data->modified_ebi_list, modified_ebi_list_node) {
            cJSON *itemLocal = OpenAPI_ebi_arp_mapping_convertToJSON(modified_ebi_list_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [modified_ebi_list]");
                goto end;
            }
            cJSON_AddItemToArray(modified_ebi_listList, itemLocal);
        }
    }
    }

    if (vsmf_update_data->is_pti) {
    if (cJSON_AddNumberToObject(item, "pti", vsmf_update_data->pti) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [pti]");
        goto end;
    }
    }

    if (vsmf_update_data->n1_sm_info_to_ue) {
    cJSON *n1_sm_info_to_ue_local_JSON = OpenAPI_ref_to_binary_data_convertToJSON(vsmf_update_data->n1_sm_info_to_ue);
    if (n1_sm_info_to_ue_local_JSON == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [n1_sm_info_to_ue]");
        goto end;
    }
    cJSON_AddItemToObject(item, "n1SmInfoToUe", n1_sm_info_to_ue_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [n1_sm_info_to_ue]");
        goto end;
    }
    }

    if (vsmf_update_data->is_always_on_granted) {
    if (cJSON_AddBoolToObject(item, "alwaysOnGranted", vsmf_update_data->always_on_granted) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [always_on_granted]");
        goto end;
    }
    }

    if (vsmf_update_data->hsmf_pdu_session_uri) {
    if (cJSON_AddStringToObject(item, "hsmfPduSessionUri", vsmf_update_data->hsmf_pdu_session_uri) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [hsmf_pdu_session_uri]");
        goto end;
    }
    }

    if (vsmf_update_data->supported_features) {
    if (cJSON_AddStringToObject(item, "supportedFeatures", vsmf_update_data->supported_features) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [supported_features]");
        goto end;
    }
    }

    if (vsmf_update_data->cause) {
    if (cJSON_AddStringToObject(item, "cause", OpenAPI_cause_ToString(vsmf_update_data->cause)) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [cause]");
        goto end;
    }
    }

    if (vsmf_update_data->n1sm_cause) {
    if (cJSON_AddStringToObject(item, "n1smCause", vsmf_update_data->n1sm_cause) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [n1sm_cause]");
        goto end;
    }
    }

    if (vsmf_update_data->is_back_off_timer) {
    if (cJSON_AddNumberToObject(item, "backOffTimer", vsmf_update_data->back_off_timer) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [back_off_timer]");
        goto end;
    }
    }

    if (vsmf_update_data->ma_release_ind) {
    if (cJSON_AddStringToObject(item, "maReleaseInd", OpenAPI_ma_release_indication_ToString(vsmf_update_data->ma_release_ind)) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [ma_release_ind]");
        goto end;
    }
    }

    if (vsmf_update_data->is_ma_accepted_ind) {
    if (cJSON_AddBoolToObject(item, "maAcceptedInd", vsmf_update_data->ma_accepted_ind) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [ma_accepted_ind]");
        goto end;
    }
    }

    if (vsmf_update_data->additional_cn_tunnel_info) {
    cJSON *additional_cn_tunnel_info_local_JSON = OpenAPI_tunnel_info_convertToJSON(vsmf_update_data->additional_cn_tunnel_info);
    if (additional_cn_tunnel_info_local_JSON == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [additional_cn_tunnel_info]");
        goto end;
    }
    cJSON_AddItemToObject(item, "additionalCnTunnelInfo", additional_cn_tunnel_info_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [additional_cn_tunnel_info]");
        goto end;
    }
    }

    if (vsmf_update_data->dnai_list) {
    cJSON *dnai_list = cJSON_AddArrayToObject(item, "dnaiList");
    if (dnai_list == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [dnai_list]");
        goto end;
    }

    OpenAPI_lnode_t *dnai_list_node;
    OpenAPI_list_for_each(vsmf_update_data->dnai_list, dnai_list_node)  {
    if (cJSON_AddStringToObject(dnai_list, "", (char*)dnai_list_node->data) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [dnai_list]");
        goto end;
    }
                    }
    }

    if (vsmf_update_data->n4_info) {
    cJSON *n4_info_local_JSON = OpenAPI_n4_information_convertToJSON(vsmf_update_data->n4_info);
    if (n4_info_local_JSON == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [n4_info]");
        goto end;
    }
    cJSON_AddItemToObject(item, "n4Info", n4_info_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [n4_info]");
        goto end;
    }
    }

    if (vsmf_update_data->n4_info_ext1) {
    cJSON *n4_info_ext1_local_JSON = OpenAPI_n4_information_convertToJSON(vsmf_update_data->n4_info_ext1);
    if (n4_info_ext1_local_JSON == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [n4_info_ext1]");
        goto end;
    }
    cJSON_AddItemToObject(item, "n4InfoExt1", n4_info_ext1_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [n4_info_ext1]");
        goto end;
    }
    }

    if (vsmf_update_data->n4_info_ext2) {
    cJSON *n4_info_ext2_local_JSON = OpenAPI_n4_information_convertToJSON(vsmf_update_data->n4_info_ext2);
    if (n4_info_ext2_local_JSON == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [n4_info_ext2]");
        goto end;
    }
    cJSON_AddItemToObject(item, "n4InfoExt2", n4_info_ext2_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [n4_info_ext2]");
        goto end;
    }
    }

    if (vsmf_update_data->is_small_data_rate_control_enabled) {
    if (cJSON_AddBoolToObject(item, "smallDataRateControlEnabled", vsmf_update_data->small_data_rate_control_enabled) == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [small_data_rate_control_enabled]");
        goto end;
    }
    }

    if (vsmf_update_data->qos_monitoring_info) {
    cJSON *qos_monitoring_info_local_JSON = OpenAPI_qos_monitoring_info_convertToJSON(vsmf_update_data->qos_monitoring_info);
    if (qos_monitoring_info_local_JSON == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [qos_monitoring_info]");
        goto end;
    }
    cJSON_AddItemToObject(item, "qosMonitoringInfo", qos_monitoring_info_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed [qos_monitoring_info]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_vsmf_update_data_t *OpenAPI_vsmf_update_data_parseFromJSON(cJSON *vsmf_update_dataJSON)
{
    OpenAPI_vsmf_update_data_t *vsmf_update_data_local_var = NULL;
    cJSON *request_indication = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "requestIndication");
    if (!request_indication) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [request_indication]");
        goto end;
    }

    OpenAPI_request_indication_e request_indicationVariable;
    if (!cJSON_IsString(request_indication)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [request_indication]");
        goto end;
    }
    request_indicationVariable = OpenAPI_request_indication_FromString(request_indication->valuestring);

    cJSON *session_ambr = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "sessionAmbr");

    OpenAPI_ambr_t *session_ambr_local_nonprim = NULL;
    if (session_ambr) {
    session_ambr_local_nonprim = OpenAPI_ambr_parseFromJSON(session_ambr);
    }

    cJSON *qos_flows_add_mod_request_list = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "qosFlowsAddModRequestList");

    OpenAPI_list_t *qos_flows_add_mod_request_listList;
    if (qos_flows_add_mod_request_list) {
    cJSON *qos_flows_add_mod_request_list_local_nonprimitive;
    if (!cJSON_IsArray(qos_flows_add_mod_request_list)){
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [qos_flows_add_mod_request_list]");
        goto end;
    }

    qos_flows_add_mod_request_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(qos_flows_add_mod_request_list_local_nonprimitive, qos_flows_add_mod_request_list ) {
        if (!cJSON_IsObject(qos_flows_add_mod_request_list_local_nonprimitive)) {
            ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [qos_flows_add_mod_request_list]");
            goto end;
        }
        OpenAPI_qos_flow_add_modify_request_item_t *qos_flows_add_mod_request_listItem = OpenAPI_qos_flow_add_modify_request_item_parseFromJSON(qos_flows_add_mod_request_list_local_nonprimitive);

        if (!qos_flows_add_mod_request_listItem) {
            ogs_error("No qos_flows_add_mod_request_listItem");
            OpenAPI_list_free(qos_flows_add_mod_request_listList);
            goto end;
        }

        OpenAPI_list_add(qos_flows_add_mod_request_listList, qos_flows_add_mod_request_listItem);
    }
    }

    cJSON *qos_flows_rel_request_list = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "qosFlowsRelRequestList");

    OpenAPI_list_t *qos_flows_rel_request_listList;
    if (qos_flows_rel_request_list) {
    cJSON *qos_flows_rel_request_list_local_nonprimitive;
    if (!cJSON_IsArray(qos_flows_rel_request_list)){
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [qos_flows_rel_request_list]");
        goto end;
    }

    qos_flows_rel_request_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(qos_flows_rel_request_list_local_nonprimitive, qos_flows_rel_request_list ) {
        if (!cJSON_IsObject(qos_flows_rel_request_list_local_nonprimitive)) {
            ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [qos_flows_rel_request_list]");
            goto end;
        }
        OpenAPI_qos_flow_release_request_item_t *qos_flows_rel_request_listItem = OpenAPI_qos_flow_release_request_item_parseFromJSON(qos_flows_rel_request_list_local_nonprimitive);

        if (!qos_flows_rel_request_listItem) {
            ogs_error("No qos_flows_rel_request_listItem");
            OpenAPI_list_free(qos_flows_rel_request_listList);
            goto end;
        }

        OpenAPI_list_add(qos_flows_rel_request_listList, qos_flows_rel_request_listItem);
    }
    }

    cJSON *eps_bearer_info = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "epsBearerInfo");

    OpenAPI_list_t *eps_bearer_infoList;
    if (eps_bearer_info) {
    cJSON *eps_bearer_info_local_nonprimitive;
    if (!cJSON_IsArray(eps_bearer_info)){
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [eps_bearer_info]");
        goto end;
    }

    eps_bearer_infoList = OpenAPI_list_create();

    cJSON_ArrayForEach(eps_bearer_info_local_nonprimitive, eps_bearer_info ) {
        if (!cJSON_IsObject(eps_bearer_info_local_nonprimitive)) {
            ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [eps_bearer_info]");
            goto end;
        }
        OpenAPI_eps_bearer_info_t *eps_bearer_infoItem = OpenAPI_eps_bearer_info_parseFromJSON(eps_bearer_info_local_nonprimitive);

        if (!eps_bearer_infoItem) {
            ogs_error("No eps_bearer_infoItem");
            OpenAPI_list_free(eps_bearer_infoList);
            goto end;
        }

        OpenAPI_list_add(eps_bearer_infoList, eps_bearer_infoItem);
    }
    }

    cJSON *assign_ebi_list = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "assignEbiList");

    OpenAPI_list_t *assign_ebi_listList;
    if (assign_ebi_list) {
    cJSON *assign_ebi_list_local_nonprimitive;
    if (!cJSON_IsArray(assign_ebi_list)){
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [assign_ebi_list]");
        goto end;
    }

    assign_ebi_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(assign_ebi_list_local_nonprimitive, assign_ebi_list ) {
        if (!cJSON_IsObject(assign_ebi_list_local_nonprimitive)) {
            ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [assign_ebi_list]");
            goto end;
        }
        OpenAPI_arp_t *assign_ebi_listItem = OpenAPI_arp_parseFromJSON(assign_ebi_list_local_nonprimitive);

        if (!assign_ebi_listItem) {
            ogs_error("No assign_ebi_listItem");
            OpenAPI_list_free(assign_ebi_listList);
            goto end;
        }

        OpenAPI_list_add(assign_ebi_listList, assign_ebi_listItem);
    }
    }

    cJSON *revoke_ebi_list = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "revokeEbiList");

    OpenAPI_list_t *revoke_ebi_listList;
    if (revoke_ebi_list) {
    cJSON *revoke_ebi_list_local;
    if (!cJSON_IsArray(revoke_ebi_list)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [revoke_ebi_list]");
        goto end;
    }
    revoke_ebi_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(revoke_ebi_list_local, revoke_ebi_list) {
    if (!cJSON_IsNumber(revoke_ebi_list_local)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [revoke_ebi_list]");
        goto end;
    }
    OpenAPI_list_add(revoke_ebi_listList , &revoke_ebi_list_local->valuedouble);
    }
    }

    cJSON *modified_ebi_list = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "modifiedEbiList");

    OpenAPI_list_t *modified_ebi_listList;
    if (modified_ebi_list) {
    cJSON *modified_ebi_list_local_nonprimitive;
    if (!cJSON_IsArray(modified_ebi_list)){
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [modified_ebi_list]");
        goto end;
    }

    modified_ebi_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(modified_ebi_list_local_nonprimitive, modified_ebi_list ) {
        if (!cJSON_IsObject(modified_ebi_list_local_nonprimitive)) {
            ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [modified_ebi_list]");
            goto end;
        }
        OpenAPI_ebi_arp_mapping_t *modified_ebi_listItem = OpenAPI_ebi_arp_mapping_parseFromJSON(modified_ebi_list_local_nonprimitive);

        if (!modified_ebi_listItem) {
            ogs_error("No modified_ebi_listItem");
            OpenAPI_list_free(modified_ebi_listList);
            goto end;
        }

        OpenAPI_list_add(modified_ebi_listList, modified_ebi_listItem);
    }
    }

    cJSON *pti = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "pti");

    if (pti) {
    if (!cJSON_IsNumber(pti)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [pti]");
        goto end;
    }
    }

    cJSON *n1_sm_info_to_ue = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "n1SmInfoToUe");

    OpenAPI_ref_to_binary_data_t *n1_sm_info_to_ue_local_nonprim = NULL;
    if (n1_sm_info_to_ue) {
    n1_sm_info_to_ue_local_nonprim = OpenAPI_ref_to_binary_data_parseFromJSON(n1_sm_info_to_ue);
    }

    cJSON *always_on_granted = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "alwaysOnGranted");

    if (always_on_granted) {
    if (!cJSON_IsBool(always_on_granted)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [always_on_granted]");
        goto end;
    }
    }

    cJSON *hsmf_pdu_session_uri = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "hsmfPduSessionUri");

    if (hsmf_pdu_session_uri) {
    if (!cJSON_IsString(hsmf_pdu_session_uri)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [hsmf_pdu_session_uri]");
        goto end;
    }
    }

    cJSON *supported_features = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "supportedFeatures");

    if (supported_features) {
    if (!cJSON_IsString(supported_features)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [supported_features]");
        goto end;
    }
    }

    cJSON *cause = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "cause");

    OpenAPI_cause_e causeVariable;
    if (cause) {
    if (!cJSON_IsString(cause)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [cause]");
        goto end;
    }
    causeVariable = OpenAPI_cause_FromString(cause->valuestring);
    }

    cJSON *n1sm_cause = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "n1smCause");

    if (n1sm_cause) {
    if (!cJSON_IsString(n1sm_cause)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [n1sm_cause]");
        goto end;
    }
    }

    cJSON *back_off_timer = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "backOffTimer");

    if (back_off_timer) {
    if (!cJSON_IsNumber(back_off_timer)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [back_off_timer]");
        goto end;
    }
    }

    cJSON *ma_release_ind = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "maReleaseInd");

    OpenAPI_ma_release_indication_e ma_release_indVariable;
    if (ma_release_ind) {
    if (!cJSON_IsString(ma_release_ind)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [ma_release_ind]");
        goto end;
    }
    ma_release_indVariable = OpenAPI_ma_release_indication_FromString(ma_release_ind->valuestring);
    }

    cJSON *ma_accepted_ind = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "maAcceptedInd");

    if (ma_accepted_ind) {
    if (!cJSON_IsBool(ma_accepted_ind)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [ma_accepted_ind]");
        goto end;
    }
    }

    cJSON *additional_cn_tunnel_info = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "additionalCnTunnelInfo");

    OpenAPI_tunnel_info_t *additional_cn_tunnel_info_local_nonprim = NULL;
    if (additional_cn_tunnel_info) {
    additional_cn_tunnel_info_local_nonprim = OpenAPI_tunnel_info_parseFromJSON(additional_cn_tunnel_info);
    }

    cJSON *dnai_list = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "dnaiList");

    OpenAPI_list_t *dnai_listList;
    if (dnai_list) {
    cJSON *dnai_list_local;
    if (!cJSON_IsArray(dnai_list)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [dnai_list]");
        goto end;
    }
    dnai_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(dnai_list_local, dnai_list) {
    if (!cJSON_IsString(dnai_list_local)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [dnai_list]");
        goto end;
    }
    OpenAPI_list_add(dnai_listList , ogs_strdup(dnai_list_local->valuestring));
    }
    }

    cJSON *n4_info = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "n4Info");

    OpenAPI_n4_information_t *n4_info_local_nonprim = NULL;
    if (n4_info) {
    n4_info_local_nonprim = OpenAPI_n4_information_parseFromJSON(n4_info);
    }

    cJSON *n4_info_ext1 = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "n4InfoExt1");

    OpenAPI_n4_information_t *n4_info_ext1_local_nonprim = NULL;
    if (n4_info_ext1) {
    n4_info_ext1_local_nonprim = OpenAPI_n4_information_parseFromJSON(n4_info_ext1);
    }

    cJSON *n4_info_ext2 = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "n4InfoExt2");

    OpenAPI_n4_information_t *n4_info_ext2_local_nonprim = NULL;
    if (n4_info_ext2) {
    n4_info_ext2_local_nonprim = OpenAPI_n4_information_parseFromJSON(n4_info_ext2);
    }

    cJSON *small_data_rate_control_enabled = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "smallDataRateControlEnabled");

    if (small_data_rate_control_enabled) {
    if (!cJSON_IsBool(small_data_rate_control_enabled)) {
        ogs_error("OpenAPI_vsmf_update_data_parseFromJSON() failed [small_data_rate_control_enabled]");
        goto end;
    }
    }

    cJSON *qos_monitoring_info = cJSON_GetObjectItemCaseSensitive(vsmf_update_dataJSON, "qosMonitoringInfo");

    OpenAPI_qos_monitoring_info_t *qos_monitoring_info_local_nonprim = NULL;
    if (qos_monitoring_info) {
    qos_monitoring_info_local_nonprim = OpenAPI_qos_monitoring_info_parseFromJSON(qos_monitoring_info);
    }

    vsmf_update_data_local_var = OpenAPI_vsmf_update_data_create (
        request_indicationVariable,
        session_ambr ? session_ambr_local_nonprim : NULL,
        qos_flows_add_mod_request_list ? qos_flows_add_mod_request_listList : NULL,
        qos_flows_rel_request_list ? qos_flows_rel_request_listList : NULL,
        eps_bearer_info ? eps_bearer_infoList : NULL,
        assign_ebi_list ? assign_ebi_listList : NULL,
        revoke_ebi_list ? revoke_ebi_listList : NULL,
        modified_ebi_list ? modified_ebi_listList : NULL,
        pti ? true : false,
        pti ? pti->valuedouble : 0,
        n1_sm_info_to_ue ? n1_sm_info_to_ue_local_nonprim : NULL,
        always_on_granted ? true : false,
        always_on_granted ? always_on_granted->valueint : 0,
        hsmf_pdu_session_uri ? ogs_strdup(hsmf_pdu_session_uri->valuestring) : NULL,
        supported_features ? ogs_strdup(supported_features->valuestring) : NULL,
        cause ? causeVariable : 0,
        n1sm_cause ? ogs_strdup(n1sm_cause->valuestring) : NULL,
        back_off_timer ? true : false,
        back_off_timer ? back_off_timer->valuedouble : 0,
        ma_release_ind ? ma_release_indVariable : 0,
        ma_accepted_ind ? true : false,
        ma_accepted_ind ? ma_accepted_ind->valueint : 0,
        additional_cn_tunnel_info ? additional_cn_tunnel_info_local_nonprim : NULL,
        dnai_list ? dnai_listList : NULL,
        n4_info ? n4_info_local_nonprim : NULL,
        n4_info_ext1 ? n4_info_ext1_local_nonprim : NULL,
        n4_info_ext2 ? n4_info_ext2_local_nonprim : NULL,
        small_data_rate_control_enabled ? true : false,
        small_data_rate_control_enabled ? small_data_rate_control_enabled->valueint : 0,
        qos_monitoring_info ? qos_monitoring_info_local_nonprim : NULL
    );

    return vsmf_update_data_local_var;
end:
    return NULL;
}

OpenAPI_vsmf_update_data_t *OpenAPI_vsmf_update_data_copy(OpenAPI_vsmf_update_data_t *dst, OpenAPI_vsmf_update_data_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_vsmf_update_data_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_vsmf_update_data_convertToJSON() failed");
        return NULL;
    }

    content = cJSON_Print(item);
    cJSON_Delete(item);

    if (!content) {
        ogs_error("cJSON_Print() failed");
        return NULL;
    }

    item = cJSON_Parse(content);
    ogs_free(content);
    if (!item) {
        ogs_error("cJSON_Parse() failed");
        return NULL;
    }

    OpenAPI_vsmf_update_data_free(dst);
    dst = OpenAPI_vsmf_update_data_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

