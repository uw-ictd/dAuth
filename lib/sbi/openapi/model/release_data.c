
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "release_data.h"

OpenAPI_release_data_t *OpenAPI_release_data_create(
    OpenAPI_cause_e cause,
    OpenAPI_ng_ap_cause_t *ng_ap_cause,
    bool is__5g_mm_cause_value,
    int _5g_mm_cause_value,
    OpenAPI_user_location_t *ue_location,
    char *ue_time_zone,
    OpenAPI_user_location_t *add_ue_location,
    OpenAPI_list_t *secondary_rat_usage_report,
    OpenAPI_list_t *secondary_rat_usage_info,
    OpenAPI_n4_information_t *n4_info,
    OpenAPI_n4_information_t *n4_info_ext1,
    OpenAPI_n4_information_t *n4_info_ext2
)
{
    OpenAPI_release_data_t *release_data_local_var = ogs_malloc(sizeof(OpenAPI_release_data_t));
    ogs_assert(release_data_local_var);

    release_data_local_var->cause = cause;
    release_data_local_var->ng_ap_cause = ng_ap_cause;
    release_data_local_var->is__5g_mm_cause_value = is__5g_mm_cause_value;
    release_data_local_var->_5g_mm_cause_value = _5g_mm_cause_value;
    release_data_local_var->ue_location = ue_location;
    release_data_local_var->ue_time_zone = ue_time_zone;
    release_data_local_var->add_ue_location = add_ue_location;
    release_data_local_var->secondary_rat_usage_report = secondary_rat_usage_report;
    release_data_local_var->secondary_rat_usage_info = secondary_rat_usage_info;
    release_data_local_var->n4_info = n4_info;
    release_data_local_var->n4_info_ext1 = n4_info_ext1;
    release_data_local_var->n4_info_ext2 = n4_info_ext2;

    return release_data_local_var;
}

void OpenAPI_release_data_free(OpenAPI_release_data_t *release_data)
{
    if (NULL == release_data) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_ng_ap_cause_free(release_data->ng_ap_cause);
    OpenAPI_user_location_free(release_data->ue_location);
    ogs_free(release_data->ue_time_zone);
    OpenAPI_user_location_free(release_data->add_ue_location);
    OpenAPI_list_for_each(release_data->secondary_rat_usage_report, node) {
        OpenAPI_secondary_rat_usage_report_free(node->data);
    }
    OpenAPI_list_free(release_data->secondary_rat_usage_report);
    OpenAPI_list_for_each(release_data->secondary_rat_usage_info, node) {
        OpenAPI_secondary_rat_usage_info_free(node->data);
    }
    OpenAPI_list_free(release_data->secondary_rat_usage_info);
    OpenAPI_n4_information_free(release_data->n4_info);
    OpenAPI_n4_information_free(release_data->n4_info_ext1);
    OpenAPI_n4_information_free(release_data->n4_info_ext2);
    ogs_free(release_data);
}

