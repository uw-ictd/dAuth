
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "snssai_info.h"

OpenAPI_snssai_info_t *OpenAPI_snssai_info_create(
    OpenAPI_list_t *dnn_infos
)
{
    OpenAPI_snssai_info_t *snssai_info_local_var = ogs_malloc(sizeof(OpenAPI_snssai_info_t));
    ogs_assert(snssai_info_local_var);

    snssai_info_local_var->dnn_infos = dnn_infos;

    return snssai_info_local_var;
}

void OpenAPI_snssai_info_free(OpenAPI_snssai_info_t *snssai_info)
{
    if (NULL == snssai_info) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_list_for_each(snssai_info->dnn_infos, node) {
        OpenAPI_dnn_info_free(node->data);
    }
    OpenAPI_list_free(snssai_info->dnn_infos);
    ogs_free(snssai_info);
}

cJSON *OpenAPI_snssai_info_convertToJSON(OpenAPI_snssai_info_t *snssai_info)
{
    cJSON *item = NULL;

    if (snssai_info == NULL) {
        ogs_error("OpenAPI_snssai_info_convertToJSON() failed [SnssaiInfo]");
        return NULL;
    }

    item = cJSON_CreateObject();
    cJSON *dnn_infosList = cJSON_AddArrayToObject(item, "dnnInfos");
    if (dnn_infosList == NULL) {
        ogs_error("OpenAPI_snssai_info_convertToJSON() failed [dnn_infos]");
        goto end;
    }

    OpenAPI_lnode_t *dnn_infos_node;
    if (snssai_info->dnn_infos) {
        OpenAPI_list_for_each(snssai_info->dnn_infos, dnn_infos_node) {
            cJSON *itemLocal = OpenAPI_dnn_info_convertToJSON(dnn_infos_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_snssai_info_convertToJSON() failed [dnn_infos]");
                goto end;
            }
            cJSON_AddItemToArray(dnn_infosList, itemLocal);
        }
    }

end:
    return item;
}

OpenAPI_snssai_info_t *OpenAPI_snssai_info_parseFromJSON(cJSON *snssai_infoJSON)
{
    OpenAPI_snssai_info_t *snssai_info_local_var = NULL;
    cJSON *dnn_infos = cJSON_GetObjectItemCaseSensitive(snssai_infoJSON, "dnnInfos");
    if (!dnn_infos) {
        ogs_error("OpenAPI_snssai_info_parseFromJSON() failed [dnn_infos]");
        goto end;
    }

    OpenAPI_list_t *dnn_infosList;
    cJSON *dnn_infos_local_nonprimitive;
    if (!cJSON_IsArray(dnn_infos)){
        ogs_error("OpenAPI_snssai_info_parseFromJSON() failed [dnn_infos]");
        goto end;
    }

    dnn_infosList = OpenAPI_list_create();

    cJSON_ArrayForEach(dnn_infos_local_nonprimitive, dnn_infos ) {
        if (!cJSON_IsObject(dnn_infos_local_nonprimitive)) {
            ogs_error("OpenAPI_snssai_info_parseFromJSON() failed [dnn_infos]");
            goto end;
        }
        OpenAPI_dnn_info_t *dnn_infosItem = OpenAPI_dnn_info_parseFromJSON(dnn_infos_local_nonprimitive);

        if (!dnn_infosItem) {
            ogs_error("No dnn_infosItem");
            OpenAPI_list_free(dnn_infosList);
            goto end;
        }

        OpenAPI_list_add(dnn_infosList, dnn_infosItem);
    }

    snssai_info_local_var = OpenAPI_snssai_info_create (
        dnn_infosList
    );

    return snssai_info_local_var;
end:
    return NULL;
}

OpenAPI_snssai_info_t *OpenAPI_snssai_info_copy(OpenAPI_snssai_info_t *dst, OpenAPI_snssai_info_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_snssai_info_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_snssai_info_convertToJSON() failed");
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

    OpenAPI_snssai_info_free(dst);
    dst = OpenAPI_snssai_info_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

