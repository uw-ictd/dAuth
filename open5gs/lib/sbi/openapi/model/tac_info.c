
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "tac_info.h"

OpenAPI_tac_info_t *OpenAPI_tac_info_create(
    OpenAPI_list_t *tac_list
)
{
    OpenAPI_tac_info_t *tac_info_local_var = ogs_malloc(sizeof(OpenAPI_tac_info_t));
    ogs_assert(tac_info_local_var);

    tac_info_local_var->tac_list = tac_list;

    return tac_info_local_var;
}

void OpenAPI_tac_info_free(OpenAPI_tac_info_t *tac_info)
{
    if (NULL == tac_info) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_list_for_each(tac_info->tac_list, node) {
        ogs_free(node->data);
    }
    OpenAPI_list_free(tac_info->tac_list);
    ogs_free(tac_info);
}

cJSON *OpenAPI_tac_info_convertToJSON(OpenAPI_tac_info_t *tac_info)
{
    cJSON *item = NULL;

    if (tac_info == NULL) {
        ogs_error("OpenAPI_tac_info_convertToJSON() failed [TacInfo]");
        return NULL;
    }

    item = cJSON_CreateObject();
    cJSON *tac_list = cJSON_AddArrayToObject(item, "tacList");
    if (tac_list == NULL) {
        ogs_error("OpenAPI_tac_info_convertToJSON() failed [tac_list]");
        goto end;
    }

    OpenAPI_lnode_t *tac_list_node;
    OpenAPI_list_for_each(tac_info->tac_list, tac_list_node)  {
    if (cJSON_AddStringToObject(tac_list, "", (char*)tac_list_node->data) == NULL) {
        ogs_error("OpenAPI_tac_info_convertToJSON() failed [tac_list]");
        goto end;
    }
                    }

end:
    return item;
}

OpenAPI_tac_info_t *OpenAPI_tac_info_parseFromJSON(cJSON *tac_infoJSON)
{
    OpenAPI_tac_info_t *tac_info_local_var = NULL;
    cJSON *tac_list = cJSON_GetObjectItemCaseSensitive(tac_infoJSON, "tacList");
    if (!tac_list) {
        ogs_error("OpenAPI_tac_info_parseFromJSON() failed [tac_list]");
        goto end;
    }

    OpenAPI_list_t *tac_listList;
    cJSON *tac_list_local;
    if (!cJSON_IsArray(tac_list)) {
        ogs_error("OpenAPI_tac_info_parseFromJSON() failed [tac_list]");
        goto end;
    }
    tac_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(tac_list_local, tac_list) {
    if (!cJSON_IsString(tac_list_local)) {
        ogs_error("OpenAPI_tac_info_parseFromJSON() failed [tac_list]");
        goto end;
    }
    OpenAPI_list_add(tac_listList , ogs_strdup(tac_list_local->valuestring));
    }

    tac_info_local_var = OpenAPI_tac_info_create (
        tac_listList
    );

    return tac_info_local_var;
end:
    return NULL;
}

OpenAPI_tac_info_t *OpenAPI_tac_info_copy(OpenAPI_tac_info_t *dst, OpenAPI_tac_info_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_tac_info_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_tac_info_convertToJSON() failed");
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

    OpenAPI_tac_info_free(dst);
    dst = OpenAPI_tac_info_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

