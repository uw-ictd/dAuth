
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "pdu_session_created_data.h"

OpenAPI_pdu_session_created_data_t *OpenAPI_pdu_session_created_data_create(
    OpenAPI_pdu_session_type_e pdu_session_type,
    char *ssc_mode,
    OpenAPI_tunnel_info_t *hcn_tunnel_info,
    OpenAPI_tunnel_info_t *cn_tunnel_info,
    OpenAPI_tunnel_info_t *additional_cn_tunnel_info,
    OpenAPI_ambr_t *session_ambr,
    OpenAPI_list_t *qos_flows_setup_list,
    char *h_smf_instance_id,
    char *smf_instance_id,
    bool is_pdu_session_id,
    int pdu_session_id,
    OpenAPI_snssai_t *s_nssai,
    bool is_enable_pause_charging,
    int enable_pause_charging,
    char *ue_ipv4_address,
    char *ue_ipv6_prefix,
    OpenAPI_ref_to_binary_data_t *n1_sm_info_to_ue,
    OpenAPI_eps_pdn_cnx_info_t *eps_pdn_cnx_info,
    OpenAPI_list_t *eps_bearer_info,
    char *supported_features,
    OpenAPI_max_integrity_protected_data_rate_e max_integrity_protected_data_rate,
    OpenAPI_max_integrity_protected_data_rate_e max_integrity_protected_data_rate_dl,
    bool is_always_on_granted,
    int always_on_granted,
    char *gpsi,
    OpenAPI_up_security_t *up_security,
    OpenAPI_roaming_charging_profile_t *roaming_charging_profile,
    char *h_smf_service_instance_id,
    char *smf_service_instance_id,
    char *recovery_time,
    OpenAPI_list_t *dnai_list,
    bool is_ipv6_multi_homing_ind,
    int ipv6_multi_homing_ind,
    bool is_ma_accepted_ind,
    int ma_accepted_ind,
    char *home_provided_charging_id,
    bool is_nef_ext_buf_support_ind,
    int nef_ext_buf_support_ind,
    bool is_small_data_rate_control_enabled,
    int small_data_rate_control_enabled,
    char *ue_ipv6_interface_id,
    bool is_ipv6_index,
    int ipv6_index,
    OpenAPI_ip_address_t *dn_aaa_address,
    OpenAPI_redundant_pdu_session_information_t *redundant_pdu_session_info
)
{
    OpenAPI_pdu_session_created_data_t *pdu_session_created_data_local_var = ogs_malloc(sizeof(OpenAPI_pdu_session_created_data_t));
    ogs_assert(pdu_session_created_data_local_var);

    pdu_session_created_data_local_var->pdu_session_type = pdu_session_type;
    pdu_session_created_data_local_var->ssc_mode = ssc_mode;
    pdu_session_created_data_local_var->hcn_tunnel_info = hcn_tunnel_info;
    pdu_session_created_data_local_var->cn_tunnel_info = cn_tunnel_info;
    pdu_session_created_data_local_var->additional_cn_tunnel_info = additional_cn_tunnel_info;
    pdu_session_created_data_local_var->session_ambr = session_ambr;
    pdu_session_created_data_local_var->qos_flows_setup_list = qos_flows_setup_list;
    pdu_session_created_data_local_var->h_smf_instance_id = h_smf_instance_id;
    pdu_session_created_data_local_var->smf_instance_id = smf_instance_id;
    pdu_session_created_data_local_var->is_pdu_session_id = is_pdu_session_id;
    pdu_session_created_data_local_var->pdu_session_id = pdu_session_id;
    pdu_session_created_data_local_var->s_nssai = s_nssai;
    pdu_session_created_data_local_var->is_enable_pause_charging = is_enable_pause_charging;
    pdu_session_created_data_local_var->enable_pause_charging = enable_pause_charging;
    pdu_session_created_data_local_var->ue_ipv4_address = ue_ipv4_address;
    pdu_session_created_data_local_var->ue_ipv6_prefix = ue_ipv6_prefix;
    pdu_session_created_data_local_var->n1_sm_info_to_ue = n1_sm_info_to_ue;
    pdu_session_created_data_local_var->eps_pdn_cnx_info = eps_pdn_cnx_info;
    pdu_session_created_data_local_var->eps_bearer_info = eps_bearer_info;
    pdu_session_created_data_local_var->supported_features = supported_features;
    pdu_session_created_data_local_var->max_integrity_protected_data_rate = max_integrity_protected_data_rate;
    pdu_session_created_data_local_var->max_integrity_protected_data_rate_dl = max_integrity_protected_data_rate_dl;
    pdu_session_created_data_local_var->is_always_on_granted = is_always_on_granted;
    pdu_session_created_data_local_var->always_on_granted = always_on_granted;
    pdu_session_created_data_local_var->gpsi = gpsi;
    pdu_session_created_data_local_var->up_security = up_security;
    pdu_session_created_data_local_var->roaming_charging_profile = roaming_charging_profile;
    pdu_session_created_data_local_var->h_smf_service_instance_id = h_smf_service_instance_id;
    pdu_session_created_data_local_var->smf_service_instance_id = smf_service_instance_id;
    pdu_session_created_data_local_var->recovery_time = recovery_time;
    pdu_session_created_data_local_var->dnai_list = dnai_list;
    pdu_session_created_data_local_var->is_ipv6_multi_homing_ind = is_ipv6_multi_homing_ind;
    pdu_session_created_data_local_var->ipv6_multi_homing_ind = ipv6_multi_homing_ind;
    pdu_session_created_data_local_var->is_ma_accepted_ind = is_ma_accepted_ind;
    pdu_session_created_data_local_var->ma_accepted_ind = ma_accepted_ind;
    pdu_session_created_data_local_var->home_provided_charging_id = home_provided_charging_id;
    pdu_session_created_data_local_var->is_nef_ext_buf_support_ind = is_nef_ext_buf_support_ind;
    pdu_session_created_data_local_var->nef_ext_buf_support_ind = nef_ext_buf_support_ind;
    pdu_session_created_data_local_var->is_small_data_rate_control_enabled = is_small_data_rate_control_enabled;
    pdu_session_created_data_local_var->small_data_rate_control_enabled = small_data_rate_control_enabled;
    pdu_session_created_data_local_var->ue_ipv6_interface_id = ue_ipv6_interface_id;
    pdu_session_created_data_local_var->is_ipv6_index = is_ipv6_index;
    pdu_session_created_data_local_var->ipv6_index = ipv6_index;
    pdu_session_created_data_local_var->dn_aaa_address = dn_aaa_address;
    pdu_session_created_data_local_var->redundant_pdu_session_info = redundant_pdu_session_info;

    return pdu_session_created_data_local_var;
}

