
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "ue_context_transfer_req_data.h"

OpenAPI_ue_context_transfer_req_data_t *OpenAPI_ue_context_transfer_req_data_create(
    OpenAPI_transfer_reason_e reason,
    OpenAPI_access_type_e access_type,
    OpenAPI_plmn_id_t *plmn_id,
    OpenAPI_n1_message_container_t *reg_request,
    char *supported_features
)
{
    OpenAPI_ue_context_transfer_req_data_t *ue_context_transfer_req_data_local_var = ogs_malloc(sizeof(OpenAPI_ue_context_transfer_req_data_t));
    ogs_assert(ue_context_transfer_req_data_local_var);

    ue_context_transfer_req_data_local_var->reason = reason;
    ue_context_transfer_req_data_local_var->access_type = access_type;
    ue_context_transfer_req_data_local_var->plmn_id = plmn_id;
    ue_context_transfer_req_data_local_var->reg_request = reg_request;
    ue_context_transfer_req_data_local_var->supported_features = supported_features;

    return ue_context_transfer_req_data_local_var;
}

void OpenAPI_ue_context_transfer_req_data_free(OpenAPI_ue_context_transfer_req_data_t *ue_context_transfer_req_data)
{
    if (NULL == ue_context_transfer_req_data) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_plmn_id_free(ue_context_transfer_req_data->plmn_id);
    OpenAPI_n1_message_container_free(ue_context_transfer_req_data->reg_request);
    ogs_free(ue_context_transfer_req_data->supported_features);
    ogs_free(ue_context_transfer_req_data);
}

cJSON *OpenAPI_ue_context_transfer_req_data_convertToJSON(OpenAPI_ue_context_transfer_req_data_t *ue_context_transfer_req_data)
{
    cJSON *item = NULL;

    if (ue_context_transfer_req_data == NULL) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_convertToJSON() failed [UeContextTransferReqData]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (cJSON_AddStringToObject(item, "reason", OpenAPI_transfer_reason_ToString(ue_context_transfer_req_data->reason)) == NULL) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_convertToJSON() failed [reason]");
        goto end;
    }

    if (cJSON_AddStringToObject(item, "accessType", OpenAPI_access_type_ToString(ue_context_transfer_req_data->access_type)) == NULL) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_convertToJSON() failed [access_type]");
        goto end;
    }

    if (ue_context_transfer_req_data->plmn_id) {
    cJSON *plmn_id_local_JSON = OpenAPI_plmn_id_convertToJSON(ue_context_transfer_req_data->plmn_id);
    if (plmn_id_local_JSON == NULL) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_convertToJSON() failed [plmn_id]");
        goto end;
    }
    cJSON_AddItemToObject(item, "plmnId", plmn_id_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_convertToJSON() failed [plmn_id]");
        goto end;
    }
    }

    if (ue_context_transfer_req_data->reg_request) {
    cJSON *reg_request_local_JSON = OpenAPI_n1_message_container_convertToJSON(ue_context_transfer_req_data->reg_request);
    if (reg_request_local_JSON == NULL) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_convertToJSON() failed [reg_request]");
        goto end;
    }
    cJSON_AddItemToObject(item, "regRequest", reg_request_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_convertToJSON() failed [reg_request]");
        goto end;
    }
    }

    if (ue_context_transfer_req_data->supported_features) {
    if (cJSON_AddStringToObject(item, "supportedFeatures", ue_context_transfer_req_data->supported_features) == NULL) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_convertToJSON() failed [supported_features]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_ue_context_transfer_req_data_t *OpenAPI_ue_context_transfer_req_data_parseFromJSON(cJSON *ue_context_transfer_req_dataJSON)
{
    OpenAPI_ue_context_transfer_req_data_t *ue_context_transfer_req_data_local_var = NULL;
    cJSON *reason = cJSON_GetObjectItemCaseSensitive(ue_context_transfer_req_dataJSON, "reason");
    if (!reason) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_parseFromJSON() failed [reason]");
        goto end;
    }

    OpenAPI_transfer_reason_e reasonVariable;
    if (!cJSON_IsString(reason)) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_parseFromJSON() failed [reason]");
        goto end;
    }
    reasonVariable = OpenAPI_transfer_reason_FromString(reason->valuestring);

    cJSON *access_type = cJSON_GetObjectItemCaseSensitive(ue_context_transfer_req_dataJSON, "accessType");
    if (!access_type) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_parseFromJSON() failed [access_type]");
        goto end;
    }

    OpenAPI_access_type_e access_typeVariable;
    if (!cJSON_IsString(access_type)) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_parseFromJSON() failed [access_type]");
        goto end;
    }
    access_typeVariable = OpenAPI_access_type_FromString(access_type->valuestring);

    cJSON *plmn_id = cJSON_GetObjectItemCaseSensitive(ue_context_transfer_req_dataJSON, "plmnId");

    OpenAPI_plmn_id_t *plmn_id_local_nonprim = NULL;
    if (plmn_id) {
    plmn_id_local_nonprim = OpenAPI_plmn_id_parseFromJSON(plmn_id);
    }

    cJSON *reg_request = cJSON_GetObjectItemCaseSensitive(ue_context_transfer_req_dataJSON, "regRequest");

    OpenAPI_n1_message_container_t *reg_request_local_nonprim = NULL;
    if (reg_request) {
    reg_request_local_nonprim = OpenAPI_n1_message_container_parseFromJSON(reg_request);
    }

    cJSON *supported_features = cJSON_GetObjectItemCaseSensitive(ue_context_transfer_req_dataJSON, "supportedFeatures");

    if (supported_features) {
    if (!cJSON_IsString(supported_features)) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_parseFromJSON() failed [supported_features]");
        goto end;
    }
    }

    ue_context_transfer_req_data_local_var = OpenAPI_ue_context_transfer_req_data_create (
        reasonVariable,
        access_typeVariable,
        plmn_id ? plmn_id_local_nonprim : NULL,
        reg_request ? reg_request_local_nonprim : NULL,
        supported_features ? ogs_strdup(supported_features->valuestring) : NULL
    );

    return ue_context_transfer_req_data_local_var;
end:
    return NULL;
}

OpenAPI_ue_context_transfer_req_data_t *OpenAPI_ue_context_transfer_req_data_copy(OpenAPI_ue_context_transfer_req_data_t *dst, OpenAPI_ue_context_transfer_req_data_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_ue_context_transfer_req_data_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_ue_context_transfer_req_data_convertToJSON() failed");
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

    OpenAPI_ue_context_transfer_req_data_free(dst);
    dst = OpenAPI_ue_context_transfer_req_data_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

