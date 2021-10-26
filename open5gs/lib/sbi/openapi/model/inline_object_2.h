/*
 * inline_object_2.h
 *
 * 
 */

#ifndef _OpenAPI_inline_object_2_H_
#define _OpenAPI_inline_object_2_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "ue_context_cancel_relocate_data.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OpenAPI_inline_object_2_s OpenAPI_inline_object_2_t;
typedef struct OpenAPI_inline_object_2_s {
    struct OpenAPI_ue_context_cancel_relocate_data_s *json_data;
    OpenAPI_binary_t* binary_data_gtpc_message;
} OpenAPI_inline_object_2_t;

OpenAPI_inline_object_2_t *OpenAPI_inline_object_2_create(
    OpenAPI_ue_context_cancel_relocate_data_t *json_data,
    OpenAPI_binary_t* binary_data_gtpc_message
);
void OpenAPI_inline_object_2_free(OpenAPI_inline_object_2_t *inline_object_2);
OpenAPI_inline_object_2_t *OpenAPI_inline_object_2_parseFromJSON(cJSON *inline_object_2JSON);
cJSON *OpenAPI_inline_object_2_convertToJSON(OpenAPI_inline_object_2_t *inline_object_2);
OpenAPI_inline_object_2_t *OpenAPI_inline_object_2_copy(OpenAPI_inline_object_2_t *dst, OpenAPI_inline_object_2_t *src);

#ifdef __cplusplus
}
#endif

#endif /* _OpenAPI_inline_object_2_H_ */

