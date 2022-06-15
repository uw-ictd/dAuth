
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "presence_info_rm.h"

OpenAPI_presence_info_rm_t *OpenAPI_presence_info_rm_create(
    char *pra_id,
    char *additional_pra_id,
    OpenAPI_presence_state_e presence_state,
    OpenAPI_list_t *tracking_area_list,
    OpenAPI_list_t *ecgi_list,
    OpenAPI_list_t *ncgi_list,
    OpenAPI_list_t *global_ran_node_id_list,
    OpenAPI_list_t *globale_nb_id_list
)
{
    OpenAPI_presence_info_rm_t *presence_info_rm_local_var = ogs_malloc(sizeof(OpenAPI_presence_info_rm_t));
    ogs_assert(presence_info_rm_local_var);

    presence_info_rm_local_var->pra_id = pra_id;
    presence_info_rm_local_var->additional_pra_id = additional_pra_id;
    presence_info_rm_local_var->presence_state = presence_state;
    presence_info_rm_local_var->tracking_area_list = tracking_area_list;
    presence_info_rm_local_var->ecgi_list = ecgi_list;
    presence_info_rm_local_var->ncgi_list = ncgi_list;
    presence_info_rm_local_var->global_ran_node_id_list = global_ran_node_id_list;
    presence_info_rm_local_var->globale_nb_id_list = globale_nb_id_list;

    return presence_info_rm_local_var;
}

void OpenAPI_presence_info_rm_free(OpenAPI_presence_info_rm_t *presence_info_rm)
{
    if (NULL == presence_info_rm) {
        return;
    }
    OpenAPI_lnode_t *node;
    ogs_free(presence_info_rm->pra_id);
    ogs_free(presence_info_rm->additional_pra_id);
    OpenAPI_list_for_each(presence_info_rm->tracking_area_list, node) {
        OpenAPI_tai_free(node->data);
    }
    OpenAPI_list_free(presence_info_rm->tracking_area_list);
    OpenAPI_list_for_each(presence_info_rm->ecgi_list, node) {
        OpenAPI_ecgi_free(node->data);
    }
    OpenAPI_list_free(presence_info_rm->ecgi_list);
    OpenAPI_list_for_each(presence_info_rm->ncgi_list, node) {
        OpenAPI_ncgi_free(node->data);
    }
    OpenAPI_list_free(presence_info_rm->ncgi_list);
    OpenAPI_list_for_each(presence_info_rm->global_ran_node_id_list, node) {
        OpenAPI_global_ran_node_id_free(node->data);
    }
    OpenAPI_list_free(presence_info_rm->global_ran_node_id_list);
    OpenAPI_list_for_each(presence_info_rm->globale_nb_id_list, node) {
        OpenAPI_global_ran_node_id_free(node->data);
    }
    OpenAPI_list_free(presence_info_rm->globale_nb_id_list);
    ogs_free(presence_info_rm);
}

