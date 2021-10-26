/*
 * Generated by asn1c-0.9.29 (http://lionet.info/asn1c)
 * From ASN.1 module "NGAP-PDU-Contents"
 * 	found in "asn/ngap-15.8.0.asn1"
 * 	`asn1c -fcompound-names -pdu=all -findirect-choice -fno-include-deps -gen-PER -no-gen-OER -no-gen-example -D ngap`
 */

#ifndef	_ASN_NGAP_AMFConfigurationUpdate_H_
#define	_ASN_NGAP_AMFConfigurationUpdate_H_


#include <asn_application.h>

/* Including external dependencies */
#include "ASN_NGAP_ProtocolIE-Container.h"
#include <constr_SEQUENCE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ASN_NGAP_AMFConfigurationUpdate */
typedef struct ASN_NGAP_AMFConfigurationUpdate {
	ASN_NGAP_ProtocolIE_Container_6810P45_t	 protocolIEs;
	/*
	 * This type is extensible,
	 * possible extensions are below.
	 */
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} ASN_NGAP_AMFConfigurationUpdate_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_ASN_NGAP_AMFConfigurationUpdate;
extern asn_SEQUENCE_specifics_t asn_SPC_ASN_NGAP_AMFConfigurationUpdate_specs_1;
extern asn_TYPE_member_t asn_MBR_ASN_NGAP_AMFConfigurationUpdate_1[1];

#ifdef __cplusplus
}
#endif

#endif	/* _ASN_NGAP_AMFConfigurationUpdate_H_ */
#include <asn_internal.h>
