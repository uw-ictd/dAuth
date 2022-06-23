
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "sm_policy_data.h"

OpenAPI_sm_policy_data_t *OpenAPI_sm_policy_data_create(
    OpenAPI_list_t* sm_policy_snssai_data,
    OpenAPI_list_t* um_data_limits,
    OpenAPI_list_t* um_data,
    char *supp_feat
)
{
    OpenAPI_sm_policy_data_t *sm_policy_data_local_var = ogs_malloc(sizeof(OpenAPI_sm_policy_data_t));
    ogs_assert(sm_policy_data_local_var);

    sm_policy_data_local_var->sm_policy_snssai_data = sm_policy_snssai_data;
    sm_policy_data_local_var->um_data_limits = um_data_limits;
    sm_policy_data_local_var->um_data = um_data;
    sm_policy_data_local_var->supp_feat = supp_feat;

    return sm_policy_data_local_var;
}

void OpenAPI_sm_policy_data_free(OpenAPI_sm_policy_data_t *sm_policy_data)
{
    if (NULL == sm_policy_data) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_list_for_each(sm_policy_data->sm_policy_snssai_data, node) {
        OpenAPI_map_t *localKeyValue = (OpenAPI_map_t*)node->data;
        ogs_free(localKeyValue->key);
        OpenAPI_sm_policy_snssai_data_free(localKeyValue->value);
        ogs_free(localKeyValue);
    }
    OpenAPI_list_free(sm_policy_data->sm_policy_snssai_data);
    OpenAPI_list_for_each(sm_policy_data->um_data_limits, node) {
        OpenAPI_map_t *localKeyValue = (OpenAPI_map_t*)node->data;
        ogs_free(localKeyValue->key);
        OpenAPI_usage_mon_data_limit_free(localKeyValue->value);
        ogs_free(localKeyValue);
    }
    OpenAPI_list_free(sm_policy_data->um_data_limits);
    OpenAPI_list_for_each(sm_policy_data->um_data, node) {
        OpenAPI_map_t *localKeyValue = (OpenAPI_map_t*)node->data;
        ogs_free(localKeyValue->key);
        OpenAPI_usage_mon_data_free(localKeyValue->value);
        ogs_free(localKeyValue);
    }
    OpenAPI_list_free(sm_policy_data->um_data);
    ogs_free(sm_policy_data->supp_feat);
    ogs_free(sm_policy_data);
}