cJSON *OpenAPI_release_data_convertToJSON(OpenAPI_release_data_t *release_data)
{
    cJSON *item = NULL;

    if (release_data == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [ReleaseData]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (release_data->cause) {
    if (cJSON_AddStringToObject(item, "cause", OpenAPI_cause_ToString(release_data->cause)) == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [cause]");
        goto end;
    }
    }

    if (release_data->ng_ap_cause) {
    cJSON *ng_ap_cause_local_JSON = OpenAPI_ng_ap_cause_convertToJSON(release_data->ng_ap_cause);
    if (ng_ap_cause_local_JSON == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [ng_ap_cause]");
        goto end;
    }
    cJSON_AddItemToObject(item, "ngApCause", ng_ap_cause_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [ng_ap_cause]");
        goto end;
    }
    }

    if (release_data->is__5g_mm_cause_value) {
    if (cJSON_AddNumberToObject(item, "5gMmCauseValue", release_data->_5g_mm_cause_value) == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [_5g_mm_cause_value]");
        goto end;
    }
    }

    if (release_data->ue_location) {
    cJSON *ue_location_local_JSON = OpenAPI_user_location_convertToJSON(release_data->ue_location);
    if (ue_location_local_JSON == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [ue_location]");
        goto end;
    }
    cJSON_AddItemToObject(item, "ueLocation", ue_location_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [ue_location]");
        goto end;
    }
    }

    if (release_data->ue_time_zone) {
    if (cJSON_AddStringToObject(item, "ueTimeZone", release_data->ue_time_zone) == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [ue_time_zone]");
        goto end;
    }
    }

    if (release_data->add_ue_location) {
    cJSON *add_ue_location_local_JSON = OpenAPI_user_location_convertToJSON(release_data->add_ue_location);
    if (add_ue_location_local_JSON == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [add_ue_location]");
        goto end;
    }
    cJSON_AddItemToObject(item, "addUeLocation", add_ue_location_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [add_ue_location]");
        goto end;
    }
    }

    if (release_data->secondary_rat_usage_report) {
    cJSON *secondary_rat_usage_reportList = cJSON_AddArrayToObject(item, "secondaryRatUsageReport");
    if (secondary_rat_usage_reportList == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [secondary_rat_usage_report]");
        goto end;
    }

    OpenAPI_lnode_t *secondary_rat_usage_report_node;
    if (release_data->secondary_rat_usage_report) {
        OpenAPI_list_for_each(release_data->secondary_rat_usage_report, secondary_rat_usage_report_node) {
            cJSON *itemLocal = OpenAPI_secondary_rat_usage_report_convertToJSON(secondary_rat_usage_report_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_release_data_convertToJSON() failed [secondary_rat_usage_report]");
                goto end;
            }
            cJSON_AddItemToArray(secondary_rat_usage_reportList, itemLocal);
        }
    }
    }

    if (release_data->secondary_rat_usage_info) {
    cJSON *secondary_rat_usage_infoList = cJSON_AddArrayToObject(item, "secondaryRatUsageInfo");
    if (secondary_rat_usage_infoList == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [secondary_rat_usage_info]");
        goto end;
    }

    OpenAPI_lnode_t *secondary_rat_usage_info_node;
    if (release_data->secondary_rat_usage_info) {
        OpenAPI_list_for_each(release_data->secondary_rat_usage_info, secondary_rat_usage_info_node) {
            cJSON *itemLocal = OpenAPI_secondary_rat_usage_info_convertToJSON(secondary_rat_usage_info_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_release_data_convertToJSON() failed [secondary_rat_usage_info]");
                goto end;
            }
            cJSON_AddItemToArray(secondary_rat_usage_infoList, itemLocal);
        }
    }
    }

    if (release_data->n4_info) {
    cJSON *n4_info_local_JSON = OpenAPI_n4_information_convertToJSON(release_data->n4_info);
    if (n4_info_local_JSON == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [n4_info]");
        goto end;
    }
    cJSON_AddItemToObject(item, "n4Info", n4_info_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [n4_info]");
        goto end;
    }
    }

    if (release_data->n4_info_ext1) {
    cJSON *n4_info_ext1_local_JSON = OpenAPI_n4_information_convertToJSON(release_data->n4_info_ext1);
    if (n4_info_ext1_local_JSON == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [n4_info_ext1]");
        goto end;
    }
    cJSON_AddItemToObject(item, "n4InfoExt1", n4_info_ext1_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [n4_info_ext1]");
        goto end;
    }
    }

    if (release_data->n4_info_ext2) {
    cJSON *n4_info_ext2_local_JSON = OpenAPI_n4_information_convertToJSON(release_data->n4_info_ext2);
    if (n4_info_ext2_local_JSON == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [n4_info_ext2]");
        goto end;
    }
    cJSON_AddItemToObject(item, "n4InfoExt2", n4_info_ext2_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed [n4_info_ext2]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_release_data_t *OpenAPI_release_data_parseFromJSON(cJSON *release_dataJSON)
{
    OpenAPI_release_data_t *release_data_local_var = NULL;
    cJSON *cause = cJSON_GetObjectItemCaseSensitive(release_dataJSON, "cause");

    OpenAPI_cause_e causeVariable;
    if (cause) {
    if (!cJSON_IsString(cause)) {
        ogs_error("OpenAPI_release_data_parseFromJSON() failed [cause]");
        goto end;
    }
    causeVariable = OpenAPI_cause_FromString(cause->valuestring);
    }

    cJSON *ng_ap_cause = cJSON_GetObjectItemCaseSensitive(release_dataJSON, "ngApCause");

    OpenAPI_ng_ap_cause_t *ng_ap_cause_local_nonprim = NULL;
    if (ng_ap_cause) {
    ng_ap_cause_local_nonprim = OpenAPI_ng_ap_cause_parseFromJSON(ng_ap_cause);
    }

    cJSON *_5g_mm_cause_value = cJSON_GetObjectItemCaseSensitive(release_dataJSON, "5gMmCauseValue");

    if (_5g_mm_cause_value) {
    if (!cJSON_IsNumber(_5g_mm_cause_value)) {
        ogs_error("OpenAPI_release_data_parseFromJSON() failed [_5g_mm_cause_value]");
        goto end;
    }
    }

    cJSON *ue_location = cJSON_GetObjectItemCaseSensitive(release_dataJSON, "ueLocation");

    OpenAPI_user_location_t *ue_location_local_nonprim = NULL;
    if (ue_location) {
    ue_location_local_nonprim = OpenAPI_user_location_parseFromJSON(ue_location);
    }

    cJSON *ue_time_zone = cJSON_GetObjectItemCaseSensitive(release_dataJSON, "ueTimeZone");

    if (ue_time_zone) {
    if (!cJSON_IsString(ue_time_zone)) {
        ogs_error("OpenAPI_release_data_parseFromJSON() failed [ue_time_zone]");
        goto end;
    }
    }

    cJSON *add_ue_location = cJSON_GetObjectItemCaseSensitive(release_dataJSON, "addUeLocation");

    OpenAPI_user_location_t *add_ue_location_local_nonprim = NULL;
    if (add_ue_location) {
    add_ue_location_local_nonprim = OpenAPI_user_location_parseFromJSON(add_ue_location);
    }

    cJSON *secondary_rat_usage_report = cJSON_GetObjectItemCaseSensitive(release_dataJSON, "secondaryRatUsageReport");

    OpenAPI_list_t *secondary_rat_usage_reportList;
    if (secondary_rat_usage_report) {
    cJSON *secondary_rat_usage_report_local_nonprimitive;
    if (!cJSON_IsArray(secondary_rat_usage_report)){
        ogs_error("OpenAPI_release_data_parseFromJSON() failed [secondary_rat_usage_report]");
        goto end;
    }

    secondary_rat_usage_reportList = OpenAPI_list_create();

    cJSON_ArrayForEach(secondary_rat_usage_report_local_nonprimitive, secondary_rat_usage_report ) {
        if (!cJSON_IsObject(secondary_rat_usage_report_local_nonprimitive)) {
            ogs_error("OpenAPI_release_data_parseFromJSON() failed [secondary_rat_usage_report]");
            goto end;
        }
        OpenAPI_secondary_rat_usage_report_t *secondary_rat_usage_reportItem = OpenAPI_secondary_rat_usage_report_parseFromJSON(secondary_rat_usage_report_local_nonprimitive);

        if (!secondary_rat_usage_reportItem) {
            ogs_error("No secondary_rat_usage_reportItem");
            OpenAPI_list_free(secondary_rat_usage_reportList);
            goto end;
        }

        OpenAPI_list_add(secondary_rat_usage_reportList, secondary_rat_usage_reportItem);
    }
    }

    cJSON *secondary_rat_usage_info = cJSON_GetObjectItemCaseSensitive(release_dataJSON, "secondaryRatUsageInfo");

    OpenAPI_list_t *secondary_rat_usage_infoList;
    if (secondary_rat_usage_info) {
    cJSON *secondary_rat_usage_info_local_nonprimitive;
    if (!cJSON_IsArray(secondary_rat_usage_info)){
        ogs_error("OpenAPI_release_data_parseFromJSON() failed [secondary_rat_usage_info]");
        goto end;
    }

    secondary_rat_usage_infoList = OpenAPI_list_create();

    cJSON_ArrayForEach(secondary_rat_usage_info_local_nonprimitive, secondary_rat_usage_info ) {
        if (!cJSON_IsObject(secondary_rat_usage_info_local_nonprimitive)) {
            ogs_error("OpenAPI_release_data_parseFromJSON() failed [secondary_rat_usage_info]");
            goto end;
        }
        OpenAPI_secondary_rat_usage_info_t *secondary_rat_usage_infoItem = OpenAPI_secondary_rat_usage_info_parseFromJSON(secondary_rat_usage_info_local_nonprimitive);

        if (!secondary_rat_usage_infoItem) {
            ogs_error("No secondary_rat_usage_infoItem");
            OpenAPI_list_free(secondary_rat_usage_infoList);
            goto end;
        }

        OpenAPI_list_add(secondary_rat_usage_infoList, secondary_rat_usage_infoItem);
    }
    }

    cJSON *n4_info = cJSON_GetObjectItemCaseSensitive(release_dataJSON, "n4Info");

    OpenAPI_n4_information_t *n4_info_local_nonprim = NULL;
    if (n4_info) {
    n4_info_local_nonprim = OpenAPI_n4_information_parseFromJSON(n4_info);
    }

    cJSON *n4_info_ext1 = cJSON_GetObjectItemCaseSensitive(release_dataJSON, "n4InfoExt1");

    OpenAPI_n4_information_t *n4_info_ext1_local_nonprim = NULL;
    if (n4_info_ext1) {
    n4_info_ext1_local_nonprim = OpenAPI_n4_information_parseFromJSON(n4_info_ext1);
    }

    cJSON *n4_info_ext2 = cJSON_GetObjectItemCaseSensitive(release_dataJSON, "n4InfoExt2");

    OpenAPI_n4_information_t *n4_info_ext2_local_nonprim = NULL;
    if (n4_info_ext2) {
    n4_info_ext2_local_nonprim = OpenAPI_n4_information_parseFromJSON(n4_info_ext2);
    }

    release_data_local_var = OpenAPI_release_data_create (
        cause ? causeVariable : 0,
        ng_ap_cause ? ng_ap_cause_local_nonprim : NULL,
        _5g_mm_cause_value ? true : false,
        _5g_mm_cause_value ? _5g_mm_cause_value->valuedouble : 0,
        ue_location ? ue_location_local_nonprim : NULL,
        ue_time_zone ? ogs_strdup(ue_time_zone->valuestring) : NULL,
        add_ue_location ? add_ue_location_local_nonprim : NULL,
        secondary_rat_usage_report ? secondary_rat_usage_reportList : NULL,
        secondary_rat_usage_info ? secondary_rat_usage_infoList : NULL,
        n4_info ? n4_info_local_nonprim : NULL,
        n4_info_ext1 ? n4_info_ext1_local_nonprim : NULL,
        n4_info_ext2 ? n4_info_ext2_local_nonprim : NULL
    );

    return release_data_local_var;
end:
    return NULL;
}

OpenAPI_release_data_t *OpenAPI_release_data_copy(OpenAPI_release_data_t *dst, OpenAPI_release_data_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_release_data_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_release_data_convertToJSON() failed");
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

    OpenAPI_release_data_free(dst);
    dst = OpenAPI_release_data_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