void OpenAPI_pdu_session_created_data_free(OpenAPI_pdu_session_created_data_t *pdu_session_created_data)
{
    if (NULL == pdu_session_created_data) {
        return;
    }
    OpenAPI_lnode_t *node;
    ogs_free(pdu_session_created_data->ssc_mode);
    OpenAPI_tunnel_info_free(pdu_session_created_data->hcn_tunnel_info);
    OpenAPI_tunnel_info_free(pdu_session_created_data->cn_tunnel_info);
    OpenAPI_tunnel_info_free(pdu_session_created_data->additional_cn_tunnel_info);
    OpenAPI_ambr_free(pdu_session_created_data->session_ambr);
    OpenAPI_list_for_each(pdu_session_created_data->qos_flows_setup_list, node) {
        OpenAPI_qos_flow_setup_item_free(node->data);
    }
    OpenAPI_list_free(pdu_session_created_data->qos_flows_setup_list);
    ogs_free(pdu_session_created_data->h_smf_instance_id);
    ogs_free(pdu_session_created_data->smf_instance_id);
    OpenAPI_snssai_free(pdu_session_created_data->s_nssai);
    ogs_free(pdu_session_created_data->ue_ipv4_address);
    ogs_free(pdu_session_created_data->ue_ipv6_prefix);
    OpenAPI_ref_to_binary_data_free(pdu_session_created_data->n1_sm_info_to_ue);
    OpenAPI_eps_pdn_cnx_info_free(pdu_session_created_data->eps_pdn_cnx_info);
    OpenAPI_list_for_each(pdu_session_created_data->eps_bearer_info, node) {
        OpenAPI_eps_bearer_info_free(node->data);
    }
    OpenAPI_list_free(pdu_session_created_data->eps_bearer_info);
    ogs_free(pdu_session_created_data->supported_features);
    ogs_free(pdu_session_created_data->gpsi);
    OpenAPI_up_security_free(pdu_session_created_data->up_security);
    OpenAPI_roaming_charging_profile_free(pdu_session_created_data->roaming_charging_profile);
    ogs_free(pdu_session_created_data->h_smf_service_instance_id);
    ogs_free(pdu_session_created_data->smf_service_instance_id);
    ogs_free(pdu_session_created_data->recovery_time);
    OpenAPI_list_for_each(pdu_session_created_data->dnai_list, node) {
        ogs_free(node->data);
    }
    OpenAPI_list_free(pdu_session_created_data->dnai_list);
    ogs_free(pdu_session_created_data->home_provided_charging_id);
    ogs_free(pdu_session_created_data->ue_ipv6_interface_id);
    OpenAPI_ip_address_free(pdu_session_created_data->dn_aaa_address);
    OpenAPI_redundant_pdu_session_information_free(pdu_session_created_data->redundant_pdu_session_info);
    ogs_free(pdu_session_created_data);
}

