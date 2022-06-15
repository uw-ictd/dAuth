
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "tngf_info.h"

OpenAPI_tngf_info_t *OpenAPI_tngf_info_create(
    OpenAPI_list_t *ipv4_endpoint_addresses,
    OpenAPI_list_t *ipv6_endpoint_addresses,
    char *endpoint_fqdn
)
{
    OpenAPI_tngf_info_t *tngf_info_local_var = ogs_malloc(sizeof(OpenAPI_tngf_info_t));
    ogs_assert(tngf_info_local_var);

    tngf_info_local_var->ipv4_endpoint_addresses = ipv4_endpoint_addresses;
    tngf_info_local_var->ipv6_endpoint_addresses = ipv6_endpoint_addresses;
    tngf_info_local_var->endpoint_fqdn = endpoint_fqdn;

    return tngf_info_local_var;
}

void OpenAPI_tngf_info_free(OpenAPI_tngf_info_t *tngf_info)
{
    if (NULL == tngf_info) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_list_for_each(tngf_info->ipv4_endpoint_addresses, node) {
        ogs_free(node->data);
    }
    OpenAPI_list_free(tngf_info->ipv4_endpoint_addresses);
    OpenAPI_list_for_each(tngf_info->ipv6_endpoint_addresses, node) {
        ogs_free(node->data);
    }
    OpenAPI_list_free(tngf_info->ipv6_endpoint_addresses);
    ogs_free(tngf_info->endpoint_fqdn);
    ogs_free(tngf_info);
}

cJSON *OpenAPI_tngf_info_convertToJSON(OpenAPI_tngf_info_t *tngf_info)
{
    cJSON *item = NULL;

    if (tngf_info == NULL) {
        ogs_error("OpenAPI_tngf_info_convertToJSON() failed [TngfInfo]");
        return NULL;
    }

    item = cJSON_CreateObject();
    if (tngf_info->ipv4_endpoint_addresses) {
    cJSON *ipv4_endpoint_addresses = cJSON_AddArrayToObject(item, "ipv4EndpointAddresses");
    if (ipv4_endpoint_addresses == NULL) {
        ogs_error("OpenAPI_tngf_info_convertToJSON() failed [ipv4_endpoint_addresses]");
        goto end;
    }

    OpenAPI_lnode_t *ipv4_endpoint_addresses_node;
    OpenAPI_list_for_each(tngf_info->ipv4_endpoint_addresses, ipv4_endpoint_addresses_node)  {
    if (cJSON_AddStringToObject(ipv4_endpoint_addresses, "", (char*)ipv4_endpoint_addresses_node->data) == NULL) {
        ogs_error("OpenAPI_tngf_info_convertToJSON() failed [ipv4_endpoint_addresses]");
        goto end;
    }
                    }
    }

    if (tngf_info->ipv6_endpoint_addresses) {
    cJSON *ipv6_endpoint_addresses = cJSON_AddArrayToObject(item, "ipv6EndpointAddresses");
    if (ipv6_endpoint_addresses == NULL) {
        ogs_error("OpenAPI_tngf_info_convertToJSON() failed [ipv6_endpoint_addresses]");
        goto end;
    }

    OpenAPI_lnode_t *ipv6_endpoint_addresses_node;
    OpenAPI_list_for_each(tngf_info->ipv6_endpoint_addresses, ipv6_endpoint_addresses_node)  {
    if (cJSON_AddStringToObject(ipv6_endpoint_addresses, "", (char*)ipv6_endpoint_addresses_node->data) == NULL) {
        ogs_error("OpenAPI_tngf_info_convertToJSON() failed [ipv6_endpoint_addresses]");
        goto end;
    }
                    }
    }

    if (tngf_info->endpoint_fqdn) {
    if (cJSON_AddStringToObject(item, "endpointFqdn", tngf_info->endpoint_fqdn) == NULL) {
        ogs_error("OpenAPI_tngf_info_convertToJSON() failed [endpoint_fqdn]");
        goto end;
    }
    }

end:
    return item;
}

OpenAPI_tngf_info_t *OpenAPI_tngf_info_parseFromJSON(cJSON *tngf_infoJSON)
{
    OpenAPI_tngf_info_t *tngf_info_local_var = NULL;
    cJSON *ipv4_endpoint_addresses = cJSON_GetObjectItemCaseSensitive(tngf_infoJSON, "ipv4EndpointAddresses");

    OpenAPI_list_t *ipv4_endpoint_addressesList;
    if (ipv4_endpoint_addresses) {
    cJSON *ipv4_endpoint_addresses_local;
    if (!cJSON_IsArray(ipv4_endpoint_addresses)) {
        ogs_error("OpenAPI_tngf_info_parseFromJSON() failed [ipv4_endpoint_addresses]");
        goto end;
    }
    ipv4_endpoint_addressesList = OpenAPI_list_create();

    cJSON_ArrayForEach(ipv4_endpoint_addresses_local, ipv4_endpoint_addresses) {
    if (!cJSON_IsString(ipv4_endpoint_addresses_local)) {
        ogs_error("OpenAPI_tngf_info_parseFromJSON() failed [ipv4_endpoint_addresses]");
        goto end;
    }
    OpenAPI_list_add(ipv4_endpoint_addressesList , ogs_strdup(ipv4_endpoint_addresses_local->valuestring));
    }
    }

    cJSON *ipv6_endpoint_addresses = cJSON_GetObjectItemCaseSensitive(tngf_infoJSON, "ipv6EndpointAddresses");

    OpenAPI_list_t *ipv6_endpoint_addressesList;
    if (ipv6_endpoint_addresses) {
    cJSON *ipv6_endpoint_addresses_local;
    if (!cJSON_IsArray(ipv6_endpoint_addresses)) {
        ogs_error("OpenAPI_tngf_info_parseFromJSON() failed [ipv6_endpoint_addresses]");
        goto end;
    }
    ipv6_endpoint_addressesList = OpenAPI_list_create();

    cJSON_ArrayForEach(ipv6_endpoint_addresses_local, ipv6_endpoint_addresses) {
    if (!cJSON_IsString(ipv6_endpoint_addresses_local)) {
        ogs_error("OpenAPI_tngf_info_parseFromJSON() failed [ipv6_endpoint_addresses]");
        goto end;
    }
    OpenAPI_list_add(ipv6_endpoint_addressesList , ogs_strdup(ipv6_endpoint_addresses_local->valuestring));
    }
    }

    cJSON *endpoint_fqdn = cJSON_GetObjectItemCaseSensitive(tngf_infoJSON, "endpointFqdn");

    if (endpoint_fqdn) {
    if (!cJSON_IsString(endpoint_fqdn)) {
        ogs_error("OpenAPI_tngf_info_parseFromJSON() failed [endpoint_fqdn]");
        goto end;
    }
    }

    tngf_info_local_var = OpenAPI_tngf_info_create (
        ipv4_endpoint_addresses ? ipv4_endpoint_addressesList : NULL,
        ipv6_endpoint_addresses ? ipv6_endpoint_addressesList : NULL,
        endpoint_fqdn ? ogs_strdup(endpoint_fqdn->valuestring) : NULL
    );

    return tngf_info_local_var;
end:
    return NULL;
}

OpenAPI_tngf_info_t *OpenAPI_tngf_info_copy(OpenAPI_tngf_info_t *dst, OpenAPI_tngf_info_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_tngf_info_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_tngf_info_convertToJSON() failed");
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

    OpenAPI_tngf_info_free(dst);
    dst = OpenAPI_tngf_info_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

