
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "session_rule.h"

OpenAPI_session_rule_t *OpenAPI_session_rule_create(
    OpenAPI_ambr_t *auth_sess_ambr,
    OpenAPI_authorized_default_qos_t *auth_def_qos,
    char *sess_rule_id,
    char *ref_um_data,
    char *ref_um_n3g_data,
    char *ref_cond_data
)
{
    OpenAPI_session_rule_t *session_rule_local_var = ogs_malloc(sizeof(OpenAPI_session_rule_t));
    ogs_assert(session_rule_local_var);

    session_rule_local_var->auth_sess_ambr = auth_sess_ambr;
    session_rule_local_var->auth_def_qos = auth_def_qos;
    session_rule_local_var->sess_rule_id = sess_rule_id;
    session_rule_local_var->ref_um_data = ref_um_data;
    session_rule_local_var->ref_um_n3g_data = ref_um_n3g_data;
    session_rule_local_var->ref_cond_data = ref_cond_data;

    return session_rule_local_var;
}

void OpenAPI_session_rule_free(OpenAPI_session_rule_t *session_rule)
{
    if (NULL == session_rule) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_ambr_free(session_rule->auth_sess_ambr);
    OpenAPI_authorized_default_qos_free(session_rule->auth_def_qos);
    ogs_free(session_rule->sess_rule_id);
    ogs_free(session_rule->ref_um_data);
    ogs_free(session_rule->ref_um_n3g_data);
    ogs_free(session_rule->ref_cond_data);
    ogs_free(session_rule);
}

cJSON *OpenAPI_session_rule_convertToJSON(OpenAPI_session_rule_t *session_rule)
{
    cJSON *item = NULL;

    if (session_rule == NULL) {
        ogs_error("OpenAPI_session_rule_convertToJSON() failed [SessionRule]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (session_rule->auth_sess_ambr) {
    cJSON *auth_sess_ambr_local_JSON = OpenAPI_ambr_convertToJSON(session_rule->auth_sess_ambr);
    if (auth_sess_ambr_local_JSON == NULL) {
        ogs_error("OpenAPI_session_rule_convertToJSON() failed [auth_sess_ambr]");
        goto end;
    }
    cJSON_AddItemToObject(item, "authSessAmbr", auth_sess_ambr_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_session_rule_convertToJSON() failed [auth_sess_ambr]");
        goto end;
    }
    }

    if (session_rule->auth_def_qos) {
    cJSON *auth_def_qos_local_JSON = OpenAPI_authorized_default_qos_convertToJSON(session_rule->auth_def_qos);
    if (auth_def_qos_local_JSON == NULL) {
        ogs_error("OpenAPI_session_rule_convertToJSON() failed [auth_def_qos]");
        goto end;
    }
    cJSON_AddItemToObject(item, "authDefQos", auth_def_qos_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_session_rule_convertToJSON() failed [auth_def_qos]");
        goto end;
    }
    }

    if (cJSON_AddStringToObject(item, "sessRuleId", session_rule->sess_rule_id) == NULL) {
        ogs_error("OpenAPI_session_rule_convertToJSON() failed [sess_rule_id]");
        goto end;
    }

    if (session_rule->ref_um_data) {
    if (cJSON_AddStringToObject(item, "refUmData", session_rule->ref_um_data) == NULL) {
        ogs_error("OpenAPI_session_rule_convertToJSON() failed [ref_um_data]");
        goto end;
    }
    }

    if (session_rule->ref_um_n3g_data) {
    if (cJSON_AddStringToObject(item, "refUmN3gData", session_rule->ref_um_n3g_data) == NULL) {
        ogs_error("OpenAPI_session_rule_convertToJSON() failed [ref_um_n3g_data]");
        goto end;
    }
    }

    if (session_rule->ref_cond_data) {
    if (cJSON_AddStringToObject(item, "refCondData", session_rule->ref_cond_data) == NULL) {
        ogs_error("OpenAPI_session_rule_convertToJSON() failed [ref_cond_data]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_session_rule_t *OpenAPI_session_rule_parseFromJSON(cJSON *session_ruleJSON)
{
    OpenAPI_session_rule_t *session_rule_local_var = NULL;
    cJSON *auth_sess_ambr = cJSON_GetObjectItemCaseSensitive(session_ruleJSON, "authSessAmbr");

    OpenAPI_ambr_t *auth_sess_ambr_local_nonprim = NULL;
    if (auth_sess_ambr) {
    auth_sess_ambr_local_nonprim = OpenAPI_ambr_parseFromJSON(auth_sess_ambr);
    }

    cJSON *auth_def_qos = cJSON_GetObjectItemCaseSensitive(session_ruleJSON, "authDefQos");

    OpenAPI_authorized_default_qos_t *auth_def_qos_local_nonprim = NULL;
    if (auth_def_qos) {
    auth_def_qos_local_nonprim = OpenAPI_authorized_default_qos_parseFromJSON(auth_def_qos);
    }

    cJSON *sess_rule_id = cJSON_GetObjectItemCaseSensitive(session_ruleJSON, "sessRuleId");
    if (!sess_rule_id) {
        ogs_error("OpenAPI_session_rule_parseFromJSON() failed [sess_rule_id]");
        goto end;
    }

    if (!cJSON_IsString(sess_rule_id)) {
        ogs_error("OpenAPI_session_rule_parseFromJSON() failed [sess_rule_id]");
        goto end;
    }

    cJSON *ref_um_data = cJSON_GetObjectItemCaseSensitive(session_ruleJSON, "refUmData");

    if (ref_um_data) {
    if (!cJSON_IsString(ref_um_data)) {
        ogs_error("OpenAPI_session_rule_parseFromJSON() failed [ref_um_data]");
        goto end;
    }
    }

    cJSON *ref_um_n3g_data = cJSON_GetObjectItemCaseSensitive(session_ruleJSON, "refUmN3gData");

    if (ref_um_n3g_data) {
    if (!cJSON_IsString(ref_um_n3g_data)) {
        ogs_error("OpenAPI_session_rule_parseFromJSON() failed [ref_um_n3g_data]");
        goto end;
    }
    }

    cJSON *ref_cond_data = cJSON_GetObjectItemCaseSensitive(session_ruleJSON, "refCondData");

    if (ref_cond_data) {
    if (!cJSON_IsString(ref_cond_data)) {
        ogs_error("OpenAPI_session_rule_parseFromJSON() failed [ref_cond_data]");
        goto end;
    }
    }

    session_rule_local_var = OpenAPI_session_rule_create (
        auth_sess_ambr ? auth_sess_ambr_local_nonprim : NULL,
        auth_def_qos ? auth_def_qos_local_nonprim : NULL,
        ogs_strdup(sess_rule_id->valuestring),
        ref_um_data ? ogs_strdup(ref_um_data->valuestring) : NULL,
        ref_um_n3g_data ? ogs_strdup(ref_um_n3g_data->valuestring) : NULL,
        ref_cond_data ? ogs_strdup(ref_cond_data->valuestring) : NULL
    );

    return session_rule_local_var;
end:
    return NULL;
}

OpenAPI_session_rule_t *OpenAPI_session_rule_copy(OpenAPI_session_rule_t *dst, OpenAPI_session_rule_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_session_rule_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_session_rule_convertToJSON() failed");
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

    OpenAPI_session_rule_free(dst);
    dst = OpenAPI_session_rule_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

