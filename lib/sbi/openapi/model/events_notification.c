
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "events_notification.h"

OpenAPI_events_notification_t *OpenAPI_events_notification_create(
    OpenAPI_access_type_e access_type,
    OpenAPI_additional_access_info_t *add_access_info,
    OpenAPI_additional_access_info_t *rel_access_info,
    OpenAPI_acc_net_charging_address_t *an_charg_addr,
    OpenAPI_list_t *an_charg_ids,
    OpenAPI_an_gw_address_t *an_gw_addr,
    char *ev_subs_uri,
    OpenAPI_list_t *ev_notifs,
    OpenAPI_list_t *failed_resourc_alloc_reports,
    OpenAPI_list_t *succ_resourc_alloc_reports,
    OpenAPI_net_loc_access_support_e no_net_loc_supp,
    OpenAPI_list_t *out_of_cred_reports,
    OpenAPI_plmn_id_nid_t *plmn_id,
    OpenAPI_list_t *qnc_reports,
    OpenAPI_list_t *qos_mon_reports,
    OpenAPI_list_t *ran_nas_rel_causes,
    OpenAPI_rat_type_e rat_type,
    OpenAPI_user_location_t *ue_loc,
    char *ue_time_zone,
    OpenAPI_accumulated_usage_t *usg_rep,
    OpenAPI_bridge_management_container_t *tsn_bridge_man_cont,
    OpenAPI_port_management_container_t *tsn_port_man_cont_dstt,
    OpenAPI_list_t *tsn_port_man_cont_nwtts
)
{
    OpenAPI_events_notification_t *events_notification_local_var = ogs_malloc(sizeof(OpenAPI_events_notification_t));
    ogs_assert(events_notification_local_var);

    events_notification_local_var->access_type = access_type;
    events_notification_local_var->add_access_info = add_access_info;
    events_notification_local_var->rel_access_info = rel_access_info;
    events_notification_local_var->an_charg_addr = an_charg_addr;
    events_notification_local_var->an_charg_ids = an_charg_ids;
    events_notification_local_var->an_gw_addr = an_gw_addr;
    events_notification_local_var->ev_subs_uri = ev_subs_uri;
    events_notification_local_var->ev_notifs = ev_notifs;
    events_notification_local_var->failed_resourc_alloc_reports = failed_resourc_alloc_reports;
    events_notification_local_var->succ_resourc_alloc_reports = succ_resourc_alloc_reports;
    events_notification_local_var->no_net_loc_supp = no_net_loc_supp;
    events_notification_local_var->out_of_cred_reports = out_of_cred_reports;
    events_notification_local_var->plmn_id = plmn_id;
    events_notification_local_var->qnc_reports = qnc_reports;
    events_notification_local_var->qos_mon_reports = qos_mon_reports;
    events_notification_local_var->ran_nas_rel_causes = ran_nas_rel_causes;
    events_notification_local_var->rat_type = rat_type;
    events_notification_local_var->ue_loc = ue_loc;
    events_notification_local_var->ue_time_zone = ue_time_zone;
    events_notification_local_var->usg_rep = usg_rep;
    events_notification_local_var->tsn_bridge_man_cont = tsn_bridge_man_cont;
    events_notification_local_var->tsn_port_man_cont_dstt = tsn_port_man_cont_dstt;
    events_notification_local_var->tsn_port_man_cont_nwtts = tsn_port_man_cont_nwtts;

    return events_notification_local_var;
}

