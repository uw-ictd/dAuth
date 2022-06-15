/*
 * Generated by asn1c-0.9.29 (http://lionet.info/asn1c)
 * From ASN.1 module "S1AP-IEs"
 * 	found in "../support/s1ap-r16.7.0/36413-g70.asn"
 * 	`asn1c -pdu=all -fcompound-names -findirect-choice -fno-include-deps -no-gen-BER -no-gen-XER -no-gen-OER -no-gen-UPER`
 */

#include "S1AP_NB-IoT-Paging-eDRX-Cycle.h"

/*
 * This type is implemented using NativeEnumerated,
 * so here we adjust the DEF accordingly.
 */
#if !defined(ASN_DISABLE_UPER_SUPPORT) || !defined(ASN_DISABLE_APER_SUPPORT)
asn_per_constraints_t asn_PER_type_S1AP_NB_IoT_Paging_eDRX_Cycle_constr_1 CC_NOTUSED = {
	{ APC_CONSTRAINED | APC_EXTENSIBLE,  4,  4,  0,  13 }	/* (0..13,...) */,
	{ APC_UNCONSTRAINED,	-1, -1,  0,  0 },
	0, 0	/* No PER value map */
};
#endif  /* !defined(ASN_DISABLE_UPER_SUPPORT) || !defined(ASN_DISABLE_APER_SUPPORT) */
static const asn_INTEGER_enum_map_t asn_MAP_S1AP_NB_IoT_Paging_eDRX_Cycle_value2enum_1[] = {
	{ 0,	3,	"hf2" },
	{ 1,	3,	"hf4" },
	{ 2,	3,	"hf6" },
	{ 3,	3,	"hf8" },
	{ 4,	4,	"hf10" },
	{ 5,	4,	"hf12" },
	{ 6,	4,	"hf14" },
	{ 7,	4,	"hf16" },
	{ 8,	4,	"hf32" },
	{ 9,	4,	"hf64" },
	{ 10,	5,	"hf128" },
	{ 11,	5,	"hf256" },
	{ 12,	5,	"hf512" },
	{ 13,	6,	"hf1024" }
	/* This list is extensible */
};
static const unsigned int asn_MAP_S1AP_NB_IoT_Paging_eDRX_Cycle_enum2value_1[] = {
	4,	/* hf10(4) */
	13,	/* hf1024(13) */
	5,	/* hf12(5) */
	10,	/* hf128(10) */
	6,	/* hf14(6) */
	7,	/* hf16(7) */
	0,	/* hf2(0) */
	11,	/* hf256(11) */
	8,	/* hf32(8) */
	1,	/* hf4(1) */
	12,	/* hf512(12) */
	2,	/* hf6(2) */
	9,	/* hf64(9) */
	3	/* hf8(3) */
	/* This list is extensible */
};
const asn_INTEGER_specifics_t asn_SPC_S1AP_NB_IoT_Paging_eDRX_Cycle_specs_1 = {
	asn_MAP_S1AP_NB_IoT_Paging_eDRX_Cycle_value2enum_1,	/* "tag" => N; sorted by tag */
	asn_MAP_S1AP_NB_IoT_Paging_eDRX_Cycle_enum2value_1,	/* N => "tag"; sorted by N */
	14,	/* Number of elements in the maps */
	15,	/* Extensions before this member */
	1,	/* Strict enumeration */
	0,	/* Native long size */
	0
};
static const ber_tlv_tag_t asn_DEF_S1AP_NB_IoT_Paging_eDRX_Cycle_tags_1[] = {
	(ASN_TAG_CLASS_UNIVERSAL | (10 << 2))
};
asn_TYPE_descriptor_t asn_DEF_S1AP_NB_IoT_Paging_eDRX_Cycle = {
	"NB-IoT-Paging-eDRX-Cycle",
	"NB-IoT-Paging-eDRX-Cycle",
	&asn_OP_NativeEnumerated,
	asn_DEF_S1AP_NB_IoT_Paging_eDRX_Cycle_tags_1,
	sizeof(asn_DEF_S1AP_NB_IoT_Paging_eDRX_Cycle_tags_1)
		/sizeof(asn_DEF_S1AP_NB_IoT_Paging_eDRX_Cycle_tags_1[0]), /* 1 */
	asn_DEF_S1AP_NB_IoT_Paging_eDRX_Cycle_tags_1,	/* Same as above */
	sizeof(asn_DEF_S1AP_NB_IoT_Paging_eDRX_Cycle_tags_1)
		/sizeof(asn_DEF_S1AP_NB_IoT_Paging_eDRX_Cycle_tags_1[0]), /* 1 */
	{
#if !defined(ASN_DISABLE_OER_SUPPORT)
		0,
#endif  /* !defined(ASN_DISABLE_OER_SUPPORT) */
#if !defined(ASN_DISABLE_UPER_SUPPORT) || !defined(ASN_DISABLE_APER_SUPPORT)
		&asn_PER_type_S1AP_NB_IoT_Paging_eDRX_Cycle_constr_1,
#endif  /* !defined(ASN_DISABLE_UPER_SUPPORT) || !defined(ASN_DISABLE_APER_SUPPORT) */
		NativeEnumerated_constraint
	},
	0, 0,	/* Defined elsewhere */
	&asn_SPC_S1AP_NB_IoT_Paging_eDRX_Cycle_specs_1	/* Additional specs */
};

