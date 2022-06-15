
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "application_data_change_notif.h"

OpenAPI_application_data_change_notif_t *OpenAPI_application_data_change_notif_create(
    OpenAPI_iptv_config_data_t *iptv_config_data,
    OpenAPI_pfd_change_notification_t *pfd_data,
    OpenAPI_bdt_policy_data_t *bdt_policy_data,
    char *res_uri,
    OpenAPI_service_parameter_data_t *ser_param_data
)
{
    OpenAPI_application_data_change_notif_t *application_data_change_notif_local_var = ogs_malloc(sizeof(OpenAPI_application_data_change_notif_t));
    ogs_assert(application_data_change_notif_local_var);

    application_data_change_notif_local_var->iptv_config_data = iptv_config_data;
    application_data_change_notif_local_var->pfd_data = pfd_data;
    application_data_change_notif_local_var->bdt_policy_data = bdt_policy_data;
    application_data_change_notif_local_var->res_uri = res_uri;
    application_data_change_notif_local_var->ser_param_data = ser_param_data;

    return application_data_change_notif_local_var;
}

void OpenAPI_application_data_change_notif_free(OpenAPI_application_data_change_notif_t *application_data_change_notif)
{
    if (NULL == application_data_change_notif) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_iptv_config_data_free(application_data_change_notif->iptv_config_data);
    OpenAPI_pfd_change_notification_free(application_data_change_notif->pfd_data);
    OpenAPI_bdt_policy_data_free(application_data_change_notif->bdt_policy_data);
    ogs_free(application_data_change_notif->res_uri);
    OpenAPI_service_parameter_data_free(application_data_change_notif->ser_param_data);
    ogs_free(application_data_change_notif);
}

cJSON *OpenAPI_application_data_change_notif_convertToJSON(OpenAPI_application_data_change_notif_t *application_data_change_notif)
{
    cJSON *item = NULL;

    if (application_data_change_notif == NULL) {
        ogs_error("OpenAPI_application_data_change_notif_convertToJSON() failed [ApplicationDataChangeNotif]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (application_data_change_notif->iptv_config_data) {
    cJSON *iptv_config_data_local_JSON = OpenAPI_iptv_config_data_convertToJSON(application_data_change_notif->iptv_config_data);
    if (iptv_config_data_local_JSON == NULL) {
        ogs_error("OpenAPI_application_data_change_notif_convertToJSON() failed [iptv_config_data]");
        goto end;
    }
    cJSON_AddItemToObject(item, "iptvConfigData", iptv_config_data_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_application_data_change_notif_convertToJSON() failed [iptv_config_data]");
        goto end;
    }
    }

    if (application_data_change_notif->pfd_data) {
    cJSON *pfd_data_local_JSON = OpenAPI_pfd_change_notification_convertToJSON(application_data_change_notif->pfd_data);
    if (pfd_data_local_JSON == NULL) {
        ogs_error("OpenAPI_application_data_change_notif_convertToJSON() failed [pfd_data]");
        goto end;
    }
    cJSON_AddItemToObject(item, "pfdData", pfd_data_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_application_data_change_notif_convertToJSON() failed [pfd_data]");
        goto end;
    }
    }

    if (application_data_change_notif->bdt_policy_data) {
    cJSON *bdt_policy_data_local_JSON = OpenAPI_bdt_policy_data_convertToJSON(application_data_change_notif->bdt_policy_data);
    if (bdt_policy_data_local_JSON == NULL) {
        ogs_error("OpenAPI_application_data_change_notif_convertToJSON() failed [bdt_policy_data]");
        goto end;
    }
    cJSON_AddItemToObject(item, "bdtPolicyData", bdt_policy_data_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_application_data_change_notif_convertToJSON() failed [bdt_policy_data]");
        goto end;
    }
    }

    if (cJSON_AddStringToObject(item, "resUri", application_data_change_notif->res_uri) == NULL) {
        ogs_error("OpenAPI_application_data_change_notif_convertToJSON() failed [res_uri]");
        goto end;
    }

    if (application_data_change_notif->ser_param_data) {
    cJSON *ser_param_data_local_JSON = OpenAPI_service_parameter_data_convertToJSON(application_data_change_notif->ser_param_data);
    if (ser_param_data_local_JSON == NULL) {
        ogs_error("OpenAPI_application_data_change_notif_convertToJSON() failed [ser_param_data]");
        goto end;
    }
    cJSON_AddItemToObject(item, "serParamData", ser_param_data_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_application_data_change_notif_convertToJSON() failed [ser_param_data]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_application_data_change_notif_t *OpenAPI_application_data_change_notif_parseFromJSON(cJSON *application_data_change_notifJSON)
{
    OpenAPI_application_data_change_notif_t *application_data_change_notif_local_var = NULL;
    cJSON *iptv_config_data = cJSON_GetObjectItemCaseSensitive(application_data_change_notifJSON, "iptvConfigData");

    OpenAPI_iptv_config_data_t *iptv_config_data_local_nonprim = NULL;
    if (iptv_config_data) {
    iptv_config_data_local_nonprim = OpenAPI_iptv_config_data_parseFromJSON(iptv_config_data);
    }

    cJSON *pfd_data = cJSON_GetObjectItemCaseSensitive(application_data_change_notifJSON, "pfdData");

    OpenAPI_pfd_change_notification_t *pfd_data_local_nonprim = NULL;
    if (pfd_data) {
    pfd_data_local_nonprim = OpenAPI_pfd_change_notification_parseFromJSON(pfd_data);
    }

    cJSON *bdt_policy_data = cJSON_GetObjectItemCaseSensitive(application_data_change_notifJSON, "bdtPolicyData");

    OpenAPI_bdt_policy_data_t *bdt_policy_data_local_nonprim = NULL;
    if (bdt_policy_data) {
    bdt_policy_data_local_nonprim = OpenAPI_bdt_policy_data_parseFromJSON(bdt_policy_data);
    }

    cJSON *res_uri = cJSON_GetObjectItemCaseSensitive(application_data_change_notifJSON, "resUri");
    if (!res_uri) {
        ogs_error("OpenAPI_application_data_change_notif_parseFromJSON() failed [res_uri]");
        goto end;
    }

    if (!cJSON_IsString(res_uri)) {
        ogs_error("OpenAPI_application_data_change_notif_parseFromJSON() failed [res_uri]");
        goto end;
    }

    cJSON *ser_param_data = cJSON_GetObjectItemCaseSensitive(application_data_change_notifJSON, "serParamData");

    OpenAPI_service_parameter_data_t *ser_param_data_local_nonprim = NULL;
    if (ser_param_data) {
    ser_param_data_local_nonprim = OpenAPI_service_parameter_data_parseFromJSON(ser_param_data);
    }

    application_data_change_notif_local_var = OpenAPI_application_data_change_notif_create (
        iptv_config_data ? iptv_config_data_local_nonprim : NULL,
        pfd_data ? pfd_data_local_nonprim : NULL,
        bdt_policy_data ? bdt_policy_data_local_nonprim : NULL,
        ogs_strdup(res_uri->valuestring),
        ser_param_data ? ser_param_data_local_nonprim : NULL
    );

    return application_data_change_notif_local_var;
end:
    return NULL;
}

OpenAPI_application_data_change_notif_t *OpenAPI_application_data_change_notif_copy(OpenAPI_application_data_change_notif_t *dst, OpenAPI_application_data_change_notif_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_application_data_change_notif_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_application_data_change_notif_convertToJSON() failed");
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

    OpenAPI_application_data_change_notif_free(dst);
    dst = OpenAPI_application_data_change_notif_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