void OpenAPI_events_notification_free(OpenAPI_events_notification_t *events_notification)
{
    if (NULL == events_notification) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_additional_access_info_free(events_notification->add_access_info);
    OpenAPI_additional_access_info_free(events_notification->rel_access_info);
    OpenAPI_acc_net_charging_address_free(events_notification->an_charg_addr);
    OpenAPI_list_for_each(events_notification->an_charg_ids, node) {
        OpenAPI_access_net_charging_identifier_free(node->data);
    }
    OpenAPI_list_free(events_notification->an_charg_ids);
    OpenAPI_an_gw_address_free(events_notification->an_gw_addr);
    ogs_free(events_notification->ev_subs_uri);
    OpenAPI_list_for_each(events_notification->ev_notifs, node) {
        OpenAPI_af_event_notification_free(node->data);
    }
    OpenAPI_list_free(events_notification->ev_notifs);
    OpenAPI_list_for_each(events_notification->failed_resourc_alloc_reports, node) {
        OpenAPI_resources_allocation_info_free(node->data);
    }
    OpenAPI_list_free(events_notification->failed_resourc_alloc_reports);
    OpenAPI_list_for_each(events_notification->succ_resourc_alloc_reports, node) {
        OpenAPI_resources_allocation_info_free(node->data);
    }
    OpenAPI_list_free(events_notification->succ_resourc_alloc_reports);
    OpenAPI_list_for_each(events_notification->out_of_cred_reports, node) {
        OpenAPI_out_of_credit_information_free(node->data);
    }
    OpenAPI_list_free(events_notification->out_of_cred_reports);
    OpenAPI_plmn_id_nid_free(events_notification->plmn_id);
    OpenAPI_list_for_each(events_notification->qnc_reports, node) {
        OpenAPI_qos_notification_control_info_free(node->data);
    }
    OpenAPI_list_free(events_notification->qnc_reports);
    OpenAPI_list_for_each(events_notification->qos_mon_reports, node) {
        OpenAPI_qos_monitoring_report_free(node->data);
    }
    OpenAPI_list_free(events_notification->qos_mon_reports);
    OpenAPI_list_for_each(events_notification->ran_nas_rel_causes, node) {
        OpenAPI_ran_nas_rel_cause_free(node->data);
    }
    OpenAPI_list_free(events_notification->ran_nas_rel_causes);
    OpenAPI_user_location_free(events_notification->ue_loc);
    ogs_free(events_notification->ue_time_zone);
    OpenAPI_accumulated_usage_free(events_notification->usg_rep);
    OpenAPI_bridge_management_container_free(events_notification->tsn_bridge_man_cont);
    OpenAPI_port_management_container_free(events_notification->tsn_port_man_cont_dstt);
    OpenAPI_list_for_each(events_notification->tsn_port_man_cont_nwtts, node) {
        OpenAPI_port_management_container_free(node->data);
    }
    OpenAPI_list_free(events_notification->tsn_port_man_cont_nwtts);
    ogs_free(events_notification);
}

