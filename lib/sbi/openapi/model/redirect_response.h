/*
 * redirect_response.h
 *
 * 
 */

#ifndef _OpenAPI_redirect_response_H_
#define _OpenAPI_redirect_response_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OpenAPI_redirect_response_s OpenAPI_redirect_response_t;
typedef struct OpenAPI_redirect_response_s {
    char *cause;
    char *target_scp;
} OpenAPI_redirect_response_t;

OpenAPI_redirect_response_t *OpenAPI_redirect_response_create(
    char *cause,
    char *target_scp
);
void OpenAPI_redirect_response_free(OpenAPI_redirect_response_t *redirect_response);
OpenAPI_redirect_response_t *OpenAPI_redirect_response_parseFromJSON(cJSON *redirect_responseJSON);
cJSON *OpenAPI_redirect_response_convertToJSON(OpenAPI_redirect_response_t *redirect_response);
OpenAPI_redirect_response_t *OpenAPI_redirect_response_copy(OpenAPI_redirect_response_t *dst, OpenAPI_redirect_response_t *src);

#ifdef __cplusplus
}
#endif

#endif /* _OpenAPI_redirect_response_H_ */

