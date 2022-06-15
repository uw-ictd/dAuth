
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "preemption_control_information_rm.h"

OpenAPI_preemption_control_information_rm_t *OpenAPI_preemption_control_information_rm_create(
)
{
    OpenAPI_preemption_control_information_rm_t *preemption_control_information_rm_local_var = ogs_malloc(sizeof(OpenAPI_preemption_control_information_rm_t));
    ogs_assert(preemption_control_information_rm_local_var);


    return preemption_control_information_rm_local_var;
}

void OpenAPI_preemption_control_information_rm_free(OpenAPI_preemption_control_information_rm_t *preemption_control_information_rm)
{
    if (NULL == preemption_control_information_rm) {
        return;
    }
    OpenAPI_lnode_t *node;
    ogs_free(preemption_control_information_rm);
}

cJSON *OpenAPI_preemption_control_information_rm_convertToJSON(OpenAPI_preemption_control_information_rm_t *preemption_control_information_rm)
{
    cJSON *item = NULL;

    if (preemption_control_information_rm == NULL) {
        ogs_error("OpenAPI_preemption_control_information_rm_convertToJSON() failed [PreemptionControlInformationRm]");
        return NULL;
    }

    item = cJSON_CreateObject();
end:
    return item;
}

OpenAPI_preemption_control_information_rm_t *OpenAPI_preemption_control_information_rm_parseFromJSON(cJSON *preemption_control_information_rmJSON)
{
    OpenAPI_preemption_control_information_rm_t *preemption_control_information_rm_local_var = NULL;
    preemption_control_information_rm_local_var = OpenAPI_preemption_control_information_rm_create (
    );

    return preemption_control_information_rm_local_var;
end:
    return NULL;
}

OpenAPI_preemption_control_information_rm_t *OpenAPI_preemption_control_information_rm_copy(OpenAPI_preemption_control_information_rm_t *dst, OpenAPI_preemption_control_information_rm_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_preemption_control_information_rm_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_preemption_control_information_rm_convertToJSON() failed");
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

    OpenAPI_preemption_control_information_rm_free(dst);
    dst = OpenAPI_preemption_control_information_rm_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

