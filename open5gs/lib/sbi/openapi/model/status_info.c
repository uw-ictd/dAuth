
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "status_info.h"

OpenAPI_status_info_t *OpenAPI_status_info_create(
    OpenAPI_resource_status_e resource_status,
    OpenAPI_cause_e cause,
    OpenAPI_cn_assisted_ran_para_t *cn_assisted_ran_para,
    OpenAPI_access_type_e an_type
)
{
    OpenAPI_status_info_t *status_info_local_var = ogs_malloc(sizeof(OpenAPI_status_info_t));
    ogs_assert(status_info_local_var);

    status_info_local_var->resource_status = resource_status;
    status_info_local_var->cause = cause;
    status_info_local_var->cn_assisted_ran_para = cn_assisted_ran_para;
    status_info_local_var->an_type = an_type;

    return status_info_local_var;
}

void OpenAPI_status_info_free(OpenAPI_status_info_t *status_info)
{
    if (NULL == status_info) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_cn_assisted_ran_para_free(status_info->cn_assisted_ran_para);
    ogs_free(status_info);
}

cJSON *OpenAPI_status_info_convertToJSON(OpenAPI_status_info_t *status_info)
{
    cJSON *item = NULL;

    if (status_info == NULL) {
        ogs_error("OpenAPI_status_info_convertToJSON() failed [StatusInfo]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (cJSON_AddStringToObject(item, "resourceStatus", OpenAPI_resource_status_ToString(status_info->resource_status)) == NULL) {
        ogs_error("OpenAPI_status_info_convertToJSON() failed [resource_status]");
        goto end;
    }

    if (status_info->cause) {
    if (cJSON_AddStringToObject(item, "cause", OpenAPI_cause_ToString(status_info->cause)) == NULL) {
        ogs_error("OpenAPI_status_info_convertToJSON() failed [cause]");
        goto end;
    }
    }

    if (status_info->cn_assisted_ran_para) {
    cJSON *cn_assisted_ran_para_local_JSON = OpenAPI_cn_assisted_ran_para_convertToJSON(status_info->cn_assisted_ran_para);
    if (cn_assisted_ran_para_local_JSON == NULL) {
        ogs_error("OpenAPI_status_info_convertToJSON() failed [cn_assisted_ran_para]");
        goto end;
    }
    cJSON_AddItemToObject(item, "cnAssistedRanPara", cn_assisted_ran_para_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_status_info_convertToJSON() failed [cn_assisted_ran_para]");
        goto end;
    }
    }

    if (status_info->an_type) {
    if (cJSON_AddStringToObject(item, "anType", OpenAPI_access_type_ToString(status_info->an_type)) == NULL) {
        ogs_error("OpenAPI_status_info_convertToJSON() failed [an_type]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_status_info_t *OpenAPI_status_info_parseFromJSON(cJSON *status_infoJSON)
{
    OpenAPI_status_info_t *status_info_local_var = NULL;
    cJSON *resource_status = cJSON_GetObjectItemCaseSensitive(status_infoJSON, "resourceStatus");
    if (!resource_status) {
        ogs_error("OpenAPI_status_info_parseFromJSON() failed [resource_status]");
        goto end;
    }

    OpenAPI_resource_status_e resource_statusVariable;
    if (!cJSON_IsString(resource_status)) {
        ogs_error("OpenAPI_status_info_parseFromJSON() failed [resource_status]");
        goto end;
    }
    resource_statusVariable = OpenAPI_resource_status_FromString(resource_status->valuestring);

    cJSON *cause = cJSON_GetObjectItemCaseSensitive(status_infoJSON, "cause");

    OpenAPI_cause_e causeVariable;
    if (cause) {
    if (!cJSON_IsString(cause)) {
        ogs_error("OpenAPI_status_info_parseFromJSON() failed [cause]");
        goto end;
    }
    causeVariable = OpenAPI_cause_FromString(cause->valuestring);
    }

    cJSON *cn_assisted_ran_para = cJSON_GetObjectItemCaseSensitive(status_infoJSON, "cnAssistedRanPara");

    OpenAPI_cn_assisted_ran_para_t *cn_assisted_ran_para_local_nonprim = NULL;
    if (cn_assisted_ran_para) {
    cn_assisted_ran_para_local_nonprim = OpenAPI_cn_assisted_ran_para_parseFromJSON(cn_assisted_ran_para);
    }

    cJSON *an_type = cJSON_GetObjectItemCaseSensitive(status_infoJSON, "anType");

    OpenAPI_access_type_e an_typeVariable;
    if (an_type) {
    if (!cJSON_IsString(an_type)) {
        ogs_error("OpenAPI_status_info_parseFromJSON() failed [an_type]");
        goto end;
    }
    an_typeVariable = OpenAPI_access_type_FromString(an_type->valuestring);
    }

    status_info_local_var = OpenAPI_status_info_create (
        resource_statusVariable,
        cause ? causeVariable : 0,
        cn_assisted_ran_para ? cn_assisted_ran_para_local_nonprim : NULL,
        an_type ? an_typeVariable : 0
    );

    return status_info_local_var;
end:
    return NULL;
}

OpenAPI_status_info_t *OpenAPI_status_info_copy(OpenAPI_status_info_t *dst, OpenAPI_status_info_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_status_info_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_status_info_convertToJSON() failed");
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

    OpenAPI_status_info_free(dst);
    dst = OpenAPI_status_info_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

