
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "slice_info_for_pdu_session.h"

OpenAPI_slice_info_for_pdu_session_t *OpenAPI_slice_info_for_pdu_session_create(
    OpenAPI_snssai_t *s_nssai,
    OpenAPI_roaming_indication_e roaming_indication,
    OpenAPI_snssai_t *home_snssai
)
{
    OpenAPI_slice_info_for_pdu_session_t *slice_info_for_pdu_session_local_var = ogs_malloc(sizeof(OpenAPI_slice_info_for_pdu_session_t));
    ogs_assert(slice_info_for_pdu_session_local_var);

    slice_info_for_pdu_session_local_var->s_nssai = s_nssai;
    slice_info_for_pdu_session_local_var->roaming_indication = roaming_indication;
    slice_info_for_pdu_session_local_var->home_snssai = home_snssai;

    return slice_info_for_pdu_session_local_var;
}

void OpenAPI_slice_info_for_pdu_session_free(OpenAPI_slice_info_for_pdu_session_t *slice_info_for_pdu_session)
{
    if (NULL == slice_info_for_pdu_session) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_snssai_free(slice_info_for_pdu_session->s_nssai);
    OpenAPI_snssai_free(slice_info_for_pdu_session->home_snssai);
    ogs_free(slice_info_for_pdu_session);
}

cJSON *OpenAPI_slice_info_for_pdu_session_convertToJSON(OpenAPI_slice_info_for_pdu_session_t *slice_info_for_pdu_session)
{
    cJSON *item = NULL;

    if (slice_info_for_pdu_session == NULL) {
        ogs_error("OpenAPI_slice_info_for_pdu_session_convertToJSON() failed [SliceInfoForPDUSession]");
        return NULL;
    }

    item = cJSON_CreateObject();
    cJSON *s_nssai_local_JSON = OpenAPI_snssai_convertToJSON(slice_info_for_pdu_session->s_nssai);
    if (s_nssai_local_JSON == NULL) {
        ogs_error("OpenAPI_slice_info_for_pdu_session_convertToJSON() failed [s_nssai]");
        goto end;
    }
    cJSON_AddItemToObject(item, "sNssai", s_nssai_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_slice_info_for_pdu_session_convertToJSON() failed [s_nssai]");
        goto end;
    }

    if (cJSON_AddStringToObject(item, "roamingIndication", OpenAPI_roaming_indication_ToString(slice_info_for_pdu_session->roaming_indication)) == NULL) {
        ogs_error("OpenAPI_slice_info_for_pdu_session_convertToJSON() failed [roaming_indication]");
        goto end;
    }

    if (slice_info_for_pdu_session->home_snssai) {
    cJSON *home_snssai_local_JSON = OpenAPI_snssai_convertToJSON(slice_info_for_pdu_session->home_snssai);
    if (home_snssai_local_JSON == NULL) {
        ogs_error("OpenAPI_slice_info_for_pdu_session_convertToJSON() failed [home_snssai]");
        goto end;
    }
    cJSON_AddItemToObject(item, "homeSnssai", home_snssai_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_slice_info_for_pdu_session_convertToJSON() failed [home_snssai]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_slice_info_for_pdu_session_t *OpenAPI_slice_info_for_pdu_session_parseFromJSON(cJSON *slice_info_for_pdu_sessionJSON)
{
    OpenAPI_slice_info_for_pdu_session_t *slice_info_for_pdu_session_local_var = NULL;
    cJSON *s_nssai = cJSON_GetObjectItemCaseSensitive(slice_info_for_pdu_sessionJSON, "sNssai");
    if (!s_nssai) {
        ogs_error("OpenAPI_slice_info_for_pdu_session_parseFromJSON() failed [s_nssai]");
        goto end;
    }

    OpenAPI_snssai_t *s_nssai_local_nonprim = NULL;
    s_nssai_local_nonprim = OpenAPI_snssai_parseFromJSON(s_nssai);

    cJSON *roaming_indication = cJSON_GetObjectItemCaseSensitive(slice_info_for_pdu_sessionJSON, "roamingIndication");
    if (!roaming_indication) {
        ogs_error("OpenAPI_slice_info_for_pdu_session_parseFromJSON() failed [roaming_indication]");
        goto end;
    }

    OpenAPI_roaming_indication_e roaming_indicationVariable;
    if (!cJSON_IsString(roaming_indication)) {
        ogs_error("OpenAPI_slice_info_for_pdu_session_parseFromJSON() failed [roaming_indication]");
        goto end;
    }
    roaming_indicationVariable = OpenAPI_roaming_indication_FromString(roaming_indication->valuestring);

    cJSON *home_snssai = cJSON_GetObjectItemCaseSensitive(slice_info_for_pdu_sessionJSON, "homeSnssai");

    OpenAPI_snssai_t *home_snssai_local_nonprim = NULL;
    if (home_snssai) {
    home_snssai_local_nonprim = OpenAPI_snssai_parseFromJSON(home_snssai);
    }

    slice_info_for_pdu_session_local_var = OpenAPI_slice_info_for_pdu_session_create (
        s_nssai_local_nonprim,
        roaming_indicationVariable,
        home_snssai ? home_snssai_local_nonprim : NULL
    );

    return slice_info_for_pdu_session_local_var;
end:
    return NULL;
}

OpenAPI_slice_info_for_pdu_session_t *OpenAPI_slice_info_for_pdu_session_copy(OpenAPI_slice_info_for_pdu_session_t *dst, OpenAPI_slice_info_for_pdu_session_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_slice_info_for_pdu_session_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_slice_info_for_pdu_session_convertToJSON() failed");
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

    OpenAPI_slice_info_for_pdu_session_free(dst);
    dst = OpenAPI_slice_info_for_pdu_session_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