cJSON *OpenAPI_sm_policy_data_convertToJSON(OpenAPI_sm_policy_data_t *sm_policy_data)
{
    cJSON *item = NULL;

    if (sm_policy_data == NULL) {
        ogs_error("OpenAPI_sm_policy_data_convertToJSON() failed [SmPolicyData]");
        return NULL;
    }

    item = cJSON_CreateObject();
    cJSON *sm_policy_snssai_data = cJSON_AddObjectToObject(item, "smPolicySnssaiData");
    if (sm_policy_snssai_data == NULL) {
        ogs_error("OpenAPI_sm_policy_data_convertToJSON() failed [sm_policy_snssai_data]");
        goto end;
    }
    cJSON *localMapObject = sm_policy_snssai_data;
    OpenAPI_lnode_t *sm_policy_snssai_data_node;
    if (sm_policy_data->sm_policy_snssai_data) {
        OpenAPI_list_for_each(sm_policy_data->sm_policy_snssai_data, sm_policy_snssai_data_node) {
            OpenAPI_map_t *localKeyValue = (OpenAPI_map_t*)sm_policy_snssai_data_node->data;
        cJSON *itemLocal = localKeyValue->value ?
            OpenAPI_sm_policy_snssai_data_convertToJSON(localKeyValue->value) :
            cJSON_CreateNull();
        if (itemLocal == NULL) {
            ogs_error("OpenAPI_sm_policy_data_convertToJSON() failed [sm_policy_snssai_data]");
            goto end;
        }
        cJSON_AddItemToObject(sm_policy_snssai_data, localKeyValue->key, itemLocal);
            }
        }

    if (sm_policy_data->um_data_limits) {
    cJSON *um_data_limits = cJSON_AddObjectToObject(item, "umDataLimits");
    if (um_data_limits == NULL) {
        ogs_error("OpenAPI_sm_policy_data_convertToJSON() failed [um_data_limits]");
        goto end;
    }
    cJSON *localMapObject = um_data_limits;
    OpenAPI_lnode_t *um_data_limits_node;
    if (sm_policy_data->um_data_limits) {
        OpenAPI_list_for_each(sm_policy_data->um_data_limits, um_data_limits_node) {
            OpenAPI_map_t *localKeyValue = (OpenAPI_map_t*)um_data_limits_node->data;
        cJSON *itemLocal = localKeyValue->value ?
            OpenAPI_usage_mon_data_limit_convertToJSON(localKeyValue->value) :
            cJSON_CreateNull();
        if (itemLocal == NULL) {
            ogs_error("OpenAPI_sm_policy_data_convertToJSON() failed [um_data_limits]");
            goto end;
        }
        cJSON_AddItemToObject(um_data_limits, localKeyValue->key, itemLocal);
            }
        }
    }

    if (sm_policy_data->um_data) {
    cJSON *um_data = cJSON_AddObjectToObject(item, "umData");
    if (um_data == NULL) {
        ogs_error("OpenAPI_sm_policy_data_convertToJSON() failed [um_data]");
        goto end;
    }
    cJSON *localMapObject = um_data;
    OpenAPI_lnode_t *um_data_node;
    if (sm_policy_data->um_data) {
        OpenAPI_list_for_each(sm_policy_data->um_data, um_data_node) {
            OpenAPI_map_t *localKeyValue = (OpenAPI_map_t*)um_data_node->data;
        cJSON *itemLocal = localKeyValue->value ?
            OpenAPI_usage_mon_data_convertToJSON(localKeyValue->value) :
            cJSON_CreateNull();
        if (itemLocal == NULL) {
            ogs_error("OpenAPI_sm_policy_data_convertToJSON() failed [um_data]");
            goto end;
        }
        cJSON_AddItemToObject(um_data, localKeyValue->key, itemLocal);
            }
        }
    }

    if (sm_policy_data->supp_feat) {
    if (cJSON_AddStringToObject(item, "suppFeat", sm_policy_data->supp_feat) == NULL) {
        ogs_error("OpenAPI_sm_policy_data_convertToJSON() failed [supp_feat]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_sm_policy_data_t *OpenAPI_sm_policy_data_parseFromJSON(cJSON *sm_policy_dataJSON)
{
    OpenAPI_sm_policy_data_t *sm_policy_data_local_var = NULL;
    cJSON *sm_policy_snssai_data = cJSON_GetObjectItemCaseSensitive(sm_policy_dataJSON, "smPolicySnssaiData");
    if (!sm_policy_snssai_data) {
        ogs_error("OpenAPI_sm_policy_data_parseFromJSON() failed [sm_policy_snssai_data]");
        goto end;
    }

    OpenAPI_list_t *sm_policy_snssai_dataList;
    cJSON *sm_policy_snssai_data_local_map;
    if (!cJSON_IsObject(sm_policy_snssai_data)) {
        ogs_error("OpenAPI_sm_policy_data_parseFromJSON() failed [sm_policy_snssai_data]");
        goto end;
    }
    sm_policy_snssai_dataList = OpenAPI_list_create();
    OpenAPI_map_t *localMapKeyPair = NULL;
    cJSON_ArrayForEach(sm_policy_snssai_data_local_map, sm_policy_snssai_data) {
        cJSON *localMapObject = sm_policy_snssai_data_local_map;
        if (cJSON_IsObject(sm_policy_snssai_data_local_map)) {
            localMapKeyPair = OpenAPI_map_create(
                ogs_strdup(localMapObject->string), OpenAPI_sm_policy_snssai_data_parseFromJSON(localMapObject));
        } else if (cJSON_IsNull(sm_policy_snssai_data_local_map)) {
            localMapKeyPair = OpenAPI_map_create(ogs_strdup(localMapObject->string), NULL);
        } else {
            ogs_error("OpenAPI_sm_policy_data_parseFromJSON() failed [sm_policy_snssai_data]");
            goto end;
        }
        OpenAPI_list_add(sm_policy_snssai_dataList , localMapKeyPair);
    }

    cJSON *um_data_limits = cJSON_GetObjectItemCaseSensitive(sm_policy_dataJSON, "umDataLimits");

    OpenAPI_list_t *um_data_limitsList;
    if (um_data_limits) {
    cJSON *um_data_limits_local_map;
    if (!cJSON_IsObject(um_data_limits)) {
        ogs_error("OpenAPI_sm_policy_data_parseFromJSON() failed [um_data_limits]");
        goto end;
    }
    um_data_limitsList = OpenAPI_list_create();
    OpenAPI_map_t *localMapKeyPair = NULL;
    cJSON_ArrayForEach(um_data_limits_local_map, um_data_limits) {
        cJSON *localMapObject = um_data_limits_local_map;
        if (cJSON_IsObject(um_data_limits_local_map)) {
            localMapKeyPair = OpenAPI_map_create(
                ogs_strdup(localMapObject->string), OpenAPI_usage_mon_data_limit_parseFromJSON(localMapObject));
        } else if (cJSON_IsNull(um_data_limits_local_map)) {
            localMapKeyPair = OpenAPI_map_create(ogs_strdup(localMapObject->string), NULL);
        } else {
            ogs_error("OpenAPI_sm_policy_data_parseFromJSON() failed [um_data_limits]");
            goto end;
        }
        OpenAPI_list_add(um_data_limitsList , localMapKeyPair);
    }
    }

    cJSON *um_data = cJSON_GetObjectItemCaseSensitive(sm_policy_dataJSON, "umData");

    OpenAPI_list_t *um_dataList;
    if (um_data) {
    cJSON *um_data_local_map;
    if (!cJSON_IsObject(um_data)) {
        ogs_error("OpenAPI_sm_policy_data_parseFromJSON() failed [um_data]");
        goto end;
    }
    um_dataList = OpenAPI_list_create();
    OpenAPI_map_t *localMapKeyPair = NULL;
    cJSON_ArrayForEach(um_data_local_map, um_data) {
        cJSON *localMapObject = um_data_local_map;
        if (cJSON_IsObject(um_data_local_map)) {
            localMapKeyPair = OpenAPI_map_create(
                ogs_strdup(localMapObject->string), OpenAPI_usage_mon_data_parseFromJSON(localMapObject));
        } else if (cJSON_IsNull(um_data_local_map)) {
            localMapKeyPair = OpenAPI_map_create(ogs_strdup(localMapObject->string), NULL);
        } else {
            ogs_error("OpenAPI_sm_policy_data_parseFromJSON() failed [um_data]");
            goto end;
        }
        OpenAPI_list_add(um_dataList , localMapKeyPair);
    }
    }

    cJSON *supp_feat = cJSON_GetObjectItemCaseSensitive(sm_policy_dataJSON, "suppFeat");

    if (supp_feat) {
    if (!cJSON_IsString(supp_feat)) {
        ogs_error("OpenAPI_sm_policy_data_parseFromJSON() failed [supp_feat]");
        goto end;
    }
    }

    sm_policy_data_local_var = OpenAPI_sm_policy_data_create (
        sm_policy_snssai_dataList,
        um_data_limits ? um_data_limitsList : NULL,
        um_data ? um_dataList : NULL,
        supp_feat ? ogs_strdup(supp_feat->valuestring) : NULL
    );

    return sm_policy_data_local_var;
end:
    return NULL;
}

OpenAPI_sm_policy_data_t *OpenAPI_sm_policy_data_copy(OpenAPI_sm_policy_data_t *dst, OpenAPI_sm_policy_data_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_sm_policy_data_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_sm_policy_data_convertToJSON() failed");
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

    OpenAPI_sm_policy_data_free(dst);
    dst = OpenAPI_sm_policy_data_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

