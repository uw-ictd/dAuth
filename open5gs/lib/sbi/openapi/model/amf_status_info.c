
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "amf_status_info.h"

OpenAPI_amf_status_info_t *OpenAPI_amf_status_info_create(
    OpenAPI_list_t *guami_list,
    OpenAPI_status_change_e status_change,
    char *target_amf_removal,
    char *target_amf_failure
)
{
    OpenAPI_amf_status_info_t *amf_status_info_local_var = ogs_malloc(sizeof(OpenAPI_amf_status_info_t));
    ogs_assert(amf_status_info_local_var);

    amf_status_info_local_var->guami_list = guami_list;
    amf_status_info_local_var->status_change = status_change;
    amf_status_info_local_var->target_amf_removal = target_amf_removal;
    amf_status_info_local_var->target_amf_failure = target_amf_failure;

    return amf_status_info_local_var;
}

void OpenAPI_amf_status_info_free(OpenAPI_amf_status_info_t *amf_status_info)
{
    if (NULL == amf_status_info) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_list_for_each(amf_status_info->guami_list, node) {
        OpenAPI_guami_free(node->data);
    }
    OpenAPI_list_free(amf_status_info->guami_list);
    ogs_free(amf_status_info->target_amf_removal);
    ogs_free(amf_status_info->target_amf_failure);
    ogs_free(amf_status_info);
}

cJSON *OpenAPI_amf_status_info_convertToJSON(OpenAPI_amf_status_info_t *amf_status_info)
{
    cJSON *item = NULL;

    if (amf_status_info == NULL) {
        ogs_error("OpenAPI_amf_status_info_convertToJSON() failed [AmfStatusInfo]");
        return NULL;
    }

    item = cJSON_CreateObject();
    cJSON *guami_listList = cJSON_AddArrayToObject(item, "guamiList");
    if (guami_listList == NULL) {
        ogs_error("OpenAPI_amf_status_info_convertToJSON() failed [guami_list]");
        goto end;
    }

    OpenAPI_lnode_t *guami_list_node;
    if (amf_status_info->guami_list) {
        OpenAPI_list_for_each(amf_status_info->guami_list, guami_list_node) {
            cJSON *itemLocal = OpenAPI_guami_convertToJSON(guami_list_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_amf_status_info_convertToJSON() failed [guami_list]");
                goto end;
            }
            cJSON_AddItemToArray(guami_listList, itemLocal);
        }
    }

    if (cJSON_AddStringToObject(item, "statusChange", OpenAPI_status_change_ToString(amf_status_info->status_change)) == NULL) {
        ogs_error("OpenAPI_amf_status_info_convertToJSON() failed [status_change]");
        goto end;
    }

    if (amf_status_info->target_amf_removal) {
    if (cJSON_AddStringToObject(item, "targetAmfRemoval", amf_status_info->target_amf_removal) == NULL) {
        ogs_error("OpenAPI_amf_status_info_convertToJSON() failed [target_amf_removal]");
        goto end;
    }
    }

    if (amf_status_info->target_amf_failure) {
    if (cJSON_AddStringToObject(item, "targetAmfFailure", amf_status_info->target_amf_failure) == NULL) {
        ogs_error("OpenAPI_amf_status_info_convertToJSON() failed [target_amf_failure]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_amf_status_info_t *OpenAPI_amf_status_info_parseFromJSON(cJSON *amf_status_infoJSON)
{
    OpenAPI_amf_status_info_t *amf_status_info_local_var = NULL;
    cJSON *guami_list = cJSON_GetObjectItemCaseSensitive(amf_status_infoJSON, "guamiList");
    if (!guami_list) {
        ogs_error("OpenAPI_amf_status_info_parseFromJSON() failed [guami_list]");
        goto end;
    }

    OpenAPI_list_t *guami_listList;
    cJSON *guami_list_local_nonprimitive;
    if (!cJSON_IsArray(guami_list)){
        ogs_error("OpenAPI_amf_status_info_parseFromJSON() failed [guami_list]");
        goto end;
    }

    guami_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(guami_list_local_nonprimitive, guami_list ) {
        if (!cJSON_IsObject(guami_list_local_nonprimitive)) {
            ogs_error("OpenAPI_amf_status_info_parseFromJSON() failed [guami_list]");
            goto end;
        }
        OpenAPI_guami_t *guami_listItem = OpenAPI_guami_parseFromJSON(guami_list_local_nonprimitive);

        if (!guami_listItem) {
            ogs_error("No guami_listItem");
            OpenAPI_list_free(guami_listList);
            goto end;
        }

        OpenAPI_list_add(guami_listList, guami_listItem);
    }

    cJSON *status_change = cJSON_GetObjectItemCaseSensitive(amf_status_infoJSON, "statusChange");
    if (!status_change) {
        ogs_error("OpenAPI_amf_status_info_parseFromJSON() failed [status_change]");
        goto end;
    }

    OpenAPI_status_change_e status_changeVariable;
    if (!cJSON_IsString(status_change)) {
        ogs_error("OpenAPI_amf_status_info_parseFromJSON() failed [status_change]");
        goto end;
    }
    status_changeVariable = OpenAPI_status_change_FromString(status_change->valuestring);

    cJSON *target_amf_removal = cJSON_GetObjectItemCaseSensitive(amf_status_infoJSON, "targetAmfRemoval");

    if (target_amf_removal) {
    if (!cJSON_IsString(target_amf_removal)) {
        ogs_error("OpenAPI_amf_status_info_parseFromJSON() failed [target_amf_removal]");
        goto end;
    }
    }

    cJSON *target_amf_failure = cJSON_GetObjectItemCaseSensitive(amf_status_infoJSON, "targetAmfFailure");

    if (target_amf_failure) {
    if (!cJSON_IsString(target_amf_failure)) {
        ogs_error("OpenAPI_amf_status_info_parseFromJSON() failed [target_amf_failure]");
        goto end;
    }
    }

    amf_status_info_local_var = OpenAPI_amf_status_info_create (
        guami_listList,
        status_changeVariable,
        target_amf_removal ? ogs_strdup(target_amf_removal->valuestring) : NULL,
        target_amf_failure ? ogs_strdup(target_amf_failure->valuestring) : NULL
    );

    return amf_status_info_local_var;
end:
    return NULL;
}

OpenAPI_amf_status_info_t *OpenAPI_amf_status_info_copy(OpenAPI_amf_status_info_t *dst, OpenAPI_amf_status_info_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_amf_status_info_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_amf_status_info_convertToJSON() failed");
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

    OpenAPI_amf_status_info_free(dst);
    dst = OpenAPI_amf_status_info_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

