
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "amf_event_mode.h"

OpenAPI_amf_event_mode_t *OpenAPI_amf_event_mode_create(
    OpenAPI_amf_event_trigger_t *trigger,
    bool is_max_reports,
    int max_reports,
    char *expiry,
    bool is_rep_period,
    int rep_period,
    bool is_samp_ratio,
    int samp_ratio
)
{
    OpenAPI_amf_event_mode_t *amf_event_mode_local_var = ogs_malloc(sizeof(OpenAPI_amf_event_mode_t));
    ogs_assert(amf_event_mode_local_var);

    amf_event_mode_local_var->trigger = trigger;
    amf_event_mode_local_var->is_max_reports = is_max_reports;
    amf_event_mode_local_var->max_reports = max_reports;
    amf_event_mode_local_var->expiry = expiry;
    amf_event_mode_local_var->is_rep_period = is_rep_period;
    amf_event_mode_local_var->rep_period = rep_period;
    amf_event_mode_local_var->is_samp_ratio = is_samp_ratio;
    amf_event_mode_local_var->samp_ratio = samp_ratio;

    return amf_event_mode_local_var;
}

void OpenAPI_amf_event_mode_free(OpenAPI_amf_event_mode_t *amf_event_mode)
{
    if (NULL == amf_event_mode) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_amf_event_trigger_free(amf_event_mode->trigger);
    ogs_free(amf_event_mode->expiry);
    ogs_free(amf_event_mode);
}

cJSON *OpenAPI_amf_event_mode_convertToJSON(OpenAPI_amf_event_mode_t *amf_event_mode)
{
    cJSON *item = NULL;

    if (amf_event_mode == NULL) {
        ogs_error("OpenAPI_amf_event_mode_convertToJSON() failed [AmfEventMode]");
        return NULL;
    }

    item = cJSON_CreateObject();
    cJSON *trigger_local_JSON = OpenAPI_amf_event_trigger_convertToJSON(amf_event_mode->trigger);
    if (trigger_local_JSON == NULL) {
        ogs_error("OpenAPI_amf_event_mode_convertToJSON() failed [trigger]");
        goto end;
    }
    cJSON_AddItemToObject(item, "trigger", trigger_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_amf_event_mode_convertToJSON() failed [trigger]");
        goto end;
    }

    if (amf_event_mode->is_max_reports) {
    if (cJSON_AddNumberToObject(item, "maxReports", amf_event_mode->max_reports) == NULL) {
        ogs_error("OpenAPI_amf_event_mode_convertToJSON() failed [max_reports]");
        goto end;
    }
    }

    if (amf_event_mode->expiry) {
    if (cJSON_AddStringToObject(item, "expiry", amf_event_mode->expiry) == NULL) {
        ogs_error("OpenAPI_amf_event_mode_convertToJSON() failed [expiry]");
        goto end;
    }
    }

    if (amf_event_mode->is_rep_period) {
    if (cJSON_AddNumberToObject(item, "repPeriod", amf_event_mode->rep_period) == NULL) {
        ogs_error("OpenAPI_amf_event_mode_convertToJSON() failed [rep_period]");
        goto end;
    }
    }

    if (amf_event_mode->is_samp_ratio) {
    if (cJSON_AddNumberToObject(item, "sampRatio", amf_event_mode->samp_ratio) == NULL) {
        ogs_error("OpenAPI_amf_event_mode_convertToJSON() failed [samp_ratio]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_amf_event_mode_t *OpenAPI_amf_event_mode_parseFromJSON(cJSON *amf_event_modeJSON)
{
    OpenAPI_amf_event_mode_t *amf_event_mode_local_var = NULL;
    cJSON *trigger = cJSON_GetObjectItemCaseSensitive(amf_event_modeJSON, "trigger");
    if (!trigger) {
        ogs_error("OpenAPI_amf_event_mode_parseFromJSON() failed [trigger]");
        goto end;
    }

    OpenAPI_amf_event_trigger_t *trigger_local_nonprim = NULL;
    trigger_local_nonprim = OpenAPI_amf_event_trigger_parseFromJSON(trigger);

    cJSON *max_reports = cJSON_GetObjectItemCaseSensitive(amf_event_modeJSON, "maxReports");

    if (max_reports) {
    if (!cJSON_IsNumber(max_reports)) {
        ogs_error("OpenAPI_amf_event_mode_parseFromJSON() failed [max_reports]");
        goto end;
    }
    }

    cJSON *expiry = cJSON_GetObjectItemCaseSensitive(amf_event_modeJSON, "expiry");

    if (expiry) {
    if (!cJSON_IsString(expiry)) {
        ogs_error("OpenAPI_amf_event_mode_parseFromJSON() failed [expiry]");
        goto end;
    }
    }

    cJSON *rep_period = cJSON_GetObjectItemCaseSensitive(amf_event_modeJSON, "repPeriod");

    if (rep_period) {
    if (!cJSON_IsNumber(rep_period)) {
        ogs_error("OpenAPI_amf_event_mode_parseFromJSON() failed [rep_period]");
        goto end;
    }
    }

    cJSON *samp_ratio = cJSON_GetObjectItemCaseSensitive(amf_event_modeJSON, "sampRatio");

    if (samp_ratio) {
    if (!cJSON_IsNumber(samp_ratio)) {
        ogs_error("OpenAPI_amf_event_mode_parseFromJSON() failed [samp_ratio]");
        goto end;
    }
    }

    amf_event_mode_local_var = OpenAPI_amf_event_mode_create (
        trigger_local_nonprim,
        max_reports ? true : false,
        max_reports ? max_reports->valuedouble : 0,
        expiry ? ogs_strdup(expiry->valuestring) : NULL,
        rep_period ? true : false,
        rep_period ? rep_period->valuedouble : 0,
        samp_ratio ? true : false,
        samp_ratio ? samp_ratio->valuedouble : 0
    );

    return amf_event_mode_local_var;
end:
    return NULL;
}

OpenAPI_amf_event_mode_t *OpenAPI_amf_event_mode_copy(OpenAPI_amf_event_mode_t *dst, OpenAPI_amf_event_mode_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_amf_event_mode_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_amf_event_mode_convertToJSON() failed");
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

    OpenAPI_amf_event_mode_free(dst);
    dst = OpenAPI_amf_event_mode_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

