/*
 * Generated by asn1c-0.9.29 (http://lionet.info/asn1c)
 * From ASN.1 module "NGAP-IEs"
 * 	found in "../support/ngap-r16.7.0/38413-g70.asn"
 * 	`asn1c -pdu=all -fcompound-names -findirect-choice -fno-include-deps -no-gen-BER -no-gen-XER -no-gen-OER -no-gen-UPER`
 */

#ifndef	_NGAP_LoggedMDTTrigger_H_
#define	_NGAP_LoggedMDTTrigger_H_


#include <asn_application.h>

/* Including external dependencies */
#include <NULL.h>
#include <constr_CHOICE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Dependencies */
typedef enum NGAP_LoggedMDTTrigger_PR {
	NGAP_LoggedMDTTrigger_PR_NOTHING,	/* No components present */
	NGAP_LoggedMDTTrigger_PR_periodical,
	NGAP_LoggedMDTTrigger_PR_eventTrigger,
	NGAP_LoggedMDTTrigger_PR_choice_Extensions
} NGAP_LoggedMDTTrigger_PR;

/* Forward declarations */
struct NGAP_EventTrigger;
struct NGAP_ProtocolIE_SingleContainer;

/* NGAP_LoggedMDTTrigger */
typedef struct NGAP_LoggedMDTTrigger {
	NGAP_LoggedMDTTrigger_PR present;
	union NGAP_LoggedMDTTrigger_u {
		NULL_t	 periodical;
		struct NGAP_EventTrigger	*eventTrigger;
		struct NGAP_ProtocolIE_SingleContainer	*choice_Extensions;
	} choice;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} NGAP_LoggedMDTTrigger_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_NGAP_LoggedMDTTrigger;
extern asn_CHOICE_specifics_t asn_SPC_NGAP_LoggedMDTTrigger_specs_1;
extern asn_TYPE_member_t asn_MBR_NGAP_LoggedMDTTrigger_1[3];
extern asn_per_constraints_t asn_PER_type_NGAP_LoggedMDTTrigger_constr_1;

#ifdef __cplusplus
}
#endif

#endif	/* _NGAP_LoggedMDTTrigger_H_ */
#include <asn_internal.h>
