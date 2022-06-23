
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "odb_data.h"

OpenAPI_odb_data_t *OpenAPI_odb_data_create(
    OpenAPI_roaming_odb_e roaming_odb
)
{
    OpenAPI_odb_data_t *odb_data_local_var = ogs_malloc(sizeof(OpenAPI_odb_data_t));
    ogs_assert(odb_data_local_var);

    odb_data_local_var->roaming_odb = roaming_odb;

    return odb_data_local_var;
}

void OpenAPI_odb_data_free(OpenAPI_odb_data_t *odb_data)
{
    if (NULL == odb_data) {
        return;
    }
    OpenAPI_lnode_t *node;
    ogs_free(odb_data);
}

cJSON *OpenAPI_odb_data_convertToJSON(OpenAPI_odb_data_t *odb_data)
{
    cJSON *item = NULL;

    if (odb_data == NULL) {
        ogs_error("OpenAPI_odb_data_convertToJSON() failed [OdbData]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (odb_data->roaming_odb) {
    if (cJSON_AddStringToObject(item, "roamingOdb", OpenAPI_roaming_odb_ToString(odb_data->roaming_odb)) == NULL) {
        ogs_error("OpenAPI_odb_data_convertToJSON() failed [roaming_odb]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_odb_data_t *OpenAPI_odb_data_parseFromJSON(cJSON *odb_dataJSON)
{
    OpenAPI_odb_data_t *odb_data_local_var = NULL;
    cJSON *roaming_odb = cJSON_GetObjectItemCaseSensitive(odb_dataJSON, "roamingOdb");

    OpenAPI_roaming_odb_e roaming_odbVariable;
    if (roaming_odb) {
    if (!cJSON_IsString(roaming_odb)) {
        ogs_error("OpenAPI_odb_data_parseFromJSON() failed [roaming_odb]");
        goto end;
    }
    roaming_odbVariable = OpenAPI_roaming_odb_FromString(roaming_odb->valuestring);
    }

    odb_data_local_var = OpenAPI_odb_data_create (
        roaming_odb ? roaming_odbVariable : 0
    );

    return odb_data_local_var;
end:
    return NULL;
}

OpenAPI_odb_data_t *OpenAPI_odb_data_copy(OpenAPI_odb_data_t *dst, OpenAPI_odb_data_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_odb_data_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_odb_data_convertToJSON() failed");
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

    OpenAPI_odb_data_free(dst);
    dst = OpenAPI_odb_data_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

