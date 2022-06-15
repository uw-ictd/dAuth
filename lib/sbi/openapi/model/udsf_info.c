
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "udsf_info.h"

OpenAPI_udsf_info_t *OpenAPI_udsf_info_create(
    char *group_id,
    OpenAPI_list_t *supi_ranges,
    OpenAPI_list_t* storage_id_ranges
)
{
    OpenAPI_udsf_info_t *udsf_info_local_var = ogs_malloc(sizeof(OpenAPI_udsf_info_t));
    ogs_assert(udsf_info_local_var);

    udsf_info_local_var->group_id = group_id;
    udsf_info_local_var->supi_ranges = supi_ranges;
    udsf_info_local_var->storage_id_ranges = storage_id_ranges;

    return udsf_info_local_var;
}

void OpenAPI_udsf_info_free(OpenAPI_udsf_info_t *udsf_info)
{
    if (NULL == udsf_info) {
        return;
    }
    OpenAPI_lnode_t *node;
    ogs_free(udsf_info->group_id);
    OpenAPI_list_for_each(udsf_info->supi_ranges, node) {
        OpenAPI_supi_range_free(node->data);
    }
    OpenAPI_list_free(udsf_info->supi_ranges);
    OpenAPI_list_for_each(udsf_info->storage_id_ranges, node) {
        OpenAPI_map_t *localKeyValue = (OpenAPI_map_t*)node->data;
        ogs_free(localKeyValue->key);
        ogs_free(localKeyValue->value);
        ogs_free(localKeyValue);
    }
    OpenAPI_list_free(udsf_info->storage_id_ranges);
    ogs_free(udsf_info);
}

cJSON *OpenAPI_udsf_info_convertToJSON(OpenAPI_udsf_info_t *udsf_info)
{
    cJSON *item = NULL;

    if (udsf_info == NULL) {
        ogs_error("OpenAPI_udsf_info_convertToJSON() failed [UdsfInfo]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (udsf_info->group_id) {
    if (cJSON_AddStringToObject(item, "groupId", udsf_info->group_id) == NULL) {
        ogs_error("OpenAPI_udsf_info_convertToJSON() failed [group_id]");
        goto end;
    }
    }

    if (udsf_info->supi_ranges) {
    cJSON *supi_rangesList = cJSON_AddArrayToObject(item, "supiRanges");
    if (supi_rangesList == NULL) {
        ogs_error("OpenAPI_udsf_info_convertToJSON() failed [supi_ranges]");
        goto end;
    }

    OpenAPI_lnode_t *supi_ranges_node;
    if (udsf_info->supi_ranges) {
        OpenAPI_list_for_each(udsf_info->supi_ranges, supi_ranges_node) {
            cJSON *itemLocal = OpenAPI_supi_range_convertToJSON(supi_ranges_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_udsf_info_convertToJSON() failed [supi_ranges]");
                goto end;
            }
            cJSON_AddItemToArray(supi_rangesList, itemLocal);
        }
    }
    }

    if (udsf_info->storage_id_ranges) {
    cJSON *storage_id_ranges = cJSON_AddObjectToObject(item, "storageIdRanges");
    if (storage_id_ranges == NULL) {
        ogs_error("OpenAPI_udsf_info_convertToJSON() failed [storage_id_ranges]");
        goto end;
    }
    cJSON *localMapObject = storage_id_ranges;
    OpenAPI_lnode_t *storage_id_ranges_node;
    if (udsf_info->storage_id_ranges) {
        OpenAPI_list_for_each(udsf_info->storage_id_ranges, storage_id_ranges_node) {
            OpenAPI_map_t *localKeyValue = (OpenAPI_map_t*)storage_id_ranges_node->data;
            }
        }
    }

end:
    return item;
}

OpenAPI_udsf_info_t *OpenAPI_udsf_info_parseFromJSON(cJSON *udsf_infoJSON)
{
    OpenAPI_udsf_info_t *udsf_info_local_var = NULL;
    cJSON *group_id = cJSON_GetObjectItemCaseSensitive(udsf_infoJSON, "groupId");

    if (group_id) {
    if (!cJSON_IsString(group_id)) {
        ogs_error("OpenAPI_udsf_info_parseFromJSON() failed [group_id]");
        goto end;
    }
    }

    cJSON *supi_ranges = cJSON_GetObjectItemCaseSensitive(udsf_infoJSON, "supiRanges");

    OpenAPI_list_t *supi_rangesList;
    if (supi_ranges) {
    cJSON *supi_ranges_local_nonprimitive;
    if (!cJSON_IsArray(supi_ranges)){
        ogs_error("OpenAPI_udsf_info_parseFromJSON() failed [supi_ranges]");
        goto end;
    }

    supi_rangesList = OpenAPI_list_create();

    cJSON_ArrayForEach(supi_ranges_local_nonprimitive, supi_ranges ) {
        if (!cJSON_IsObject(supi_ranges_local_nonprimitive)) {
            ogs_error("OpenAPI_udsf_info_parseFromJSON() failed [supi_ranges]");
            goto end;
        }
        OpenAPI_supi_range_t *supi_rangesItem = OpenAPI_supi_range_parseFromJSON(supi_ranges_local_nonprimitive);

        if (!supi_rangesItem) {
            ogs_error("No supi_rangesItem");
            OpenAPI_list_free(supi_rangesList);
            goto end;
        }

        OpenAPI_list_add(supi_rangesList, supi_rangesItem);
    }
    }

    cJSON *storage_id_ranges = cJSON_GetObjectItemCaseSensitive(udsf_infoJSON, "storageIdRanges");

    OpenAPI_list_t *storage_id_rangesList;
    if (storage_id_ranges) {
    cJSON *storage_id_ranges_local_map;
    if (!cJSON_IsObject(storage_id_ranges)) {
        ogs_error("OpenAPI_udsf_info_parseFromJSON() failed [storage_id_ranges]");
        goto end;
    }
    storage_id_rangesList = OpenAPI_list_create();
    OpenAPI_map_t *localMapKeyPair = NULL;
    cJSON_ArrayForEach(storage_id_ranges_local_map, storage_id_ranges) {
        cJSON *localMapObject = storage_id_ranges_local_map;
        OpenAPI_list_add(storage_id_rangesList , localMapKeyPair);
    }
    }

    udsf_info_local_var = OpenAPI_udsf_info_create (
        group_id ? ogs_strdup(group_id->valuestring) : NULL,
        supi_ranges ? supi_rangesList : NULL,
        storage_id_ranges ? storage_id_rangesList : NULL
    );

    return udsf_info_local_var;
end:
    return NULL;
}

OpenAPI_udsf_info_t *OpenAPI_udsf_info_copy(OpenAPI_udsf_info_t *dst, OpenAPI_udsf_info_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_udsf_info_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_udsf_info_convertToJSON() failed");
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

    OpenAPI_udsf_info_free(dst);
    dst = OpenAPI_udsf_info_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

