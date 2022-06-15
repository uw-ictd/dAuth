
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "qos_monitoring_data.h"

OpenAPI_qos_monitoring_data_t *OpenAPI_qos_monitoring_data_create(
    char *qm_id,
    OpenAPI_list_t *req_qos_mon_params,
    OpenAPI_list_t *rep_freqs,
    bool is_rep_thresh_dl,
    int rep_thresh_dl,
    bool is_rep_thresh_ul,
    int rep_thresh_ul,
    bool is_rep_thresh_rp,
    int rep_thresh_rp,
    bool is_wait_time,
    int wait_time,
    bool is_rep_period,
    int rep_period,
    char *notify_uri,
    char *notify_corre_id
)
{
    OpenAPI_qos_monitoring_data_t *qos_monitoring_data_local_var = ogs_malloc(sizeof(OpenAPI_qos_monitoring_data_t));
    ogs_assert(qos_monitoring_data_local_var);

    qos_monitoring_data_local_var->qm_id = qm_id;
    qos_monitoring_data_local_var->req_qos_mon_params = req_qos_mon_params;
    qos_monitoring_data_local_var->rep_freqs = rep_freqs;
    qos_monitoring_data_local_var->is_rep_thresh_dl = is_rep_thresh_dl;
    qos_monitoring_data_local_var->rep_thresh_dl = rep_thresh_dl;
    qos_monitoring_data_local_var->is_rep_thresh_ul = is_rep_thresh_ul;
    qos_monitoring_data_local_var->rep_thresh_ul = rep_thresh_ul;
    qos_monitoring_data_local_var->is_rep_thresh_rp = is_rep_thresh_rp;
    qos_monitoring_data_local_var->rep_thresh_rp = rep_thresh_rp;
    qos_monitoring_data_local_var->is_wait_time = is_wait_time;
    qos_monitoring_data_local_var->wait_time = wait_time;
    qos_monitoring_data_local_var->is_rep_period = is_rep_period;
    qos_monitoring_data_local_var->rep_period = rep_period;
    qos_monitoring_data_local_var->notify_uri = notify_uri;
    qos_monitoring_data_local_var->notify_corre_id = notify_corre_id;

    return qos_monitoring_data_local_var;
}

void OpenAPI_qos_monitoring_data_free(OpenAPI_qos_monitoring_data_t *qos_monitoring_data)
{
    if (NULL == qos_monitoring_data) {
        return;
    }
    OpenAPI_lnode_t *node;
    ogs_free(qos_monitoring_data->qm_id);
    OpenAPI_list_free(qos_monitoring_data->req_qos_mon_params);
    OpenAPI_list_free(qos_monitoring_data->rep_freqs);
    ogs_free(qos_monitoring_data->notify_uri);
    ogs_free(qos_monitoring_data->notify_corre_id);
    ogs_free(qos_monitoring_data);
}

