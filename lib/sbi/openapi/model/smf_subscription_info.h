/*
 * smf_subscription_info.h
 *
 * Information related to active subscriptions at the SMF(s)
 */

#ifndef _OpenAPI_smf_subscription_info_H_
#define _OpenAPI_smf_subscription_info_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "smf_subscription_item.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OpenAPI_smf_subscription_info_s OpenAPI_smf_subscription_info_t;
typedef struct OpenAPI_smf_subscription_info_s {
    OpenAPI_list_t *smf_subscription_list;
} OpenAPI_smf_subscription_info_t;

OpenAPI_smf_subscription_info_t *OpenAPI_smf_subscription_info_create(
    OpenAPI_list_t *smf_subscription_list
);
void OpenAPI_smf_subscription_info_free(OpenAPI_smf_subscription_info_t *smf_subscription_info);
OpenAPI_smf_subscription_info_t *OpenAPI_smf_subscription_info_parseFromJSON(cJSON *smf_subscription_infoJSON);
cJSON *OpenAPI_smf_subscription_info_convertToJSON(OpenAPI_smf_subscription_info_t *smf_subscription_info);
OpenAPI_smf_subscription_info_t *OpenAPI_smf_subscription_info_copy(OpenAPI_smf_subscription_info_t *dst, OpenAPI_smf_subscription_info_t *src);

#ifdef __cplusplus
}
#endif

#endif /* _OpenAPI_smf_subscription_info_H_ */

