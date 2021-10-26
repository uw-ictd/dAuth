/*
 * amf_event_type_any_of.h
 *
 * 
 */

#ifndef _OpenAPI_amf_event_type_any_of_H_
#define _OpenAPI_amf_event_type_any_of_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum { OpenAPI_amf_event_type_any_of_NULL = 0, OpenAPI_amf_event_type_any_of_LOCATION_REPORT, OpenAPI_amf_event_type_any_of_PRESENCE_IN_AOI_REPORT, OpenAPI_amf_event_type_any_of_TIMEZONE_REPORT, OpenAPI_amf_event_type_any_of_ACCESS_TYPE_REPORT, OpenAPI_amf_event_type_any_of_REGISTRATION_STATE_REPORT, OpenAPI_amf_event_type_any_of_CONNECTIVITY_STATE_REPORT, OpenAPI_amf_event_type_any_of_REACHABILITY_REPORT, OpenAPI_amf_event_type_any_of_COMMUNICATION_FAILURE_REPORT, OpenAPI_amf_event_type_any_of_UES_IN_AREA_REPORT, OpenAPI_amf_event_type_any_of_SUBSCRIPTION_ID_CHANGE, OpenAPI_amf_event_type_any_of_SUBSCRIPTION_ID_ADDITION, OpenAPI_amf_event_type_any_of_LOSS_OF_CONNECTIVITY, OpenAPI_amf_event_type_any_of__5GS_USER_STATE_REPORT, OpenAPI_amf_event_type_any_of_AVAILABILITY_AFTER_DDN_FAILURE, OpenAPI_amf_event_type_any_of_TYPE_ALLOCATION_CODE_REPORT, OpenAPI_amf_event_type_any_of_FREQUENT_MOBILITY_REGISTRATION_REPORT } OpenAPI_amf_event_type_any_of_e;

char* OpenAPI_amf_event_type_any_of_ToString(OpenAPI_amf_event_type_any_of_e amf_event_type_any_of);

OpenAPI_amf_event_type_any_of_e OpenAPI_amf_event_type_any_of_FromString(char* amf_event_type_any_of);

#ifdef __cplusplus
}
#endif

#endif /* _OpenAPI_amf_event_type_any_of_H_ */