cJSON *OpenAPI_events_notification_convertToJSON(OpenAPI_events_notification_t *events_notification)
{
    cJSON *item = NULL;

    if (events_notification == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [EventsNotification]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (events_notification->access_type) {
    if (cJSON_AddStringToObject(item, "accessType", OpenAPI_access_type_ToString(events_notification->access_type)) == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [access_type]");
        goto end;
    }
    }

    if (events_notification->add_access_info) {
    cJSON *add_access_info_local_JSON = OpenAPI_additional_access_info_convertToJSON(events_notification->add_access_info);
    if (add_access_info_local_JSON == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [add_access_info]");
        goto end;
    }
    cJSON_AddItemToObject(item, "addAccessInfo", add_access_info_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [add_access_info]");
        goto end;
    }
    }

    if (events_notification->rel_access_info) {
    cJSON *rel_access_info_local_JSON = OpenAPI_additional_access_info_convertToJSON(events_notification->rel_access_info);
    if (rel_access_info_local_JSON == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [rel_access_info]");
        goto end;
    }
    cJSON_AddItemToObject(item, "relAccessInfo", rel_access_info_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [rel_access_info]");
        goto end;
    }
    }

    if (events_notification->an_charg_addr) {
    cJSON *an_charg_addr_local_JSON = OpenAPI_acc_net_charging_address_convertToJSON(events_notification->an_charg_addr);
    if (an_charg_addr_local_JSON == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [an_charg_addr]");
        goto end;
    }
    cJSON_AddItemToObject(item, "anChargAddr", an_charg_addr_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [an_charg_addr]");
        goto end;
    }
    }

    if (events_notification->an_charg_ids) {
    cJSON *an_charg_idsList = cJSON_AddArrayToObject(item, "anChargIds");
    if (an_charg_idsList == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [an_charg_ids]");
        goto end;
    }

    OpenAPI_lnode_t *an_charg_ids_node;
    if (events_notification->an_charg_ids) {
        OpenAPI_list_for_each(events_notification->an_charg_ids, an_charg_ids_node) {
            cJSON *itemLocal = OpenAPI_access_net_charging_identifier_convertToJSON(an_charg_ids_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_events_notification_convertToJSON() failed [an_charg_ids]");
                goto end;
            }
            cJSON_AddItemToArray(an_charg_idsList, itemLocal);
        }
    }
    }

    if (events_notification->an_gw_addr) {
    cJSON *an_gw_addr_local_JSON = OpenAPI_an_gw_address_convertToJSON(events_notification->an_gw_addr);
    if (an_gw_addr_local_JSON == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [an_gw_addr]");
        goto end;
    }
    cJSON_AddItemToObject(item, "anGwAddr", an_gw_addr_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [an_gw_addr]");
        goto end;
    }
    }

    if (cJSON_AddStringToObject(item, "evSubsUri", events_notification->ev_subs_uri) == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [ev_subs_uri]");
        goto end;
    }

    cJSON *ev_notifsList = cJSON_AddArrayToObject(item, "evNotifs");
    if (ev_notifsList == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [ev_notifs]");
        goto end;
    }

    OpenAPI_lnode_t *ev_notifs_node;
    if (events_notification->ev_notifs) {
        OpenAPI_list_for_each(events_notification->ev_notifs, ev_notifs_node) {
            cJSON *itemLocal = OpenAPI_af_event_notification_convertToJSON(ev_notifs_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_events_notification_convertToJSON() failed [ev_notifs]");
                goto end;
            }
            cJSON_AddItemToArray(ev_notifsList, itemLocal);
        }
    }

    if (events_notification->failed_resourc_alloc_reports) {
    cJSON *failed_resourc_alloc_reportsList = cJSON_AddArrayToObject(item, "failedResourcAllocReports");
    if (failed_resourc_alloc_reportsList == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [failed_resourc_alloc_reports]");
        goto end;
    }

    OpenAPI_lnode_t *failed_resourc_alloc_reports_node;
    if (events_notification->failed_resourc_alloc_reports) {
        OpenAPI_list_for_each(events_notification->failed_resourc_alloc_reports, failed_resourc_alloc_reports_node) {
            cJSON *itemLocal = OpenAPI_resources_allocation_info_convertToJSON(failed_resourc_alloc_reports_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_events_notification_convertToJSON() failed [failed_resourc_alloc_reports]");
                goto end;
            }
            cJSON_AddItemToArray(failed_resourc_alloc_reportsList, itemLocal);
        }
    }
    }

    if (events_notification->succ_resourc_alloc_reports) {
    cJSON *succ_resourc_alloc_reportsList = cJSON_AddArrayToObject(item, "succResourcAllocReports");
    if (succ_resourc_alloc_reportsList == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [succ_resourc_alloc_reports]");
        goto end;
    }

    OpenAPI_lnode_t *succ_resourc_alloc_reports_node;
    if (events_notification->succ_resourc_alloc_reports) {
        OpenAPI_list_for_each(events_notification->succ_resourc_alloc_reports, succ_resourc_alloc_reports_node) {
            cJSON *itemLocal = OpenAPI_resources_allocation_info_convertToJSON(succ_resourc_alloc_reports_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_events_notification_convertToJSON() failed [succ_resourc_alloc_reports]");
                goto end;
            }
            cJSON_AddItemToArray(succ_resourc_alloc_reportsList, itemLocal);
        }
    }
    }

    if (events_notification->no_net_loc_supp) {
    if (cJSON_AddStringToObject(item, "noNetLocSupp", OpenAPI_net_loc_access_support_ToString(events_notification->no_net_loc_supp)) == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [no_net_loc_supp]");
        goto end;
    }
    }

    if (events_notification->out_of_cred_reports) {
    cJSON *out_of_cred_reportsList = cJSON_AddArrayToObject(item, "outOfCredReports");
    if (out_of_cred_reportsList == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [out_of_cred_reports]");
        goto end;
    }

    OpenAPI_lnode_t *out_of_cred_reports_node;
    if (events_notification->out_of_cred_reports) {
        OpenAPI_list_for_each(events_notification->out_of_cred_reports, out_of_cred_reports_node) {
            cJSON *itemLocal = OpenAPI_out_of_credit_information_convertToJSON(out_of_cred_reports_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_events_notification_convertToJSON() failed [out_of_cred_reports]");
                goto end;
            }
            cJSON_AddItemToArray(out_of_cred_reportsList, itemLocal);
        }
    }
    }

    if (events_notification->plmn_id) {
    cJSON *plmn_id_local_JSON = OpenAPI_plmn_id_nid_convertToJSON(events_notification->plmn_id);
    if (plmn_id_local_JSON == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [plmn_id]");
        goto end;
    }
    cJSON_AddItemToObject(item, "plmnId", plmn_id_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [plmn_id]");
        goto end;
    }
    }

    if (events_notification->qnc_reports) {
    cJSON *qnc_reportsList = cJSON_AddArrayToObject(item, "qncReports");
    if (qnc_reportsList == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [qnc_reports]");
        goto end;
    }

    OpenAPI_lnode_t *qnc_reports_node;
    if (events_notification->qnc_reports) {
        OpenAPI_list_for_each(events_notification->qnc_reports, qnc_reports_node) {
            cJSON *itemLocal = OpenAPI_qos_notification_control_info_convertToJSON(qnc_reports_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_events_notification_convertToJSON() failed [qnc_reports]");
                goto end;
            }
            cJSON_AddItemToArray(qnc_reportsList, itemLocal);
        }
    }
    }

    if (events_notification->qos_mon_reports) {
    cJSON *qos_mon_reportsList = cJSON_AddArrayToObject(item, "qosMonReports");
    if (qos_mon_reportsList == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [qos_mon_reports]");
        goto end;
    }

    OpenAPI_lnode_t *qos_mon_reports_node;
    if (events_notification->qos_mon_reports) {
        OpenAPI_list_for_each(events_notification->qos_mon_reports, qos_mon_reports_node) {
            cJSON *itemLocal = OpenAPI_qos_monitoring_report_convertToJSON(qos_mon_reports_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_events_notification_convertToJSON() failed [qos_mon_reports]");
                goto end;
            }
            cJSON_AddItemToArray(qos_mon_reportsList, itemLocal);
        }
    }
    }

    if (events_notification->ran_nas_rel_causes) {
    cJSON *ran_nas_rel_causesList = cJSON_AddArrayToObject(item, "ranNasRelCauses");
    if (ran_nas_rel_causesList == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [ran_nas_rel_causes]");
        goto end;
    }

    OpenAPI_lnode_t *ran_nas_rel_causes_node;
    if (events_notification->ran_nas_rel_causes) {
        OpenAPI_list_for_each(events_notification->ran_nas_rel_causes, ran_nas_rel_causes_node) {
            cJSON *itemLocal = OpenAPI_ran_nas_rel_cause_convertToJSON(ran_nas_rel_causes_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_events_notification_convertToJSON() failed [ran_nas_rel_causes]");
                goto end;
            }
            cJSON_AddItemToArray(ran_nas_rel_causesList, itemLocal);
        }
    }
    }

    if (events_notification->rat_type) {
    if (cJSON_AddStringToObject(item, "ratType", OpenAPI_rat_type_ToString(events_notification->rat_type)) == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [rat_type]");
        goto end;
    }
    }

    if (events_notification->ue_loc) {
    cJSON *ue_loc_local_JSON = OpenAPI_user_location_convertToJSON(events_notification->ue_loc);
    if (ue_loc_local_JSON == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [ue_loc]");
        goto end;
    }
    cJSON_AddItemToObject(item, "ueLoc", ue_loc_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [ue_loc]");
        goto end;
    }
    }

    if (events_notification->ue_time_zone) {
    if (cJSON_AddStringToObject(item, "ueTimeZone", events_notification->ue_time_zone) == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [ue_time_zone]");
        goto end;
    }
    }

    if (events_notification->usg_rep) {
    cJSON *usg_rep_local_JSON = OpenAPI_accumulated_usage_convertToJSON(events_notification->usg_rep);
    if (usg_rep_local_JSON == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [usg_rep]");
        goto end;
    }
    cJSON_AddItemToObject(item, "usgRep", usg_rep_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [usg_rep]");
        goto end;
    }
    }

    if (events_notification->tsn_bridge_man_cont) {
    cJSON *tsn_bridge_man_cont_local_JSON = OpenAPI_bridge_management_container_convertToJSON(events_notification->tsn_bridge_man_cont);
    if (tsn_bridge_man_cont_local_JSON == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [tsn_bridge_man_cont]");
        goto end;
    }
    cJSON_AddItemToObject(item, "tsnBridgeManCont", tsn_bridge_man_cont_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [tsn_bridge_man_cont]");
        goto end;
    }
    }

    if (events_notification->tsn_port_man_cont_dstt) {
    cJSON *tsn_port_man_cont_dstt_local_JSON = OpenAPI_port_management_container_convertToJSON(events_notification->tsn_port_man_cont_dstt);
    if (tsn_port_man_cont_dstt_local_JSON == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [tsn_port_man_cont_dstt]");
        goto end;
    }
    cJSON_AddItemToObject(item, "tsnPortManContDstt", tsn_port_man_cont_dstt_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [tsn_port_man_cont_dstt]");
        goto end;
    }
    }

    if (events_notification->tsn_port_man_cont_nwtts) {
    cJSON *tsn_port_man_cont_nwttsList = cJSON_AddArrayToObject(item, "tsnPortManContNwtts");
    if (tsn_port_man_cont_nwttsList == NULL) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed [tsn_port_man_cont_nwtts]");
        goto end;
    }

    OpenAPI_lnode_t *tsn_port_man_cont_nwtts_node;
    if (events_notification->tsn_port_man_cont_nwtts) {
        OpenAPI_list_for_each(events_notification->tsn_port_man_cont_nwtts, tsn_port_man_cont_nwtts_node) {
            cJSON *itemLocal = OpenAPI_port_management_container_convertToJSON(tsn_port_man_cont_nwtts_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_events_notification_convertToJSON() failed [tsn_port_man_cont_nwtts]");
                goto end;
            }
            cJSON_AddItemToArray(tsn_port_man_cont_nwttsList, itemLocal);
        }
    }
    }

end:
    return item;
}

