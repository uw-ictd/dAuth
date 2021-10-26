/*
 * Generated by asn1c-0.9.29 (http://lionet.info/asn1c)
 * From ASN.1 module "NR-InterNodeDefinitions"
 * 	found in "asn/nr-rrc-15.6.0.asn1"
 * 	`asn1c -fcompound-names -pdu=all -findirect-choice -fno-include-deps -gen-PER -no-gen-OER -no-gen-example -D rrc`
 */

#ifndef	_ASN_RRC_AS_Context_H_
#define	_ASN_RRC_AS_Context_H_


#include <asn_application.h>

/* Including external dependencies */
#include <constr_SEQUENCE.h>
#include <OCTET_STRING.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Forward declarations */
struct ASN_RRC_ReestablishmentInfo;
struct ASN_RRC_ConfigRestrictInfoSCG;
struct ASN_RRC_RAN_NotificationAreaInfo;
struct ASN_RRC_BandCombinationInfoSN;

/* ASN_RRC_AS-Context */
typedef struct ASN_RRC_AS_Context {
	struct ASN_RRC_ReestablishmentInfo	*reestablishmentInfo;	/* OPTIONAL */
	struct ASN_RRC_ConfigRestrictInfoSCG	*configRestrictInfo;	/* OPTIONAL */
	/*
	 * This type is extensible,
	 * possible extensions are below.
	 */
	struct ASN_RRC_AS_Context__ext1 {
		struct ASN_RRC_RAN_NotificationAreaInfo	*ran_NotificationAreaInfo;	/* OPTIONAL */
		
		/* Context for parsing across buffer boundaries */
		asn_struct_ctx_t _asn_ctx;
	} *ext1;
	struct ASN_RRC_AS_Context__ext2 {
		OCTET_STRING_t	*ueAssistanceInformation;	/* OPTIONAL */
		
		/* Context for parsing across buffer boundaries */
		asn_struct_ctx_t _asn_ctx;
	} *ext2;
	struct ASN_RRC_AS_Context__ext3 {
		struct ASN_RRC_BandCombinationInfoSN	*selectedBandCombinationSN;	/* OPTIONAL */
		
		/* Context for parsing across buffer boundaries */
		asn_struct_ctx_t _asn_ctx;
	} *ext3;
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} ASN_RRC_AS_Context_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_ASN_RRC_AS_Context;
extern asn_SEQUENCE_specifics_t asn_SPC_ASN_RRC_AS_Context_specs_1;
extern asn_TYPE_member_t asn_MBR_ASN_RRC_AS_Context_1[5];

#ifdef __cplusplus
}
#endif

#endif	/* _ASN_RRC_AS_Context_H_ */
#include <asn_internal.h>
