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

#ifndef TEST_GSM_BUILD_H
#define TEST_GSM_BUILD_H

#ifdef __cplusplus
extern "C" {
#endif

ogs_pkbuf_t *testgsm_build_pdu_session_establishment_request(
        test_sess_t *test_sess);
ogs_pkbuf_t *testgsm_build_pdu_session_modification_request(
        test_bearer_t *bearer, uint8_t gsm_cause,
        uint8_t qos_rule_code, uint8_t qos_flow_description_code);
ogs_pkbuf_t *testgsm_build_pdu_session_modification_complete(
        test_sess_t *test_sess);
ogs_pkbuf_t *testgsm_build_pdu_session_release_request(test_sess_t *test_sess);
ogs_pkbuf_t *testgsm_build_pdu_session_release_complete(test_sess_t *test_sess);

#ifdef __cplusplus
}
#endif

#endif /* TEST_GSM_BUILD_H */