OpenAPI_events_notification_t *OpenAPI_events_notification_parseFromJSON(cJSON *events_notificationJSON)
{
    OpenAPI_events_notification_t *events_notification_local_var = NULL;
    cJSON *access_type = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "accessType");

    OpenAPI_access_type_e access_typeVariable;
    if (access_type) {
    if (!cJSON_IsString(access_type)) {
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [access_type]");
        goto end;
    }
    access_typeVariable = OpenAPI_access_type_FromString(access_type->valuestring);
    }

    cJSON *add_access_info = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "addAccessInfo");

    OpenAPI_additional_access_info_t *add_access_info_local_nonprim = NULL;
    if (add_access_info) {
    add_access_info_local_nonprim = OpenAPI_additional_access_info_parseFromJSON(add_access_info);
    }

    cJSON *rel_access_info = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "relAccessInfo");

    OpenAPI_additional_access_info_t *rel_access_info_local_nonprim = NULL;
    if (rel_access_info) {
    rel_access_info_local_nonprim = OpenAPI_additional_access_info_parseFromJSON(rel_access_info);
    }

    cJSON *an_charg_addr = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "anChargAddr");

    OpenAPI_acc_net_charging_address_t *an_charg_addr_local_nonprim = NULL;
    if (an_charg_addr) {
    an_charg_addr_local_nonprim = OpenAPI_acc_net_charging_address_parseFromJSON(an_charg_addr);
    }

    cJSON *an_charg_ids = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "anChargIds");

    OpenAPI_list_t *an_charg_idsList;
    if (an_charg_ids) {
    cJSON *an_charg_ids_local_nonprimitive;
    if (!cJSON_IsArray(an_charg_ids)){
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [an_charg_ids]");
        goto end;
    }

    an_charg_idsList = OpenAPI_list_create();

    cJSON_ArrayForEach(an_charg_ids_local_nonprimitive, an_charg_ids ) {
        if (!cJSON_IsObject(an_charg_ids_local_nonprimitive)) {
            ogs_error("OpenAPI_events_notification_parseFromJSON() failed [an_charg_ids]");
            goto end;
        }
        OpenAPI_access_net_charging_identifier_t *an_charg_idsItem = OpenAPI_access_net_charging_identifier_parseFromJSON(an_charg_ids_local_nonprimitive);

        if (!an_charg_idsItem) {
            ogs_error("No an_charg_idsItem");
            OpenAPI_list_free(an_charg_idsList);
            goto end;
        }

        OpenAPI_list_add(an_charg_idsList, an_charg_idsItem);
    }
    }

    cJSON *an_gw_addr = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "anGwAddr");

    OpenAPI_an_gw_address_t *an_gw_addr_local_nonprim = NULL;
    if (an_gw_addr) {
    an_gw_addr_local_nonprim = OpenAPI_an_gw_address_parseFromJSON(an_gw_addr);
    }

    cJSON *ev_subs_uri = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "evSubsUri");
    if (!ev_subs_uri) {
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [ev_subs_uri]");
        goto end;
    }

    if (!cJSON_IsString(ev_subs_uri)) {
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [ev_subs_uri]");
        goto end;
    }

    cJSON *ev_notifs = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "evNotifs");
    if (!ev_notifs) {
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [ev_notifs]");
        goto end;
    }

    OpenAPI_list_t *ev_notifsList;
    cJSON *ev_notifs_local_nonprimitive;
    if (!cJSON_IsArray(ev_notifs)){
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [ev_notifs]");
        goto end;
    }

    ev_notifsList = OpenAPI_list_create();

    cJSON_ArrayForEach(ev_notifs_local_nonprimitive, ev_notifs ) {
        if (!cJSON_IsObject(ev_notifs_local_nonprimitive)) {
            ogs_error("OpenAPI_events_notification_parseFromJSON() failed [ev_notifs]");
            goto end;
        }
        OpenAPI_af_event_notification_t *ev_notifsItem = OpenAPI_af_event_notification_parseFromJSON(ev_notifs_local_nonprimitive);

        if (!ev_notifsItem) {
            ogs_error("No ev_notifsItem");
            OpenAPI_list_free(ev_notifsList);
            goto end;
        }

        OpenAPI_list_add(ev_notifsList, ev_notifsItem);
    }

    cJSON *failed_resourc_alloc_reports = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "failedResourcAllocReports");

    OpenAPI_list_t *failed_resourc_alloc_reportsList;
    if (failed_resourc_alloc_reports) {
    cJSON *failed_resourc_alloc_reports_local_nonprimitive;
    if (!cJSON_IsArray(failed_resourc_alloc_reports)){
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [failed_resourc_alloc_reports]");
        goto end;
    }

    failed_resourc_alloc_reportsList = OpenAPI_list_create();

    cJSON_ArrayForEach(failed_resourc_alloc_reports_local_nonprimitive, failed_resourc_alloc_reports ) {
        if (!cJSON_IsObject(failed_resourc_alloc_reports_local_nonprimitive)) {
            ogs_error("OpenAPI_events_notification_parseFromJSON() failed [failed_resourc_alloc_reports]");
            goto end;
        }
        OpenAPI_resources_allocation_info_t *failed_resourc_alloc_reportsItem = OpenAPI_resources_allocation_info_parseFromJSON(failed_resourc_alloc_reports_local_nonprimitive);

        if (!failed_resourc_alloc_reportsItem) {
            ogs_error("No failed_resourc_alloc_reportsItem");
            OpenAPI_list_free(failed_resourc_alloc_reportsList);
            goto end;
        }

        OpenAPI_list_add(failed_resourc_alloc_reportsList, failed_resourc_alloc_reportsItem);
    }
    }

    cJSON *succ_resourc_alloc_reports = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "succResourcAllocReports");

    OpenAPI_list_t *succ_resourc_alloc_reportsList;
    if (succ_resourc_alloc_reports) {
    cJSON *succ_resourc_alloc_reports_local_nonprimitive;
    if (!cJSON_IsArray(succ_resourc_alloc_reports)){
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [succ_resourc_alloc_reports]");
        goto end;
    }

    succ_resourc_alloc_reportsList = OpenAPI_list_create();

    cJSON_ArrayForEach(succ_resourc_alloc_reports_local_nonprimitive, succ_resourc_alloc_reports ) {
        if (!cJSON_IsObject(succ_resourc_alloc_reports_local_nonprimitive)) {
            ogs_error("OpenAPI_events_notification_parseFromJSON() failed [succ_resourc_alloc_reports]");
            goto end;
        }
        OpenAPI_resources_allocation_info_t *succ_resourc_alloc_reportsItem = OpenAPI_resources_allocation_info_parseFromJSON(succ_resourc_alloc_reports_local_nonprimitive);

        if (!succ_resourc_alloc_reportsItem) {
            ogs_error("No succ_resourc_alloc_reportsItem");
            OpenAPI_list_free(succ_resourc_alloc_reportsList);
            goto end;
        }

        OpenAPI_list_add(succ_resourc_alloc_reportsList, succ_resourc_alloc_reportsItem);
    }
    }

    cJSON *no_net_loc_supp = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "noNetLocSupp");

    OpenAPI_net_loc_access_support_e no_net_loc_suppVariable;
    if (no_net_loc_supp) {
    if (!cJSON_IsString(no_net_loc_supp)) {
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [no_net_loc_supp]");
        goto end;
    }
    no_net_loc_suppVariable = OpenAPI_net_loc_access_support_FromString(no_net_loc_supp->valuestring);
    }

    cJSON *out_of_cred_reports = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "outOfCredReports");

    OpenAPI_list_t *out_of_cred_reportsList;
    if (out_of_cred_reports) {
    cJSON *out_of_cred_reports_local_nonprimitive;
    if (!cJSON_IsArray(out_of_cred_reports)){
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [out_of_cred_reports]");
        goto end;
    }

    out_of_cred_reportsList = OpenAPI_list_create();

    cJSON_ArrayForEach(out_of_cred_reports_local_nonprimitive, out_of_cred_reports ) {
        if (!cJSON_IsObject(out_of_cred_reports_local_nonprimitive)) {
            ogs_error("OpenAPI_events_notification_parseFromJSON() failed [out_of_cred_reports]");
            goto end;
        }
        OpenAPI_out_of_credit_information_t *out_of_cred_reportsItem = OpenAPI_out_of_credit_information_parseFromJSON(out_of_cred_reports_local_nonprimitive);

        if (!out_of_cred_reportsItem) {
            ogs_error("No out_of_cred_reportsItem");
            OpenAPI_list_free(out_of_cred_reportsList);
            goto end;
        }

        OpenAPI_list_add(out_of_cred_reportsList, out_of_cred_reportsItem);
    }
    }

    cJSON *plmn_id = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "plmnId");

    OpenAPI_plmn_id_nid_t *plmn_id_local_nonprim = NULL;
    if (plmn_id) {
    plmn_id_local_nonprim = OpenAPI_plmn_id_nid_parseFromJSON(plmn_id);
    }

    cJSON *qnc_reports = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "qncReports");

    OpenAPI_list_t *qnc_reportsList;
    if (qnc_reports) {
    cJSON *qnc_reports_local_nonprimitive;
    if (!cJSON_IsArray(qnc_reports)){
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [qnc_reports]");
        goto end;
    }

    qnc_reportsList = OpenAPI_list_create();

    cJSON_ArrayForEach(qnc_reports_local_nonprimitive, qnc_reports ) {
        if (!cJSON_IsObject(qnc_reports_local_nonprimitive)) {
            ogs_error("OpenAPI_events_notification_parseFromJSON() failed [qnc_reports]");
            goto end;
        }
        OpenAPI_qos_notification_control_info_t *qnc_reportsItem = OpenAPI_qos_notification_control_info_parseFromJSON(qnc_reports_local_nonprimitive);

        if (!qnc_reportsItem) {
            ogs_error("No qnc_reportsItem");
            OpenAPI_list_free(qnc_reportsList);
            goto end;
        }

        OpenAPI_list_add(qnc_reportsList, qnc_reportsItem);
    }
    }

    cJSON *qos_mon_reports = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "qosMonReports");

    OpenAPI_list_t *qos_mon_reportsList;
    if (qos_mon_reports) {
    cJSON *qos_mon_reports_local_nonprimitive;
    if (!cJSON_IsArray(qos_mon_reports)){
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [qos_mon_reports]");
        goto end;
    }

    qos_mon_reportsList = OpenAPI_list_create();

    cJSON_ArrayForEach(qos_mon_reports_local_nonprimitive, qos_mon_reports ) {
        if (!cJSON_IsObject(qos_mon_reports_local_nonprimitive)) {
            ogs_error("OpenAPI_events_notification_parseFromJSON() failed [qos_mon_reports]");
            goto end;
        }
        OpenAPI_qos_monitoring_report_t *qos_mon_reportsItem = OpenAPI_qos_monitoring_report_parseFromJSON(qos_mon_reports_local_nonprimitive);

        if (!qos_mon_reportsItem) {
            ogs_error("No qos_mon_reportsItem");
            OpenAPI_list_free(qos_mon_reportsList);
            goto end;
        }

        OpenAPI_list_add(qos_mon_reportsList, qos_mon_reportsItem);
    }
    }

    cJSON *ran_nas_rel_causes = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "ranNasRelCauses");

    OpenAPI_list_t *ran_nas_rel_causesList;
    if (ran_nas_rel_causes) {
    cJSON *ran_nas_rel_causes_local_nonprimitive;
    if (!cJSON_IsArray(ran_nas_rel_causes)){
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [ran_nas_rel_causes]");
        goto end;
    }

    ran_nas_rel_causesList = OpenAPI_list_create();

    cJSON_ArrayForEach(ran_nas_rel_causes_local_nonprimitive, ran_nas_rel_causes ) {
        if (!cJSON_IsObject(ran_nas_rel_causes_local_nonprimitive)) {
            ogs_error("OpenAPI_events_notification_parseFromJSON() failed [ran_nas_rel_causes]");
            goto end;
        }
        OpenAPI_ran_nas_rel_cause_t *ran_nas_rel_causesItem = OpenAPI_ran_nas_rel_cause_parseFromJSON(ran_nas_rel_causes_local_nonprimitive);

        if (!ran_nas_rel_causesItem) {
            ogs_error("No ran_nas_rel_causesItem");
            OpenAPI_list_free(ran_nas_rel_causesList);
            goto end;
        }

        OpenAPI_list_add(ran_nas_rel_causesList, ran_nas_rel_causesItem);
    }
    }

    cJSON *rat_type = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "ratType");

    OpenAPI_rat_type_e rat_typeVariable;
    if (rat_type) {
    if (!cJSON_IsString(rat_type)) {
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [rat_type]");
        goto end;
    }
    rat_typeVariable = OpenAPI_rat_type_FromString(rat_type->valuestring);
    }

    cJSON *ue_loc = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "ueLoc");

    OpenAPI_user_location_t *ue_loc_local_nonprim = NULL;
    if (ue_loc) {
    ue_loc_local_nonprim = OpenAPI_user_location_parseFromJSON(ue_loc);
    }

    cJSON *ue_time_zone = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "ueTimeZone");

    if (ue_time_zone) {
    if (!cJSON_IsString(ue_time_zone)) {
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [ue_time_zone]");
        goto end;
    }
    }

    cJSON *usg_rep = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "usgRep");

    OpenAPI_accumulated_usage_t *usg_rep_local_nonprim = NULL;
    if (usg_rep) {
    usg_rep_local_nonprim = OpenAPI_accumulated_usage_parseFromJSON(usg_rep);
    }

    cJSON *tsn_bridge_man_cont = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "tsnBridgeManCont");

    OpenAPI_bridge_management_container_t *tsn_bridge_man_cont_local_nonprim = NULL;
    if (tsn_bridge_man_cont) {
    tsn_bridge_man_cont_local_nonprim = OpenAPI_bridge_management_container_parseFromJSON(tsn_bridge_man_cont);
    }

    cJSON *tsn_port_man_cont_dstt = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "tsnPortManContDstt");

    OpenAPI_port_management_container_t *tsn_port_man_cont_dstt_local_nonprim = NULL;
    if (tsn_port_man_cont_dstt) {
    tsn_port_man_cont_dstt_local_nonprim = OpenAPI_port_management_container_parseFromJSON(tsn_port_man_cont_dstt);
    }

    cJSON *tsn_port_man_cont_nwtts = cJSON_GetObjectItemCaseSensitive(events_notificationJSON, "tsnPortManContNwtts");

    OpenAPI_list_t *tsn_port_man_cont_nwttsList;
    if (tsn_port_man_cont_nwtts) {
    cJSON *tsn_port_man_cont_nwtts_local_nonprimitive;
    if (!cJSON_IsArray(tsn_port_man_cont_nwtts)){
        ogs_error("OpenAPI_events_notification_parseFromJSON() failed [tsn_port_man_cont_nwtts]");
        goto end;
    }

    tsn_port_man_cont_nwttsList = OpenAPI_list_create();

    cJSON_ArrayForEach(tsn_port_man_cont_nwtts_local_nonprimitive, tsn_port_man_cont_nwtts ) {
        if (!cJSON_IsObject(tsn_port_man_cont_nwtts_local_nonprimitive)) {
            ogs_error("OpenAPI_events_notification_parseFromJSON() failed [tsn_port_man_cont_nwtts]");
            goto end;
        }
        OpenAPI_port_management_container_t *tsn_port_man_cont_nwttsItem = OpenAPI_port_management_container_parseFromJSON(tsn_port_man_cont_nwtts_local_nonprimitive);

        if (!tsn_port_man_cont_nwttsItem) {
            ogs_error("No tsn_port_man_cont_nwttsItem");
            OpenAPI_list_free(tsn_port_man_cont_nwttsList);
            goto end;
        }

        OpenAPI_list_add(tsn_port_man_cont_nwttsList, tsn_port_man_cont_nwttsItem);
    }
    }

    events_notification_local_var = OpenAPI_events_notification_create (
        access_type ? access_typeVariable : 0,
        add_access_info ? add_access_info_local_nonprim : NULL,
        rel_access_info ? rel_access_info_local_nonprim : NULL,
        an_charg_addr ? an_charg_addr_local_nonprim : NULL,
        an_charg_ids ? an_charg_idsList : NULL,
        an_gw_addr ? an_gw_addr_local_nonprim : NULL,
        ogs_strdup(ev_subs_uri->valuestring),
        ev_notifsList,
        failed_resourc_alloc_reports ? failed_resourc_alloc_reportsList : NULL,
        succ_resourc_alloc_reports ? succ_resourc_alloc_reportsList : NULL,
        no_net_loc_supp ? no_net_loc_suppVariable : 0,
        out_of_cred_reports ? out_of_cred_reportsList : NULL,
        plmn_id ? plmn_id_local_nonprim : NULL,
        qnc_reports ? qnc_reportsList : NULL,
        qos_mon_reports ? qos_mon_reportsList : NULL,
        ran_nas_rel_causes ? ran_nas_rel_causesList : NULL,
        rat_type ? rat_typeVariable : 0,
        ue_loc ? ue_loc_local_nonprim : NULL,
        ue_time_zone ? ogs_strdup(ue_time_zone->valuestring) : NULL,
        usg_rep ? usg_rep_local_nonprim : NULL,
        tsn_bridge_man_cont ? tsn_bridge_man_cont_local_nonprim : NULL,
        tsn_port_man_cont_dstt ? tsn_port_man_cont_dstt_local_nonprim : NULL,
        tsn_port_man_cont_nwtts ? tsn_port_man_cont_nwttsList : NULL
    );

    return events_notification_local_var;
end:
    return NULL;
}

OpenAPI_events_notification_t *OpenAPI_events_notification_copy(OpenAPI_events_notification_t *dst, OpenAPI_events_notification_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_events_notification_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_events_notification_convertToJSON() failed");
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

    OpenAPI_events_notification_free(dst);
    dst = OpenAPI_events_notification_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

