/*
 * pcc_rule.h
 *
 * 
 */

#ifndef _OpenAPI_pcc_rule_H_
#define _OpenAPI_pcc_rule_H_

#include <string.h>
#include "../external/cJSON.h"
#include "../include/list.h"
#include "../include/keyValuePair.h"
#include "../include/binary.h"
#include "af_sig_protocol.h"
#include "downlink_data_notification_control.h"
#include "downlink_data_notification_control_rm.h"
#include "flow_information.h"
#include "tscai_input_container.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OpenAPI_pcc_rule_s OpenAPI_pcc_rule_t;
typedef struct OpenAPI_pcc_rule_s {
    OpenAPI_list_t *flow_infos;
    char *app_id;
    char app_descriptor;
    bool is_cont_ver;
    int cont_ver;
    char *pcc_rule_id;
    bool is_precedence;
    int precedence;
    OpenAPI_af_sig_protocol_e af_sig_protocol;
    bool is_app_reloc;
    int app_reloc;
    OpenAPI_list_t *ref_qos_data;
    OpenAPI_list_t *ref_alt_qos_params;
    OpenAPI_list_t *ref_tc_data;
    OpenAPI_list_t *ref_chg_data;
    OpenAPI_list_t *ref_chg_n3g_data;
    OpenAPI_list_t *ref_um_data;
    OpenAPI_list_t *ref_um_n3g_data;
    char *ref_cond_data;
    OpenAPI_list_t *ref_qos_mon;
    bool is_addr_preser_ind;
    int addr_preser_ind;
    struct OpenAPI_tscai_input_container_s *tscai_input_dl;
    struct OpenAPI_tscai_input_container_s *tscai_input_ul;
    struct OpenAPI_downlink_data_notification_control_s *dd_notif_ctrl;
    struct OpenAPI_downlink_data_notification_control_rm_s *dd_notif_ctrl2;
    bool is_dis_ue_notif;
    int dis_ue_notif;
} OpenAPI_pcc_rule_t;

OpenAPI_pcc_rule_t *OpenAPI_pcc_rule_create(
    OpenAPI_list_t *flow_infos,
    char *app_id,
    char app_descriptor,
    bool is_cont_ver,
    int cont_ver,
    char *pcc_rule_id,
    bool is_precedence,
    int precedence,
    OpenAPI_af_sig_protocol_e af_sig_protocol,
    bool is_app_reloc,
    int app_reloc,
    OpenAPI_list_t *ref_qos_data,
    OpenAPI_list_t *ref_alt_qos_params,
    OpenAPI_list_t *ref_tc_data,
    OpenAPI_list_t *ref_chg_data,
    OpenAPI_list_t *ref_chg_n3g_data,
    OpenAPI_list_t *ref_um_data,
    OpenAPI_list_t *ref_um_n3g_data,
    char *ref_cond_data,
    OpenAPI_list_t *ref_qos_mon,
    bool is_addr_preser_ind,
    int addr_preser_ind,
    OpenAPI_tscai_input_container_t *tscai_input_dl,
    OpenAPI_tscai_input_container_t *tscai_input_ul,
    OpenAPI_downlink_data_notification_control_t *dd_notif_ctrl,
    OpenAPI_downlink_data_notification_control_rm_t *dd_notif_ctrl2,
    bool is_dis_ue_notif,
    int dis_ue_notif
);
void OpenAPI_pcc_rule_free(OpenAPI_pcc_rule_t *pcc_rule);
OpenAPI_pcc_rule_t *OpenAPI_pcc_rule_parseFromJSON(cJSON *pcc_ruleJSON);
cJSON *OpenAPI_pcc_rule_convertToJSON(OpenAPI_pcc_rule_t *pcc_rule);
OpenAPI_pcc_rule_t *OpenAPI_pcc_rule_copy(OpenAPI_pcc_rule_t *dst, OpenAPI_pcc_rule_t *src);

#ifdef __cplusplus
}
#endif

#endif /* _OpenAPI_pcc_rule_H_ */

