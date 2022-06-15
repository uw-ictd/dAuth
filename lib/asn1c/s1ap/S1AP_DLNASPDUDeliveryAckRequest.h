/*
 * Generated by asn1c-0.9.29 (http://lionet.info/asn1c)
 * From ASN.1 module "S1AP-IEs"
 * 	found in "../support/s1ap-r16.7.0/36413-g70.asn"
 * 	`asn1c -pdu=all -fcompound-names -findirect-choice -fno-include-deps -no-gen-BER -no-gen-XER -no-gen-OER -no-gen-UPER`
 */

#ifndef	_S1AP_DLNASPDUDeliveryAckRequest_H_
#define	_S1AP_DLNASPDUDeliveryAckRequest_H_


#include <asn_application.h>

/* Including external dependencies */
#include <NativeEnumerated.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Dependencies */
typedef enum S1AP_DLNASPDUDeliveryAckRequest {
	S1AP_DLNASPDUDeliveryAckRequest_requested	= 0
	/*
	 * Enumeration is extensible
	 */
} e_S1AP_DLNASPDUDeliveryAckRequest;

/* S1AP_DLNASPDUDeliveryAckRequest */
typedef long	 S1AP_DLNASPDUDeliveryAckRequest_t;

/* Implementation */
extern asn_TYPE_descriptor_t asn_DEF_S1AP_DLNASPDUDeliveryAckRequest;
asn_struct_free_f S1AP_DLNASPDUDeliveryAckRequest_free;
asn_struct_print_f S1AP_DLNASPDUDeliveryAckRequest_print;
asn_constr_check_f S1AP_DLNASPDUDeliveryAckRequest_constraint;
per_type_decoder_f S1AP_DLNASPDUDeliveryAckRequest_decode_aper;
per_type_encoder_f S1AP_DLNASPDUDeliveryAckRequest_encode_aper;

#ifdef __cplusplus
}
#endif

#endif	/* _S1AP_DLNASPDUDeliveryAckRequest_H_ */
#include <asn_internal.h>