cJSON *OpenAPI_pdu_session_created_data_convertToJSON(OpenAPI_pdu_session_created_data_t *pdu_session_created_data)
{
    cJSON *item = NULL;

    if (pdu_session_created_data == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [PduSessionCreatedData]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (cJSON_AddStringToObject(item, "pduSessionType", OpenAPI_pdu_session_type_ToString(pdu_session_created_data->pdu_session_type)) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [pdu_session_type]");
        goto end;
    }

    if (cJSON_AddStringToObject(item, "sscMode", pdu_session_created_data->ssc_mode) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [ssc_mode]");
        goto end;
    }

    if (pdu_session_created_data->hcn_tunnel_info) {
    cJSON *hcn_tunnel_info_local_JSON = OpenAPI_tunnel_info_convertToJSON(pdu_session_created_data->hcn_tunnel_info);
    if (hcn_tunnel_info_local_JSON == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [hcn_tunnel_info]");
        goto end;
    }
    cJSON_AddItemToObject(item, "hcnTunnelInfo", hcn_tunnel_info_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [hcn_tunnel_info]");
        goto end;
    }
    }

    if (pdu_session_created_data->cn_tunnel_info) {
    cJSON *cn_tunnel_info_local_JSON = OpenAPI_tunnel_info_convertToJSON(pdu_session_created_data->cn_tunnel_info);
    if (cn_tunnel_info_local_JSON == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [cn_tunnel_info]");
        goto end;
    }
    cJSON_AddItemToObject(item, "cnTunnelInfo", cn_tunnel_info_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [cn_tunnel_info]");
        goto end;
    }
    }

    if (pdu_session_created_data->additional_cn_tunnel_info) {
    cJSON *additional_cn_tunnel_info_local_JSON = OpenAPI_tunnel_info_convertToJSON(pdu_session_created_data->additional_cn_tunnel_info);
    if (additional_cn_tunnel_info_local_JSON == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [additional_cn_tunnel_info]");
        goto end;
    }
    cJSON_AddItemToObject(item, "additionalCnTunnelInfo", additional_cn_tunnel_info_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [additional_cn_tunnel_info]");
        goto end;
    }
    }

    if (pdu_session_created_data->session_ambr) {
    cJSON *session_ambr_local_JSON = OpenAPI_ambr_convertToJSON(pdu_session_created_data->session_ambr);
    if (session_ambr_local_JSON == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [session_ambr]");
        goto end;
    }
    cJSON_AddItemToObject(item, "sessionAmbr", session_ambr_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [session_ambr]");
        goto end;
    }
    }

    if (pdu_session_created_data->qos_flows_setup_list) {
    cJSON *qos_flows_setup_listList = cJSON_AddArrayToObject(item, "qosFlowsSetupList");
    if (qos_flows_setup_listList == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [qos_flows_setup_list]");
        goto end;
    }

    OpenAPI_lnode_t *qos_flows_setup_list_node;
    if (pdu_session_created_data->qos_flows_setup_list) {
        OpenAPI_list_for_each(pdu_session_created_data->qos_flows_setup_list, qos_flows_setup_list_node) {
            cJSON *itemLocal = OpenAPI_qos_flow_setup_item_convertToJSON(qos_flows_setup_list_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [qos_flows_setup_list]");
                goto end;
            }
            cJSON_AddItemToArray(qos_flows_setup_listList, itemLocal);
        }
    }
    }

    if (pdu_session_created_data->h_smf_instance_id) {
    if (cJSON_AddStringToObject(item, "hSmfInstanceId", pdu_session_created_data->h_smf_instance_id) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [h_smf_instance_id]");
        goto end;
    }
    }

    if (pdu_session_created_data->smf_instance_id) {
    if (cJSON_AddStringToObject(item, "smfInstanceId", pdu_session_created_data->smf_instance_id) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [smf_instance_id]");
        goto end;
    }
    }

    if (pdu_session_created_data->is_pdu_session_id) {
    if (cJSON_AddNumberToObject(item, "pduSessionId", pdu_session_created_data->pdu_session_id) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [pdu_session_id]");
        goto end;
    }
    }

    if (pdu_session_created_data->s_nssai) {
    cJSON *s_nssai_local_JSON = OpenAPI_snssai_convertToJSON(pdu_session_created_data->s_nssai);
    if (s_nssai_local_JSON == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [s_nssai]");
        goto end;
    }
    cJSON_AddItemToObject(item, "sNssai", s_nssai_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [s_nssai]");
        goto end;
    }
    }

    if (pdu_session_created_data->is_enable_pause_charging) {
    if (cJSON_AddBoolToObject(item, "enablePauseCharging", pdu_session_created_data->enable_pause_charging) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [enable_pause_charging]");
        goto end;
    }
    }

    if (pdu_session_created_data->ue_ipv4_address) {
    if (cJSON_AddStringToObject(item, "ueIpv4Address", pdu_session_created_data->ue_ipv4_address) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [ue_ipv4_address]");
        goto end;
    }
    }

    if (pdu_session_created_data->ue_ipv6_prefix) {
    if (cJSON_AddStringToObject(item, "ueIpv6Prefix", pdu_session_created_data->ue_ipv6_prefix) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [ue_ipv6_prefix]");
        goto end;
    }
    }

    if (pdu_session_created_data->n1_sm_info_to_ue) {
    cJSON *n1_sm_info_to_ue_local_JSON = OpenAPI_ref_to_binary_data_convertToJSON(pdu_session_created_data->n1_sm_info_to_ue);
    if (n1_sm_info_to_ue_local_JSON == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [n1_sm_info_to_ue]");
        goto end;
    }
    cJSON_AddItemToObject(item, "n1SmInfoToUe", n1_sm_info_to_ue_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [n1_sm_info_to_ue]");
        goto end;
    }
    }

    if (pdu_session_created_data->eps_pdn_cnx_info) {
    cJSON *eps_pdn_cnx_info_local_JSON = OpenAPI_eps_pdn_cnx_info_convertToJSON(pdu_session_created_data->eps_pdn_cnx_info);
    if (eps_pdn_cnx_info_local_JSON == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [eps_pdn_cnx_info]");
        goto end;
    }
    cJSON_AddItemToObject(item, "epsPdnCnxInfo", eps_pdn_cnx_info_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [eps_pdn_cnx_info]");
        goto end;
    }
    }

    if (pdu_session_created_data->eps_bearer_info) {
    cJSON *eps_bearer_infoList = cJSON_AddArrayToObject(item, "epsBearerInfo");
    if (eps_bearer_infoList == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [eps_bearer_info]");
        goto end;
    }

    OpenAPI_lnode_t *eps_bearer_info_node;
    if (pdu_session_created_data->eps_bearer_info) {
        OpenAPI_list_for_each(pdu_session_created_data->eps_bearer_info, eps_bearer_info_node) {
            cJSON *itemLocal = OpenAPI_eps_bearer_info_convertToJSON(eps_bearer_info_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [eps_bearer_info]");
                goto end;
            }
            cJSON_AddItemToArray(eps_bearer_infoList, itemLocal);
        }
    }
    }

    if (pdu_session_created_data->supported_features) {
    if (cJSON_AddStringToObject(item, "supportedFeatures", pdu_session_created_data->supported_features) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [supported_features]");
        goto end;
    }
    }

    if (pdu_session_created_data->max_integrity_protected_data_rate) {
    if (cJSON_AddStringToObject(item, "maxIntegrityProtectedDataRate", OpenAPI_max_integrity_protected_data_rate_ToString(pdu_session_created_data->max_integrity_protected_data_rate)) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [max_integrity_protected_data_rate]");
        goto end;
    }
    }

    if (pdu_session_created_data->max_integrity_protected_data_rate_dl) {
    if (cJSON_AddStringToObject(item, "maxIntegrityProtectedDataRateDl", OpenAPI_max_integrity_protected_data_rate_ToString(pdu_session_created_data->max_integrity_protected_data_rate_dl)) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [max_integrity_protected_data_rate_dl]");
        goto end;
    }
    }

    if (pdu_session_created_data->is_always_on_granted) {
    if (cJSON_AddBoolToObject(item, "alwaysOnGranted", pdu_session_created_data->always_on_granted) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [always_on_granted]");
        goto end;
    }
    }

    if (pdu_session_created_data->gpsi) {
    if (cJSON_AddStringToObject(item, "gpsi", pdu_session_created_data->gpsi) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [gpsi]");
        goto end;
    }
    }

    if (pdu_session_created_data->up_security) {
    cJSON *up_security_local_JSON = OpenAPI_up_security_convertToJSON(pdu_session_created_data->up_security);
    if (up_security_local_JSON == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [up_security]");
        goto end;
    }
    cJSON_AddItemToObject(item, "upSecurity", up_security_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [up_security]");
        goto end;
    }
    }

    if (pdu_session_created_data->roaming_charging_profile) {
    cJSON *roaming_charging_profile_local_JSON = OpenAPI_roaming_charging_profile_convertToJSON(pdu_session_created_data->roaming_charging_profile);
    if (roaming_charging_profile_local_JSON == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [roaming_charging_profile]");
        goto end;
    }
    cJSON_AddItemToObject(item, "roamingChargingProfile", roaming_charging_profile_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [roaming_charging_profile]");
        goto end;
    }
    }

    if (pdu_session_created_data->h_smf_service_instance_id) {
    if (cJSON_AddStringToObject(item, "hSmfServiceInstanceId", pdu_session_created_data->h_smf_service_instance_id) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [h_smf_service_instance_id]");
        goto end;
    }
    }

    if (pdu_session_created_data->smf_service_instance_id) {
    if (cJSON_AddStringToObject(item, "smfServiceInstanceId", pdu_session_created_data->smf_service_instance_id) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [smf_service_instance_id]");
        goto end;
    }
    }

    if (pdu_session_created_data->recovery_time) {
    if (cJSON_AddStringToObject(item, "recoveryTime", pdu_session_created_data->recovery_time) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [recovery_time]");
        goto end;
    }
    }

    if (pdu_session_created_data->dnai_list) {
    cJSON *dnai_list = cJSON_AddArrayToObject(item, "dnaiList");
    if (dnai_list == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [dnai_list]");
        goto end;
    }

    OpenAPI_lnode_t *dnai_list_node;
    OpenAPI_list_for_each(pdu_session_created_data->dnai_list, dnai_list_node)  {
    if (cJSON_AddStringToObject(dnai_list, "", (char*)dnai_list_node->data) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [dnai_list]");
        goto end;
    }
                    }
    }

    if (pdu_session_created_data->is_ipv6_multi_homing_ind) {
    if (cJSON_AddBoolToObject(item, "ipv6MultiHomingInd", pdu_session_created_data->ipv6_multi_homing_ind) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [ipv6_multi_homing_ind]");
        goto end;
    }
    }

    if (pdu_session_created_data->is_ma_accepted_ind) {
    if (cJSON_AddBoolToObject(item, "maAcceptedInd", pdu_session_created_data->ma_accepted_ind) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [ma_accepted_ind]");
        goto end;
    }
    }

    if (pdu_session_created_data->home_provided_charging_id) {
    if (cJSON_AddStringToObject(item, "homeProvidedChargingId", pdu_session_created_data->home_provided_charging_id) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [home_provided_charging_id]");
        goto end;
    }
    }

    if (pdu_session_created_data->is_nef_ext_buf_support_ind) {
    if (cJSON_AddBoolToObject(item, "nefExtBufSupportInd", pdu_session_created_data->nef_ext_buf_support_ind) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [nef_ext_buf_support_ind]");
        goto end;
    }
    }

    if (pdu_session_created_data->is_small_data_rate_control_enabled) {
    if (cJSON_AddBoolToObject(item, "smallDataRateControlEnabled", pdu_session_created_data->small_data_rate_control_enabled) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [small_data_rate_control_enabled]");
        goto end;
    }
    }

    if (pdu_session_created_data->ue_ipv6_interface_id) {
    if (cJSON_AddStringToObject(item, "ueIpv6InterfaceId", pdu_session_created_data->ue_ipv6_interface_id) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [ue_ipv6_interface_id]");
        goto end;
    }
    }

    if (pdu_session_created_data->is_ipv6_index) {
    if (cJSON_AddNumberToObject(item, "ipv6Index", pdu_session_created_data->ipv6_index) == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [ipv6_index]");
        goto end;
    }
    }

    if (pdu_session_created_data->dn_aaa_address) {
    cJSON *dn_aaa_address_local_JSON = OpenAPI_ip_address_convertToJSON(pdu_session_created_data->dn_aaa_address);
    if (dn_aaa_address_local_JSON == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [dn_aaa_address]");
        goto end;
    }
    cJSON_AddItemToObject(item, "dnAaaAddress", dn_aaa_address_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [dn_aaa_address]");
        goto end;
    }
    }

    if (pdu_session_created_data->redundant_pdu_session_info) {
    cJSON *redundant_pdu_session_info_local_JSON = OpenAPI_redundant_pdu_session_information_convertToJSON(pdu_session_created_data->redundant_pdu_session_info);
    if (redundant_pdu_session_info_local_JSON == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [redundant_pdu_session_info]");
        goto end;
    }
    cJSON_AddItemToObject(item, "redundantPduSessionInfo", redundant_pdu_session_info_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed [redundant_pdu_session_info]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_pdu_session_created_data_t *OpenAPI_pdu_session_created_data_parseFromJSON(cJSON *pdu_session_created_dataJSON)
{
    OpenAPI_pdu_session_created_data_t *pdu_session_created_data_local_var = NULL;
    cJSON *pdu_session_type = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "pduSessionType");
    if (!pdu_session_type) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [pdu_session_type]");
        goto end;
    }

    OpenAPI_pdu_session_type_e pdu_session_typeVariable;
    if (!cJSON_IsString(pdu_session_type)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [pdu_session_type]");
        goto end;
    }
    pdu_session_typeVariable = OpenAPI_pdu_session_type_FromString(pdu_session_type->valuestring);

    cJSON *ssc_mode = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "sscMode");
    if (!ssc_mode) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [ssc_mode]");
        goto end;
    }

    if (!cJSON_IsString(ssc_mode)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [ssc_mode]");
        goto end;
    }

    cJSON *hcn_tunnel_info = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "hcnTunnelInfo");

    OpenAPI_tunnel_info_t *hcn_tunnel_info_local_nonprim = NULL;
    if (hcn_tunnel_info) {
    hcn_tunnel_info_local_nonprim = OpenAPI_tunnel_info_parseFromJSON(hcn_tunnel_info);
    }

    cJSON *cn_tunnel_info = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "cnTunnelInfo");

    OpenAPI_tunnel_info_t *cn_tunnel_info_local_nonprim = NULL;
    if (cn_tunnel_info) {
    cn_tunnel_info_local_nonprim = OpenAPI_tunnel_info_parseFromJSON(cn_tunnel_info);
    }

    cJSON *additional_cn_tunnel_info = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "additionalCnTunnelInfo");

    OpenAPI_tunnel_info_t *additional_cn_tunnel_info_local_nonprim = NULL;
    if (additional_cn_tunnel_info) {
    additional_cn_tunnel_info_local_nonprim = OpenAPI_tunnel_info_parseFromJSON(additional_cn_tunnel_info);
    }

    cJSON *session_ambr = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "sessionAmbr");

    OpenAPI_ambr_t *session_ambr_local_nonprim = NULL;
    if (session_ambr) {
    session_ambr_local_nonprim = OpenAPI_ambr_parseFromJSON(session_ambr);
    }

    cJSON *qos_flows_setup_list = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "qosFlowsSetupList");

    OpenAPI_list_t *qos_flows_setup_listList;
    if (qos_flows_setup_list) {
    cJSON *qos_flows_setup_list_local_nonprimitive;
    if (!cJSON_IsArray(qos_flows_setup_list)){
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [qos_flows_setup_list]");
        goto end;
    }

    qos_flows_setup_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(qos_flows_setup_list_local_nonprimitive, qos_flows_setup_list ) {
        if (!cJSON_IsObject(qos_flows_setup_list_local_nonprimitive)) {
            ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [qos_flows_setup_list]");
            goto end;
        }
        OpenAPI_qos_flow_setup_item_t *qos_flows_setup_listItem = OpenAPI_qos_flow_setup_item_parseFromJSON(qos_flows_setup_list_local_nonprimitive);

        if (!qos_flows_setup_listItem) {
            ogs_error("No qos_flows_setup_listItem");
            OpenAPI_list_free(qos_flows_setup_listList);
            goto end;
        }

        OpenAPI_list_add(qos_flows_setup_listList, qos_flows_setup_listItem);
    }
    }

    cJSON *h_smf_instance_id = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "hSmfInstanceId");

    if (h_smf_instance_id) {
    if (!cJSON_IsString(h_smf_instance_id)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [h_smf_instance_id]");
        goto end;
    }
    }

    cJSON *smf_instance_id = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "smfInstanceId");

    if (smf_instance_id) {
    if (!cJSON_IsString(smf_instance_id)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [smf_instance_id]");
        goto end;
    }
    }

    cJSON *pdu_session_id = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "pduSessionId");

    if (pdu_session_id) {
    if (!cJSON_IsNumber(pdu_session_id)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [pdu_session_id]");
        goto end;
    }
    }

    cJSON *s_nssai = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "sNssai");

    OpenAPI_snssai_t *s_nssai_local_nonprim = NULL;
    if (s_nssai) {
    s_nssai_local_nonprim = OpenAPI_snssai_parseFromJSON(s_nssai);
    }

    cJSON *enable_pause_charging = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "enablePauseCharging");

    if (enable_pause_charging) {
    if (!cJSON_IsBool(enable_pause_charging)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [enable_pause_charging]");
        goto end;
    }
    }

    cJSON *ue_ipv4_address = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "ueIpv4Address");

    if (ue_ipv4_address) {
    if (!cJSON_IsString(ue_ipv4_address)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [ue_ipv4_address]");
        goto end;
    }
    }

    cJSON *ue_ipv6_prefix = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "ueIpv6Prefix");

    if (ue_ipv6_prefix) {
    if (!cJSON_IsString(ue_ipv6_prefix)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [ue_ipv6_prefix]");
        goto end;
    }
    }

    cJSON *n1_sm_info_to_ue = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "n1SmInfoToUe");

    OpenAPI_ref_to_binary_data_t *n1_sm_info_to_ue_local_nonprim = NULL;
    if (n1_sm_info_to_ue) {
    n1_sm_info_to_ue_local_nonprim = OpenAPI_ref_to_binary_data_parseFromJSON(n1_sm_info_to_ue);
    }

    cJSON *eps_pdn_cnx_info = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "epsPdnCnxInfo");

    OpenAPI_eps_pdn_cnx_info_t *eps_pdn_cnx_info_local_nonprim = NULL;
    if (eps_pdn_cnx_info) {
    eps_pdn_cnx_info_local_nonprim = OpenAPI_eps_pdn_cnx_info_parseFromJSON(eps_pdn_cnx_info);
    }

    cJSON *eps_bearer_info = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "epsBearerInfo");

    OpenAPI_list_t *eps_bearer_infoList;
    if (eps_bearer_info) {
    cJSON *eps_bearer_info_local_nonprimitive;
    if (!cJSON_IsArray(eps_bearer_info)){
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [eps_bearer_info]");
        goto end;
    }

    eps_bearer_infoList = OpenAPI_list_create();

    cJSON_ArrayForEach(eps_bearer_info_local_nonprimitive, eps_bearer_info ) {
        if (!cJSON_IsObject(eps_bearer_info_local_nonprimitive)) {
            ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [eps_bearer_info]");
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

    cJSON *supported_features = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "supportedFeatures");

    if (supported_features) {
    if (!cJSON_IsString(supported_features)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [supported_features]");
        goto end;
    }
    }

    cJSON *max_integrity_protected_data_rate = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "maxIntegrityProtectedDataRate");

    OpenAPI_max_integrity_protected_data_rate_e max_integrity_protected_data_rateVariable;
    if (max_integrity_protected_data_rate) {
    if (!cJSON_IsString(max_integrity_protected_data_rate)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [max_integrity_protected_data_rate]");
        goto end;
    }
    max_integrity_protected_data_rateVariable = OpenAPI_max_integrity_protected_data_rate_FromString(max_integrity_protected_data_rate->valuestring);
    }

    cJSON *max_integrity_protected_data_rate_dl = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "maxIntegrityProtectedDataRateDl");

    OpenAPI_max_integrity_protected_data_rate_e max_integrity_protected_data_rate_dlVariable;
    if (max_integrity_protected_data_rate_dl) {
    if (!cJSON_IsString(max_integrity_protected_data_rate_dl)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [max_integrity_protected_data_rate_dl]");
        goto end;
    }
    max_integrity_protected_data_rate_dlVariable = OpenAPI_max_integrity_protected_data_rate_FromString(max_integrity_protected_data_rate_dl->valuestring);
    }

    cJSON *always_on_granted = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "alwaysOnGranted");

    if (always_on_granted) {
    if (!cJSON_IsBool(always_on_granted)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [always_on_granted]");
        goto end;
    }
    }

    cJSON *gpsi = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "gpsi");

    if (gpsi) {
    if (!cJSON_IsString(gpsi)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [gpsi]");
        goto end;
    }
    }

    cJSON *up_security = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "upSecurity");

    OpenAPI_up_security_t *up_security_local_nonprim = NULL;
    if (up_security) {
    up_security_local_nonprim = OpenAPI_up_security_parseFromJSON(up_security);
    }

    cJSON *roaming_charging_profile = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "roamingChargingProfile");

    OpenAPI_roaming_charging_profile_t *roaming_charging_profile_local_nonprim = NULL;
    if (roaming_charging_profile) {
    roaming_charging_profile_local_nonprim = OpenAPI_roaming_charging_profile_parseFromJSON(roaming_charging_profile);
    }

    cJSON *h_smf_service_instance_id = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "hSmfServiceInstanceId");

    if (h_smf_service_instance_id) {
    if (!cJSON_IsString(h_smf_service_instance_id)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [h_smf_service_instance_id]");
        goto end;
    }
    }

    cJSON *smf_service_instance_id = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "smfServiceInstanceId");

    if (smf_service_instance_id) {
    if (!cJSON_IsString(smf_service_instance_id)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [smf_service_instance_id]");
        goto end;
    }
    }

    cJSON *recovery_time = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "recoveryTime");

    if (recovery_time) {
    if (!cJSON_IsString(recovery_time)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [recovery_time]");
        goto end;
    }
    }

    cJSON *dnai_list = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "dnaiList");

    OpenAPI_list_t *dnai_listList;
    if (dnai_list) {
    cJSON *dnai_list_local;
    if (!cJSON_IsArray(dnai_list)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [dnai_list]");
        goto end;
    }
    dnai_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(dnai_list_local, dnai_list) {
    if (!cJSON_IsString(dnai_list_local)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [dnai_list]");
        goto end;
    }
    OpenAPI_list_add(dnai_listList , ogs_strdup(dnai_list_local->valuestring));
    }
    }

    cJSON *ipv6_multi_homing_ind = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "ipv6MultiHomingInd");

    if (ipv6_multi_homing_ind) {
    if (!cJSON_IsBool(ipv6_multi_homing_ind)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [ipv6_multi_homing_ind]");
        goto end;
    }
    }

    cJSON *ma_accepted_ind = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "maAcceptedInd");

    if (ma_accepted_ind) {
    if (!cJSON_IsBool(ma_accepted_ind)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [ma_accepted_ind]");
        goto end;
    }
    }

    cJSON *home_provided_charging_id = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "homeProvidedChargingId");

    if (home_provided_charging_id) {
    if (!cJSON_IsString(home_provided_charging_id)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [home_provided_charging_id]");
        goto end;
    }
    }

    cJSON *nef_ext_buf_support_ind = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "nefExtBufSupportInd");

    if (nef_ext_buf_support_ind) {
    if (!cJSON_IsBool(nef_ext_buf_support_ind)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [nef_ext_buf_support_ind]");
        goto end;
    }
    }

    cJSON *small_data_rate_control_enabled = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "smallDataRateControlEnabled");

    if (small_data_rate_control_enabled) {
    if (!cJSON_IsBool(small_data_rate_control_enabled)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [small_data_rate_control_enabled]");
        goto end;
    }
    }

    cJSON *ue_ipv6_interface_id = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "ueIpv6InterfaceId");

    if (ue_ipv6_interface_id) {
    if (!cJSON_IsString(ue_ipv6_interface_id)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [ue_ipv6_interface_id]");
        goto end;
    }
    }

    cJSON *ipv6_index = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "ipv6Index");

    if (ipv6_index) {
    if (!cJSON_IsNumber(ipv6_index)) {
        ogs_error("OpenAPI_pdu_session_created_data_parseFromJSON() failed [ipv6_index]");
        goto end;
    }
    }

    cJSON *dn_aaa_address = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "dnAaaAddress");

    OpenAPI_ip_address_t *dn_aaa_address_local_nonprim = NULL;
    if (dn_aaa_address) {
    dn_aaa_address_local_nonprim = OpenAPI_ip_address_parseFromJSON(dn_aaa_address);
    }

    cJSON *redundant_pdu_session_info = cJSON_GetObjectItemCaseSensitive(pdu_session_created_dataJSON, "redundantPduSessionInfo");

    OpenAPI_redundant_pdu_session_information_t *redundant_pdu_session_info_local_nonprim = NULL;
    if (redundant_pdu_session_info) {
    redundant_pdu_session_info_local_nonprim = OpenAPI_redundant_pdu_session_information_parseFromJSON(redundant_pdu_session_info);
    }

    pdu_session_created_data_local_var = OpenAPI_pdu_session_created_data_create (
        pdu_session_typeVariable,
        ogs_strdup(ssc_mode->valuestring),
        hcn_tunnel_info ? hcn_tunnel_info_local_nonprim : NULL,
        cn_tunnel_info ? cn_tunnel_info_local_nonprim : NULL,
        additional_cn_tunnel_info ? additional_cn_tunnel_info_local_nonprim : NULL,
        session_ambr ? session_ambr_local_nonprim : NULL,
        qos_flows_setup_list ? qos_flows_setup_listList : NULL,
        h_smf_instance_id ? ogs_strdup(h_smf_instance_id->valuestring) : NULL,
        smf_instance_id ? ogs_strdup(smf_instance_id->valuestring) : NULL,
        pdu_session_id ? true : false,
        pdu_session_id ? pdu_session_id->valuedouble : 0,
        s_nssai ? s_nssai_local_nonprim : NULL,
        enable_pause_charging ? true : false,
        enable_pause_charging ? enable_pause_charging->valueint : 0,
        ue_ipv4_address ? ogs_strdup(ue_ipv4_address->valuestring) : NULL,
        ue_ipv6_prefix ? ogs_strdup(ue_ipv6_prefix->valuestring) : NULL,
        n1_sm_info_to_ue ? n1_sm_info_to_ue_local_nonprim : NULL,
        eps_pdn_cnx_info ? eps_pdn_cnx_info_local_nonprim : NULL,
        eps_bearer_info ? eps_bearer_infoList : NULL,
        supported_features ? ogs_strdup(supported_features->valuestring) : NULL,
        max_integrity_protected_data_rate ? max_integrity_protected_data_rateVariable : 0,
        max_integrity_protected_data_rate_dl ? max_integrity_protected_data_rate_dlVariable : 0,
        always_on_granted ? true : false,
        always_on_granted ? always_on_granted->valueint : 0,
        gpsi ? ogs_strdup(gpsi->valuestring) : NULL,
        up_security ? up_security_local_nonprim : NULL,
        roaming_charging_profile ? roaming_charging_profile_local_nonprim : NULL,
        h_smf_service_instance_id ? ogs_strdup(h_smf_service_instance_id->valuestring) : NULL,
        smf_service_instance_id ? ogs_strdup(smf_service_instance_id->valuestring) : NULL,
        recovery_time ? ogs_strdup(recovery_time->valuestring) : NULL,
        dnai_list ? dnai_listList : NULL,
        ipv6_multi_homing_ind ? true : false,
        ipv6_multi_homing_ind ? ipv6_multi_homing_ind->valueint : 0,
        ma_accepted_ind ? true : false,
        ma_accepted_ind ? ma_accepted_ind->valueint : 0,
        home_provided_charging_id ? ogs_strdup(home_provided_charging_id->valuestring) : NULL,
        nef_ext_buf_support_ind ? true : false,
        nef_ext_buf_support_ind ? nef_ext_buf_support_ind->valueint : 0,
        small_data_rate_control_enabled ? true : false,
        small_data_rate_control_enabled ? small_data_rate_control_enabled->valueint : 0,
        ue_ipv6_interface_id ? ogs_strdup(ue_ipv6_interface_id->valuestring) : NULL,
        ipv6_index ? true : false,
        ipv6_index ? ipv6_index->valuedouble : 0,
        dn_aaa_address ? dn_aaa_address_local_nonprim : NULL,
        redundant_pdu_session_info ? redundant_pdu_session_info_local_nonprim : NULL
    );

    return pdu_session_created_data_local_var;
end:
    return NULL;
}

OpenAPI_pdu_session_created_data_t *OpenAPI_pdu_session_created_data_copy(OpenAPI_pdu_session_created_data_t *dst, OpenAPI_pdu_session_created_data_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_pdu_session_created_data_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_pdu_session_created_data_convertToJSON() failed");
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

    OpenAPI_pdu_session_created_data_free(dst);
    dst = OpenAPI_pdu_session_created_data_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

