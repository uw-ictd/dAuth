/*
 * Generated by asn1c-0.9.29 (http://lionet.info/asn1c)
 * From ASN.1 module "NR-RRC-Definitions"
 * 	found in "asn/nr-rrc-15.6.0.asn1"
 * 	`asn1c -fcompound-names -pdu=all -findirect-choice -fno-include-deps -gen-PER -no-gen-OER -no-gen-example -D rrc`
 */

#ifndef	_ASN_RRC_PagingRecord_H_
#define	_ASN_RRC_PagingRecord_H_


#include <asn_application.h>

/* Including external dependencies */
#include "ASN_RRC_PagingUE-Identity.h"
#include <NativeEnumerated.h>
#include <constr_SEQUENCE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Dependencies */
typedef enum ASN_RRC_PagingRecord__accessType {
	ASN_RRC_PagingRecord__accessType_non3GPP	= 0
} e_ASN_RRC_PagingRecord__accessType;

/* ASN_RRC_PagingRecord */
typedef struct ASN_RRC_PagingRecord {
	ASN_RRC_PagingUE_Identity_t	 ue_Identity;
	long	*accessType;	/* OPTIONAL */
	/*
	 * This type is extensible,
	 * possible extensions are below.
	 */
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} ASN_RRC_PagingRecord_t;

/* Implementation */
/* extern asn_TYPE_descriptor_t asn_DEF_ASN_RRC_accessType_3;	// (Use -fall-defs-global to expose) */
extern asn_TYPE_descriptor_t asn_DEF_ASN_RRC_PagingRecord;
extern asn_SEQUENCE_specifics_t asn_SPC_ASN_RRC_PagingRecord_specs_1;
extern asn_TYPE_member_t asn_MBR_ASN_RRC_PagingRecord_1[2];

#ifdef __cplusplus
}
#endif

#endif	/* _ASN_RRC_PagingRecord_H_ */
#include <asn_internal.h>
