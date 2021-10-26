/*
 * multicast_access_control_any_of.h
 *
 * 
 */

#ifndef _OpenAPI_multicast_access_control_any_of_H_
#define _OpenAPI_multicast_access_control_any_of_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum { OpenAPI_multicast_access_control_any_of_NULL = 0, OpenAPI_multicast_access_control_any_of_ALLOWED, OpenAPI_multicast_access_control_any_of_NOT_ALLOWED } OpenAPI_multicast_access_control_any_of_e;

char* OpenAPI_multicast_access_control_any_of_ToString(OpenAPI_multicast_access_control_any_of_e multicast_access_control_any_of);

OpenAPI_multicast_access_control_any_of_e OpenAPI_multicast_access_control_any_of_FromString(char* multicast_access_control_any_of);

#ifdef __cplusplus
}
#endif

#endif /* _OpenAPI_multicast_access_control_any_of_H_ */

