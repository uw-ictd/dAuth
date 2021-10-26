/*
 * Generated by asn1c-0.9.29 (http://lionet.info/asn1c)
 * From ASN.1 module "NGAP-IEs"
 * 	found in "asn/ngap-15.8.0.asn1"
 * 	`asn1c -fcompound-names -pdu=all -findirect-choice -fno-include-deps -gen-PER -no-gen-OER -no-gen-example -D ngap`
 */

#ifndef	_ASN_NGAP_CauseRadioNetwork_H_
#define	_ASN_NGAP_CauseRadioNetwork_H_


#include <asn_application.h>

/* Including external dependencies */
#include <NativeEnumerated.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Dependencies */
typedef enum ASN_NGAP_CauseRadioNetwork {
	ASN_NGAP_CauseRadioNetwork_unspecified	= 0,
	ASN_NGAP_CauseRadioNetwork_txnrelocoverall_expiry	= 1,
	ASN_NGAP_CauseRadioNetwork_successful_handover	= 2,
	ASN_NGAP_CauseRadioNetwork_release_due_to_ngran_generated_reason	= 3,
	ASN_NGAP_CauseRadioNetwork_release_due_to_5gc_generated_reason	= 4,
	ASN_NGAP_CauseRadioNetwork_handover_cancelled	= 5,
	ASN_NGAP_CauseRadioNetwork_partial_handover	= 6,
	ASN_NGAP_CauseRadioNetwork_ho_failure_in_target_5GC_ngran_node_or_target_system	= 7,
	ASN_NGAP_CauseRadioNetwork_ho_target_not_allowed	= 8,
	ASN_NGAP_CauseRadioNetwork_tngrelocoverall_expiry	= 9,
	ASN_NGAP_CauseRadioNetwork_tngrelocprep_expiry	= 10,
	ASN_NGAP_CauseRadioNetwork_cell_not_available	= 11,
	ASN_NGAP_CauseRadioNetwork_unknown_targetID	= 12,
	ASN_NGAP_CauseRadioNetwork_no_radio_resources_available_in_target_cell	= 13,
	ASN_NGAP_CauseRadioNetwork_unknown_local_UE_NGAP_ID	= 14,
	ASN_NGAP_CauseRadioNetwork_inconsistent_remote_UE_NGAP_ID	= 15,
	ASN_NGAP_CauseRadioNetwork_handover_desirable_for_radio_reason	= 16,
	ASN_NGAP_CauseRadioNetwork_time_critical_handover	= 17,
	ASN_NGAP_CauseRadioNetwork_resource_optimisation_handover	= 18,
	ASN_NGAP_CauseRadioNetwork_reduce_load_in_serving_cell	= 19,
	ASN_NGAP_CauseRadioNetwork_user_inactivity	= 20,
	ASN_NGAP_CauseRadioNetwork_radio_connection_with_ue_lost	= 21,
	ASN_NGAP_CauseRadioNetwork_radio_resources_not_available	= 22,
	ASN_NGAP_CauseRadioNetwork_invalid_qos_combination	= 23,
	ASN_NGAP_CauseRadioNetwork_failure_in_radio_interface_procedure	= 24,
	ASN_NGAP_CauseRadioNetwork_interaction_with_other_procedure	= 25,
	ASN_NGAP_CauseRadioNetwork_unknown_PDU_session_ID	= 26,
	ASN_NGAP_CauseRadioNetwork_unkown_qos_flow_ID	= 27,
	ASN_NGAP_CauseRadioNetwork_multiple_PDU_session_ID_instances	= 28,
	ASN_NGAP_CauseRadioNetwork_multiple_qos_flow_ID_instances	= 29,
	ASN_NGAP_CauseRadioNetwork_encryption_and_or_integrity_protection_algorithms_not_supported	= 30,
	ASN_NGAP_CauseRadioNetwork_ng_intra_system_handover_triggered	= 31,
	ASN_NGAP_CauseRadioNetwork_ng_inter_system_handover_triggered	= 32,
	ASN_NGAP_CauseRadioNetwork_xn_handover_triggered	= 33,
	ASN_NGAP_CauseRadioNetwork_not_supported_5QI_value	= 34,
	ASN_NGAP_CauseRadioNetwork_ue_context_transfer	= 35,
	ASN_NGAP_CauseRadioNetwork_ims_voice_eps_fallback_or_rat_fallback_triggered	= 36,
	ASN_NGAP_CauseRadioNetwork_up_integrity_protection_not_possible	= 37,
	ASN_NGAP_CauseRadioNetwork_up_confidentiality_protection_not_possible	= 38,
	ASN_NGAP_CauseRadioNetwork_slice_not_supported	= 39,
	ASN_NGAP_CauseRadioNetwork_ue_in_rrc_inactive_state_not_reachable	= 40,
	ASN_NGAP_CauseRadioNetwork_redirection	= 41,
	ASN_NGAP_CauseRadioNetwork_resources_not_available_for_the_slice	= 42,
	ASN_NGAP_CauseRadioNetwork_ue_max_integrity_protected_data_rate_reason	= 43,
	ASN_NGAP_CauseRadioNetwork_release_due_to_cn_detected_mobility	= 44,
	/*
	 * Enumeration is extensible
	 */
	ASN_NGAP_CauseRadioNetwork_n26_interface_not_available	= 45,
	ASN_NGAP_CauseRadioNetwork_release_due_to_pre_emption	= 46,
	ASN_NGAP_CauseRadioNetwork_multiple_location_reporting_reference_ID_instances	= 47
} e_ASN_NGAP_CauseRadioNetwork;

/* ASN_NGAP_CauseRadioNetwork */
typedef long	 ASN_NGAP_CauseRadioNetwork_t;

/* Implementation */
extern asn_per_constraints_t asn_PER_type_ASN_NGAP_CauseRadioNetwork_constr_1;
extern asn_TYPE_descriptor_t asn_DEF_ASN_NGAP_CauseRadioNetwork;
extern const asn_INTEGER_specifics_t asn_SPC_ASN_NGAP_CauseRadioNetwork_specs_1;
asn_struct_free_f ASN_NGAP_CauseRadioNetwork_free;
asn_struct_print_f ASN_NGAP_CauseRadioNetwork_print;
asn_constr_check_f ASN_NGAP_CauseRadioNetwork_constraint;
ber_type_decoder_f ASN_NGAP_CauseRadioNetwork_decode_ber;
der_type_encoder_f ASN_NGAP_CauseRadioNetwork_encode_der;
xer_type_decoder_f ASN_NGAP_CauseRadioNetwork_decode_xer;
xer_type_encoder_f ASN_NGAP_CauseRadioNetwork_encode_xer;
per_type_decoder_f ASN_NGAP_CauseRadioNetwork_decode_uper;
per_type_encoder_f ASN_NGAP_CauseRadioNetwork_encode_uper;
per_type_decoder_f ASN_NGAP_CauseRadioNetwork_decode_aper;
per_type_encoder_f ASN_NGAP_CauseRadioNetwork_encode_aper;

#ifdef __cplusplus
}
#endif

#endif	/* _ASN_NGAP_CauseRadioNetwork_H_ */
#include <asn_internal.h>