cJSON *OpenAPI_presence_info_rm_convertToJSON(OpenAPI_presence_info_rm_t *presence_info_rm)
{
    cJSON *item = NULL;

    if (presence_info_rm == NULL) {
        ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [PresenceInfoRm]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (presence_info_rm->pra_id) {
    if (cJSON_AddStringToObject(item, "praId", presence_info_rm->pra_id) == NULL) {
        ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [pra_id]");
        goto end;
    }
    }

    if (presence_info_rm->additional_pra_id) {
    if (cJSON_AddStringToObject(item, "additionalPraId", presence_info_rm->additional_pra_id) == NULL) {
        ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [additional_pra_id]");
        goto end;
    }
    }

    if (presence_info_rm->presence_state) {
    if (cJSON_AddStringToObject(item, "presenceState", OpenAPI_presence_state_ToString(presence_info_rm->presence_state)) == NULL) {
        ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [presence_state]");
        goto end;
    }
    }

    if (presence_info_rm->tracking_area_list) {
    cJSON *tracking_area_listList = cJSON_AddArrayToObject(item, "trackingAreaList");
    if (tracking_area_listList == NULL) {
        ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [tracking_area_list]");
        goto end;
    }

    OpenAPI_lnode_t *tracking_area_list_node;
    if (presence_info_rm->tracking_area_list) {
        OpenAPI_list_for_each(presence_info_rm->tracking_area_list, tracking_area_list_node) {
            cJSON *itemLocal = OpenAPI_tai_convertToJSON(tracking_area_list_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [tracking_area_list]");
                goto end;
            }
            cJSON_AddItemToArray(tracking_area_listList, itemLocal);
        }
    }
    }

    if (presence_info_rm->ecgi_list) {
    cJSON *ecgi_listList = cJSON_AddArrayToObject(item, "ecgiList");
    if (ecgi_listList == NULL) {
        ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [ecgi_list]");
        goto end;
    }

    OpenAPI_lnode_t *ecgi_list_node;
    if (presence_info_rm->ecgi_list) {
        OpenAPI_list_for_each(presence_info_rm->ecgi_list, ecgi_list_node) {
            cJSON *itemLocal = OpenAPI_ecgi_convertToJSON(ecgi_list_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [ecgi_list]");
                goto end;
            }
            cJSON_AddItemToArray(ecgi_listList, itemLocal);
        }
    }
    }

    if (presence_info_rm->ncgi_list) {
    cJSON *ncgi_listList = cJSON_AddArrayToObject(item, "ncgiList");
    if (ncgi_listList == NULL) {
        ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [ncgi_list]");
        goto end;
    }

    OpenAPI_lnode_t *ncgi_list_node;
    if (presence_info_rm->ncgi_list) {
        OpenAPI_list_for_each(presence_info_rm->ncgi_list, ncgi_list_node) {
            cJSON *itemLocal = OpenAPI_ncgi_convertToJSON(ncgi_list_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [ncgi_list]");
                goto end;
            }
            cJSON_AddItemToArray(ncgi_listList, itemLocal);
        }
    }
    }

    if (presence_info_rm->global_ran_node_id_list) {
    cJSON *global_ran_node_id_listList = cJSON_AddArrayToObject(item, "globalRanNodeIdList");
    if (global_ran_node_id_listList == NULL) {
        ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [global_ran_node_id_list]");
        goto end;
    }

    OpenAPI_lnode_t *global_ran_node_id_list_node;
    if (presence_info_rm->global_ran_node_id_list) {
        OpenAPI_list_for_each(presence_info_rm->global_ran_node_id_list, global_ran_node_id_list_node) {
            cJSON *itemLocal = OpenAPI_global_ran_node_id_convertToJSON(global_ran_node_id_list_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [global_ran_node_id_list]");
                goto end;
            }
            cJSON_AddItemToArray(global_ran_node_id_listList, itemLocal);
        }
    }
    }

    if (presence_info_rm->globale_nb_id_list) {
    cJSON *globale_nb_id_listList = cJSON_AddArrayToObject(item, "globaleNbIdList");
    if (globale_nb_id_listList == NULL) {
        ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [globale_nb_id_list]");
        goto end;
    }

    OpenAPI_lnode_t *globale_nb_id_list_node;
    if (presence_info_rm->globale_nb_id_list) {
        OpenAPI_list_for_each(presence_info_rm->globale_nb_id_list, globale_nb_id_list_node) {
            cJSON *itemLocal = OpenAPI_global_ran_node_id_convertToJSON(globale_nb_id_list_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed [globale_nb_id_list]");
                goto end;
            }
            cJSON_AddItemToArray(globale_nb_id_listList, itemLocal);
        }
    }
    }

end:
    return item;
}

OpenAPI_presence_info_rm_t *OpenAPI_presence_info_rm_parseFromJSON(cJSON *presence_info_rmJSON)
{
    OpenAPI_presence_info_rm_t *presence_info_rm_local_var = NULL;
    cJSON *pra_id = cJSON_GetObjectItemCaseSensitive(presence_info_rmJSON, "praId");

    if (pra_id) {
    if (!cJSON_IsString(pra_id)) {
        ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [pra_id]");
        goto end;
    }
    }

    cJSON *additional_pra_id = cJSON_GetObjectItemCaseSensitive(presence_info_rmJSON, "additionalPraId");

    if (additional_pra_id) {
    if (!cJSON_IsString(additional_pra_id)) {
        ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [additional_pra_id]");
        goto end;
    }
    }

    cJSON *presence_state = cJSON_GetObjectItemCaseSensitive(presence_info_rmJSON, "presenceState");

    OpenAPI_presence_state_e presence_stateVariable;
    if (presence_state) {
    if (!cJSON_IsString(presence_state)) {
        ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [presence_state]");
        goto end;
    }
    presence_stateVariable = OpenAPI_presence_state_FromString(presence_state->valuestring);
    }

    cJSON *tracking_area_list = cJSON_GetObjectItemCaseSensitive(presence_info_rmJSON, "trackingAreaList");

    OpenAPI_list_t *tracking_area_listList;
    if (tracking_area_list) {
    cJSON *tracking_area_list_local_nonprimitive;
    if (!cJSON_IsArray(tracking_area_list)){
        ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [tracking_area_list]");
        goto end;
    }

    tracking_area_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(tracking_area_list_local_nonprimitive, tracking_area_list ) {
        if (!cJSON_IsObject(tracking_area_list_local_nonprimitive)) {
            ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [tracking_area_list]");
            goto end;
        }
        OpenAPI_tai_t *tracking_area_listItem = OpenAPI_tai_parseFromJSON(tracking_area_list_local_nonprimitive);

        if (!tracking_area_listItem) {
            ogs_error("No tracking_area_listItem");
            OpenAPI_list_free(tracking_area_listList);
            goto end;
        }

        OpenAPI_list_add(tracking_area_listList, tracking_area_listItem);
    }
    }

    cJSON *ecgi_list = cJSON_GetObjectItemCaseSensitive(presence_info_rmJSON, "ecgiList");

    OpenAPI_list_t *ecgi_listList;
    if (ecgi_list) {
    cJSON *ecgi_list_local_nonprimitive;
    if (!cJSON_IsArray(ecgi_list)){
        ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [ecgi_list]");
        goto end;
    }

    ecgi_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(ecgi_list_local_nonprimitive, ecgi_list ) {
        if (!cJSON_IsObject(ecgi_list_local_nonprimitive)) {
            ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [ecgi_list]");
            goto end;
        }
        OpenAPI_ecgi_t *ecgi_listItem = OpenAPI_ecgi_parseFromJSON(ecgi_list_local_nonprimitive);

        if (!ecgi_listItem) {
            ogs_error("No ecgi_listItem");
            OpenAPI_list_free(ecgi_listList);
            goto end;
        }

        OpenAPI_list_add(ecgi_listList, ecgi_listItem);
    }
    }

    cJSON *ncgi_list = cJSON_GetObjectItemCaseSensitive(presence_info_rmJSON, "ncgiList");

    OpenAPI_list_t *ncgi_listList;
    if (ncgi_list) {
    cJSON *ncgi_list_local_nonprimitive;
    if (!cJSON_IsArray(ncgi_list)){
        ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [ncgi_list]");
        goto end;
    }

    ncgi_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(ncgi_list_local_nonprimitive, ncgi_list ) {
        if (!cJSON_IsObject(ncgi_list_local_nonprimitive)) {
            ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [ncgi_list]");
            goto end;
        }
        OpenAPI_ncgi_t *ncgi_listItem = OpenAPI_ncgi_parseFromJSON(ncgi_list_local_nonprimitive);

        if (!ncgi_listItem) {
            ogs_error("No ncgi_listItem");
            OpenAPI_list_free(ncgi_listList);
            goto end;
        }

        OpenAPI_list_add(ncgi_listList, ncgi_listItem);
    }
    }

    cJSON *global_ran_node_id_list = cJSON_GetObjectItemCaseSensitive(presence_info_rmJSON, "globalRanNodeIdList");

    OpenAPI_list_t *global_ran_node_id_listList;
    if (global_ran_node_id_list) {
    cJSON *global_ran_node_id_list_local_nonprimitive;
    if (!cJSON_IsArray(global_ran_node_id_list)){
        ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [global_ran_node_id_list]");
        goto end;
    }

    global_ran_node_id_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(global_ran_node_id_list_local_nonprimitive, global_ran_node_id_list ) {
        if (!cJSON_IsObject(global_ran_node_id_list_local_nonprimitive)) {
            ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [global_ran_node_id_list]");
            goto end;
        }
        OpenAPI_global_ran_node_id_t *global_ran_node_id_listItem = OpenAPI_global_ran_node_id_parseFromJSON(global_ran_node_id_list_local_nonprimitive);

        if (!global_ran_node_id_listItem) {
            ogs_error("No global_ran_node_id_listItem");
            OpenAPI_list_free(global_ran_node_id_listList);
            goto end;
        }

        OpenAPI_list_add(global_ran_node_id_listList, global_ran_node_id_listItem);
    }
    }

    cJSON *globale_nb_id_list = cJSON_GetObjectItemCaseSensitive(presence_info_rmJSON, "globaleNbIdList");

    OpenAPI_list_t *globale_nb_id_listList;
    if (globale_nb_id_list) {
    cJSON *globale_nb_id_list_local_nonprimitive;
    if (!cJSON_IsArray(globale_nb_id_list)){
        ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [globale_nb_id_list]");
        goto end;
    }

    globale_nb_id_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(globale_nb_id_list_local_nonprimitive, globale_nb_id_list ) {
        if (!cJSON_IsObject(globale_nb_id_list_local_nonprimitive)) {
            ogs_error("OpenAPI_presence_info_rm_parseFromJSON() failed [globale_nb_id_list]");
            goto end;
        }
        OpenAPI_global_ran_node_id_t *globale_nb_id_listItem = OpenAPI_global_ran_node_id_parseFromJSON(globale_nb_id_list_local_nonprimitive);

        if (!globale_nb_id_listItem) {
            ogs_error("No globale_nb_id_listItem");
            OpenAPI_list_free(globale_nb_id_listList);
            goto end;
        }

        OpenAPI_list_add(globale_nb_id_listList, globale_nb_id_listItem);
    }
    }

    presence_info_rm_local_var = OpenAPI_presence_info_rm_create (
        pra_id ? ogs_strdup(pra_id->valuestring) : NULL,
        additional_pra_id ? ogs_strdup(additional_pra_id->valuestring) : NULL,
        presence_state ? presence_stateVariable : 0,
        tracking_area_list ? tracking_area_listList : NULL,
        ecgi_list ? ecgi_listList : NULL,
        ncgi_list ? ncgi_listList : NULL,
        global_ran_node_id_list ? global_ran_node_id_listList : NULL,
        globale_nb_id_list ? globale_nb_id_listList : NULL
    );

    return presence_info_rm_local_var;
end:
    return NULL;
}

OpenAPI_presence_info_rm_t *OpenAPI_presence_info_rm_copy(OpenAPI_presence_info_rm_t *dst, OpenAPI_presence_info_rm_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_presence_info_rm_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_presence_info_rm_convertToJSON() failed");
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

    OpenAPI_presence_info_rm_free(dst);
    dst = OpenAPI_presence_info_rm_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

