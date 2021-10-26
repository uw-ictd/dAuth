/*
 * inline_object_4.h
 *
 * 
 */

#ifndef _OpenAPI_inline_object_4_H_
#define _OpenAPI_inline_object_4_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "n2_information_notification.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OpenAPI_inline_object_4_s OpenAPI_inline_object_4_t;
typedef struct OpenAPI_inline_object_4_s {
    struct OpenAPI_n2_information_notification_s *json_data;
    OpenAPI_binary_t* binary_data_n1_message;
    OpenAPI_binary_t* binary_data_n2_information;
} OpenAPI_inline_object_4_t;

OpenAPI_inline_object_4_t *OpenAPI_inline_object_4_create(
    OpenAPI_n2_information_notification_t *json_data,
    OpenAPI_binary_t* binary_data_n1_message,
    OpenAPI_binary_t* binary_data_n2_information
);
void OpenAPI_inline_object_4_free(OpenAPI_inline_object_4_t *inline_object_4);
OpenAPI_inline_object_4_t *OpenAPI_inline_object_4_parseFromJSON(cJSON *inline_object_4JSON);
cJSON *OpenAPI_inline_object_4_convertToJSON(OpenAPI_inline_object_4_t *inline_object_4);
OpenAPI_inline_object_4_t *OpenAPI_inline_object_4_copy(OpenAPI_inline_object_4_t *dst, OpenAPI_inline_object_4_t *src);

#ifdef __cplusplus
}
#endif

#endif /* _OpenAPI_inline_object_4_H_ */

