/*
 * Copyright (C) 2019,2020 by Sukchan Lee <acetcom@gmail.com>
 *
 * This file is part of Open5GS.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#include "test-common.h"

ogs_pkbuf_t *testgsm_build_pdu_session_establishment_request(
        test_sess_t *test_sess)
{
    ogs_nas_5gs_message_t message;
    ogs_nas_5gs_pdu_session_establishment_request_t
        *pdu_session_establishment_request =
            &message.gsm.pdu_session_establishment_request;
    ogs_nas_integrity_protection_maximum_data_rate_t
        *integrity_protection_maximum_data_rate = NULL;
    ogs_nas_pdu_session_type_t *pdu_session_type = NULL;
    ogs_nas_ssc_mode_t *ssc_mode = NULL;
    ogs_nas_extended_protocol_configuration_options_t
        *extended_protocol_configuration_options = NULL;
    uint8_t ue_pco[35] =
        "\x80\x80\x21\x10\x01\x00\x00\x10\x81\x06\x00\x00\x00\x00"
        "\x83\x06\x00\x00\x00\x00\x00\x0c\x00\x00\x0d\x00\x00\x02\x00\x00"
        "\x0a\x00\x00\x10\x00";

    test_ue_t *test_ue = NULL;
    ogs_pkbuf_t *pkbuf = NULL;

    ogs_assert(test_sess);
    test_ue = test_sess;
    ogs_assert(test_ue);

    integrity_protection_maximum_data_rate =
        &pdu_session_establishment_request->
            integrity_protection_maximum_data_rate;
    pdu_session_type = &pdu_session_establishment_request->pdu_session_type;
    ssc_mode = &pdu_session_establishment_request->ssc_mode;
    extended_protocol_configuration_options =
        &pdu_session_establishment_request->
            extended_protocol_configuration_options;

    memset(&message, 0, sizeof(message));

    message.gsm.h.extended_protocol_discriminator =
        OGS_NAS_EXTENDED_PROTOCOL_DISCRIMINATOR_5GSM;
    message.gsm.h.pdu_session_identity = test_sess->psi;
    message.gsm.h.procedure_transaction_identity = test_sess->pti;
    message.gsm.h.message_type = OGS_NAS_5GS_PDU_SESSION_ESTABLISHMENT_REQUEST;

    integrity_protection_maximum_data_rate->ul = 0xff;
    integrity_protection_maximum_data_rate->dl = 0xff;

    pdu_session_establishment_request->presencemask |=
        OGS_NAS_5GS_PDU_SESSION_ESTABLISHMENT_REQUEST_PDU_SESSION_TYPE_PRESENT;
    pdu_session_type->value = test_sess->pdu_session_type;

    if (test_sess->pdu_session_establishment_param.ssc_mode) {
        pdu_session_establishment_request->presencemask |=
            OGS_NAS_5GS_PDU_SESSION_ESTABLISHMENT_REQUEST_SSC_MODE_PRESENT;
        ssc_mode->value = OGS_NAS_SSC_MODE_1;
    }

    if (test_sess->pdu_session_establishment_param.epco) {
        pdu_session_establishment_request->presencemask |=
            OGS_NAS_5GS_PDU_SESSION_ESTABLISHMENT_REQUEST_EXTENDED_PROTOCOL_CONFIGURATION_OPTIONS_PRESENT;
        extended_protocol_configuration_options->length = sizeof(ue_pco);
        extended_protocol_configuration_options->buffer = ue_pco;
    }

    return ogs_nas_5gs_plain_encode(&message);
}

ogs_pkbuf_t *testgsm_build_pdu_session_modification_complete(
        test_sess_t *test_sess)
{
    ogs_nas_5gs_message_t message;
    ogs_nas_5gs_pdu_session_modification_complete_t
        *pdu_session_modification_complete =
            &message.gsm.pdu_session_modification_complete;

    test_ue_t *test_ue = NULL;
    ogs_pkbuf_t *pkbuf = NULL;

    ogs_assert(test_sess);
    test_ue = test_sess;
    ogs_assert(test_ue);

    memset(&message, 0, sizeof(message));

    message.gsm.h.extended_protocol_discriminator =
        OGS_NAS_EXTENDED_PROTOCOL_DISCRIMINATOR_5GSM;
    message.gsm.h.pdu_session_identity = test_sess->psi;
    message.gsm.h.procedure_transaction_identity = test_sess->pti;
    message.gsm.h.message_type = OGS_NAS_5GS_PDU_SESSION_MODIFICATION_COMPLETE;

    return ogs_nas_5gs_plain_encode(&message);
}

ogs_pkbuf_t *testgsm_build_pdu_session_release_request(test_sess_t *test_sess)
{
    ogs_nas_5gs_message_t message;
    ogs_nas_5gs_pdu_session_release_request_t *pdu_session_release_request =
            &message.gsm.pdu_session_release_request;

    test_ue_t *test_ue = NULL;
    ogs_pkbuf_t *pkbuf = NULL;

    ogs_assert(test_sess);
    test_ue = test_sess;
    ogs_assert(test_ue);

    memset(&message, 0, sizeof(message));

    message.gsm.h.extended_protocol_discriminator =
        OGS_NAS_EXTENDED_PROTOCOL_DISCRIMINATOR_5GSM;
    message.gsm.h.pdu_session_identity = test_sess->psi;
    message.gsm.h.procedure_transaction_identity = test_sess->pti;
    message.gsm.h.message_type = OGS_NAS_5GS_PDU_SESSION_RELEASE_REQUEST;

    return ogs_nas_5gs_plain_encode(&message);
}

ogs_pkbuf_t *testgsm_build_pdu_session_release_complete(test_sess_t *test_sess)
{
    ogs_nas_5gs_message_t message;
    ogs_nas_5gs_pdu_session_release_complete_t *pdu_session_release_complete =
            &message.gsm.pdu_session_release_complete;

    test_ue_t *test_ue = NULL;
    ogs_pkbuf_t *pkbuf = NULL;

    ogs_assert(test_sess);
    test_ue = test_sess;
    ogs_assert(test_ue);

    memset(&message, 0, sizeof(message));

    message.gsm.h.extended_protocol_discriminator =
        OGS_NAS_EXTENDED_PROTOCOL_DISCRIMINATOR_5GSM;
    message.gsm.h.pdu_session_identity = test_sess->psi;
    message.gsm.h.procedure_transaction_identity = test_sess->pti;
    message.gsm.h.message_type = OGS_NAS_5GS_PDU_SESSION_RELEASE_COMPLETE;

    return ogs_nas_5gs_plain_encode(&message);
}
