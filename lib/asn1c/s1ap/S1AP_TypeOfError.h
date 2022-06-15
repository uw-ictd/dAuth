/*
 * Generated by asn1c-0.9.29 (http://lionet.info/asn1c)
 * From ASN.1 module "S1AP-IEs"
 * 	found in "../support/s1ap-r16.7.0/36413-g70.asn"
 * 	`asn1c -pdu=all -fcompound-names -findirect-choice -fno-include-deps -no-gen-BER -no-gen-XER -no-gen-OER -no-gen-UPER`
 */

#ifndef	_S1AP_TypeOfError_H_
#define	_S1AP_TypeOfError_H_


#include <asn_application.h>

/* Including external dependencies */
#include <NativeEnumerated.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Dependencies */
typedef enum S1AP_TypeOfError {
	S1AP_TypeOfError_not_understood	= 0,
	S1AP_TypeOfError_missing	= 1
	/*
	 * Enumeration is extensible
	 */
} e_S1AP_TypeOfError;

/* S1AP_TypeOfError */
typedef long	 S1AP_TypeOfError_t;

/* Implementation */
extern asn_per_constraints_t asn_PER_type_S1AP_TypeOfError_constr_1;
extern asn_TYPE_descriptor_t asn_DEF_S1AP_TypeOfError;
extern const asn_INTEGER_specifics_t asn_SPC_TypeOfError_specs_1;
asn_struct_free_f TypeOfError_free;
asn_struct_print_f TypeOfError_print;
asn_constr_check_f TypeOfError_constraint;
per_type_decoder_f TypeOfError_decode_aper;
per_type_encoder_f TypeOfError_encode_aper;

#ifdef __cplusplus
}
#endif

#endif	/* _S1AP_TypeOfError_H_ */
#include <asn_internal.h>