cJSON *OpenAPI_qos_monitoring_data_convertToJSON(OpenAPI_qos_monitoring_data_t *qos_monitoring_data)
{
    cJSON *item = NULL;

    if (qos_monitoring_data == NULL) {
        ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [QosMonitoringData]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (cJSON_AddStringToObject(item, "qmId", qos_monitoring_data->qm_id) == NULL) {
        ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [qm_id]");
        goto end;
    }

    cJSON *req_qos_mon_params = cJSON_AddArrayToObject(item, "reqQosMonParams");
    if (req_qos_mon_params == NULL) {
        ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [req_qos_mon_params]");
        goto end;
    }
    OpenAPI_lnode_t *req_qos_mon_params_node;
    OpenAPI_list_for_each(qos_monitoring_data->req_qos_mon_params, req_qos_mon_params_node) {
        if (cJSON_AddStringToObject(req_qos_mon_params, "", OpenAPI_requested_qos_monitoring_parameter_ToString((intptr_t)req_qos_mon_params_node->data)) == NULL) {
            ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [req_qos_mon_params]");
            goto end;
        }
    }

    cJSON *rep_freqs = cJSON_AddArrayToObject(item, "repFreqs");
    if (rep_freqs == NULL) {
        ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [rep_freqs]");
        goto end;
    }
    OpenAPI_lnode_t *rep_freqs_node;
    OpenAPI_list_for_each(qos_monitoring_data->rep_freqs, rep_freqs_node) {
        if (cJSON_AddStringToObject(rep_freqs, "", OpenAPI_reporting_frequency_ToString((intptr_t)rep_freqs_node->data)) == NULL) {
            ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [rep_freqs]");
            goto end;
        }
    }

    if (qos_monitoring_data->is_rep_thresh_dl) {
    if (cJSON_AddNumberToObject(item, "repThreshDl", qos_monitoring_data->rep_thresh_dl) == NULL) {
        ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [rep_thresh_dl]");
        goto end;
    }
    }

    if (qos_monitoring_data->is_rep_thresh_ul) {
    if (cJSON_AddNumberToObject(item, "repThreshUl", qos_monitoring_data->rep_thresh_ul) == NULL) {
        ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [rep_thresh_ul]");
        goto end;
    }
    }

    if (qos_monitoring_data->is_rep_thresh_rp) {
    if (cJSON_AddNumberToObject(item, "repThreshRp", qos_monitoring_data->rep_thresh_rp) == NULL) {
        ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [rep_thresh_rp]");
        goto end;
    }
    }

    if (qos_monitoring_data->is_wait_time) {
    if (cJSON_AddNumberToObject(item, "waitTime", qos_monitoring_data->wait_time) == NULL) {
        ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [wait_time]");
        goto end;
    }
    }

    if (qos_monitoring_data->is_rep_period) {
    if (cJSON_AddNumberToObject(item, "repPeriod", qos_monitoring_data->rep_period) == NULL) {
        ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [rep_period]");
        goto end;
    }
    }

    if (qos_monitoring_data->notify_uri) {
    if (cJSON_AddStringToObject(item, "notifyUri", qos_monitoring_data->notify_uri) == NULL) {
        ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [notify_uri]");
        goto end;
    }
    }

    if (qos_monitoring_data->notify_corre_id) {
    if (cJSON_AddStringToObject(item, "notifyCorreId", qos_monitoring_data->notify_corre_id) == NULL) {
        ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed [notify_corre_id]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_qos_monitoring_data_t *OpenAPI_qos_monitoring_data_parseFromJSON(cJSON *qos_monitoring_dataJSON)
{
    OpenAPI_qos_monitoring_data_t *qos_monitoring_data_local_var = NULL;
    cJSON *qm_id = cJSON_GetObjectItemCaseSensitive(qos_monitoring_dataJSON, "qmId");
    if (!qm_id) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [qm_id]");
        goto end;
    }

    if (!cJSON_IsString(qm_id)) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [qm_id]");
        goto end;
    }

    cJSON *req_qos_mon_params = cJSON_GetObjectItemCaseSensitive(qos_monitoring_dataJSON, "reqQosMonParams");
    if (!req_qos_mon_params) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [req_qos_mon_params]");
        goto end;
    }

    OpenAPI_list_t *req_qos_mon_paramsList;
    cJSON *req_qos_mon_params_local_nonprimitive;
    if (!cJSON_IsArray(req_qos_mon_params)) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [req_qos_mon_params]");
        goto end;
    }

    req_qos_mon_paramsList = OpenAPI_list_create();

    cJSON_ArrayForEach(req_qos_mon_params_local_nonprimitive, req_qos_mon_params ) {
        if (!cJSON_IsString(req_qos_mon_params_local_nonprimitive)){
            ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [req_qos_mon_params]");
            goto end;
        }

        OpenAPI_list_add(req_qos_mon_paramsList, (void *)OpenAPI_requested_qos_monitoring_parameter_FromString(req_qos_mon_params_local_nonprimitive->valuestring));
    }

    cJSON *rep_freqs = cJSON_GetObjectItemCaseSensitive(qos_monitoring_dataJSON, "repFreqs");
    if (!rep_freqs) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [rep_freqs]");
        goto end;
    }

    OpenAPI_list_t *rep_freqsList;
    cJSON *rep_freqs_local_nonprimitive;
    if (!cJSON_IsArray(rep_freqs)) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [rep_freqs]");
        goto end;
    }

    rep_freqsList = OpenAPI_list_create();

    cJSON_ArrayForEach(rep_freqs_local_nonprimitive, rep_freqs ) {
        if (!cJSON_IsString(rep_freqs_local_nonprimitive)){
            ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [rep_freqs]");
            goto end;
        }

        OpenAPI_list_add(rep_freqsList, (void *)OpenAPI_reporting_frequency_FromString(rep_freqs_local_nonprimitive->valuestring));
    }

    cJSON *rep_thresh_dl = cJSON_GetObjectItemCaseSensitive(qos_monitoring_dataJSON, "repThreshDl");

    if (rep_thresh_dl) {
    if (!cJSON_IsNumber(rep_thresh_dl)) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [rep_thresh_dl]");
        goto end;
    }
    }

    cJSON *rep_thresh_ul = cJSON_GetObjectItemCaseSensitive(qos_monitoring_dataJSON, "repThreshUl");

    if (rep_thresh_ul) {
    if (!cJSON_IsNumber(rep_thresh_ul)) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [rep_thresh_ul]");
        goto end;
    }
    }

    cJSON *rep_thresh_rp = cJSON_GetObjectItemCaseSensitive(qos_monitoring_dataJSON, "repThreshRp");

    if (rep_thresh_rp) {
    if (!cJSON_IsNumber(rep_thresh_rp)) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [rep_thresh_rp]");
        goto end;
    }
    }

    cJSON *wait_time = cJSON_GetObjectItemCaseSensitive(qos_monitoring_dataJSON, "waitTime");

    if (wait_time) {
    if (!cJSON_IsNumber(wait_time)) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [wait_time]");
        goto end;
    }
    }

    cJSON *rep_period = cJSON_GetObjectItemCaseSensitive(qos_monitoring_dataJSON, "repPeriod");

    if (rep_period) {
    if (!cJSON_IsNumber(rep_period)) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [rep_period]");
        goto end;
    }
    }

    cJSON *notify_uri = cJSON_GetObjectItemCaseSensitive(qos_monitoring_dataJSON, "notifyUri");

    if (notify_uri) {
    if (!cJSON_IsString(notify_uri)) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [notify_uri]");
        goto end;
    }
    }

    cJSON *notify_corre_id = cJSON_GetObjectItemCaseSensitive(qos_monitoring_dataJSON, "notifyCorreId");

    if (notify_corre_id) {
    if (!cJSON_IsString(notify_corre_id)) {
        ogs_error("OpenAPI_qos_monitoring_data_parseFromJSON() failed [notify_corre_id]");
        goto end;
    }
    }

    qos_monitoring_data_local_var = OpenAPI_qos_monitoring_data_create (
        ogs_strdup(qm_id->valuestring),
        req_qos_mon_paramsList,
        rep_freqsList,
        rep_thresh_dl ? true : false,
        rep_thresh_dl ? rep_thresh_dl->valuedouble : 0,
        rep_thresh_ul ? true : false,
        rep_thresh_ul ? rep_thresh_ul->valuedouble : 0,
        rep_thresh_rp ? true : false,
        rep_thresh_rp ? rep_thresh_rp->valuedouble : 0,
        wait_time ? true : false,
        wait_time ? wait_time->valuedouble : 0,
        rep_period ? true : false,
        rep_period ? rep_period->valuedouble : 0,
        notify_uri ? ogs_strdup(notify_uri->valuestring) : NULL,
        notify_corre_id ? ogs_strdup(notify_corre_id->valuestring) : NULL
    );

    return qos_monitoring_data_local_var;
end:
    return NULL;
}

OpenAPI_qos_monitoring_data_t *OpenAPI_qos_monitoring_data_copy(OpenAPI_qos_monitoring_data_t *dst, OpenAPI_qos_monitoring_data_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_qos_monitoring_data_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_qos_monitoring_data_convertToJSON() failed");
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

    OpenAPI_qos_monitoring_data_free(dst);
    dst = OpenAPI_qos_monitoring_data_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

