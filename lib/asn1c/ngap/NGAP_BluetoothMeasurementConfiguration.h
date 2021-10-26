/*
 * Generated by asn1c-0.9.29 (http://lionet.info/asn1c)
 * From ASN.1 module "NGAP-IEs"
 * 	found in "../support/ngap-r16.4.0/38413-g40.asn"
 * 	`asn1c -pdu=all -fcompound-names -findirect-choice -fno-include-deps -no-gen-BER -no-gen-XER -no-gen-OER -no-gen-UPER`
 */

#ifndef	_NGAP_BluetoothMeasurementConfiguration_H_
#define	_NGAP_BluetoothMeasurementConfiguration_H_


#include <asn_application.h>

/* Including external dependencies */
#include "NGAP_BluetoothMeasConfig.h"
#include <NativeEnumerated.h>
#include <constr_SEQUENCE.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Dependencies */
typedef enum NGAP_BluetoothMeasurementConfiguration__bt_rssi {
	NGAP_BluetoothMeasurementConfiguration__bt_rssi_true	= 0
	/*
	 * Enumeration is extensible
	 */
} e_NGAP_BluetoothMeasurementConfiguration__bt_rssi;

/* Forward declarations */
struct NGAP_BluetoothMeasConfigNameList;
struct NGAP_ProtocolExtensionContainer;

/* NGAP_BluetoothMeasurementConfiguration */
typedef struct NGAP_BluetoothMeasurementConfiguration {
	NGAP_BluetoothMeasConfig_t	 bluetoothMeasConfig;
	struct NGAP_BluetoothMeasConfigNameList	*bluetoothMeasConfigNameList;	/* OPTIONAL */
	long	*bt_rssi;	/* OPTIONAL */
	struct NGAP_ProtocolExtensionContainer	*iE_Extensions;	/* OPTIONAL */
	/*
	 * This type is extensible,
	 * possible extensions are below.
	 */
	
	/* Context for parsing across buffer boundaries */
	asn_struct_ctx_t _asn_ctx;
} NGAP_BluetoothMeasurementConfiguration_t;

/* Implementation */
/* extern asn_TYPE_descriptor_t asn_DEF_NGAP_bt_rssi_4;	// (Use -fall-defs-global to expose) */
extern asn_TYPE_descriptor_t asn_DEF_NGAP_BluetoothMeasurementConfiguration;
extern asn_SEQUENCE_specifics_t asn_SPC_NGAP_BluetoothMeasurementConfiguration_specs_1;
extern asn_TYPE_member_t asn_MBR_NGAP_BluetoothMeasurementConfiguration_1[4];

#ifdef __cplusplus
}
#endif

#endif	/* _NGAP_BluetoothMeasurementConfiguration_H_ */
#include <asn_internal.h>
