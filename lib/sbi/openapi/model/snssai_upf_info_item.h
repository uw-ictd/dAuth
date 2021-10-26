/*
 * snssai_upf_info_item.h
 *
 * Set of parameters supported by UPF for a given S-NSSAI
 */

#ifndef _OpenAPI_snssai_upf_info_item_H_
#define _OpenAPI_snssai_upf_info_item_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "dnn_upf_info_item.h"
#include "snssai.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OpenAPI_snssai_upf_info_item_s OpenAPI_snssai_upf_info_item_t;
typedef struct OpenAPI_snssai_upf_info_item_s {
    struct OpenAPI_snssai_s *s_nssai;
    OpenAPI_list_t *dnn_upf_info_list;
    bool is_redundant_transport;
    int redundant_transport;
} OpenAPI_snssai_upf_info_item_t;

OpenAPI_snssai_upf_info_item_t *OpenAPI_snssai_upf_info_item_create(
    OpenAPI_snssai_t *s_nssai,
    OpenAPI_list_t *dnn_upf_info_list,
    bool is_redundant_transport,
    int redundant_transport
);
void OpenAPI_snssai_upf_info_item_free(OpenAPI_snssai_upf_info_item_t *snssai_upf_info_item);
OpenAPI_snssai_upf_info_item_t *OpenAPI_snssai_upf_info_item_parseFromJSON(cJSON *snssai_upf_info_itemJSON);
cJSON *OpenAPI_snssai_upf_info_item_convertToJSON(OpenAPI_snssai_upf_info_item_t *snssai_upf_info_item);
OpenAPI_snssai_upf_info_item_t *OpenAPI_snssai_upf_info_item_copy(OpenAPI_snssai_upf_info_item_t *dst, OpenAPI_snssai_upf_info_item_t *src);

#ifdef __cplusplus
}
#endif

#endif /* _OpenAPI_snssai_upf_info_item_H_ */

