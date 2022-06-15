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

#if !defined(OGS_NAS_INSIDE) && !defined(OGS_NAS_COMPILATION)
#error "This header cannot be included directly."
#endif

#ifndef OGS_NAS_SECURITY_H
#define OGS_NAS_SECURITY_H

#ifdef __cplusplus
extern "C" {
#endif

#define OGS_NAS_SECURITY_DOWNLINK_DIRECTION 1
#define OGS_NAS_SECURITY_UPLINK_DIRECTION 0

void ogs_nas_mac_calculate(uint8_t algorithm_identity,
    uint8_t *knas_int, uint32_t count, uint8_t bearer, 
    uint8_t direction, ogs_pkbuf_t *pkbuf, uint8_t *mac);

void ogs_nas_encrypt(uint8_t algorithm_identity,
    uint8_t *knas_enc, uint32_t count, uint8_t bearer, 
    uint8_t direction, ogs_pkbuf_t *pkbuf);

#ifdef __cplusplus
}
#endif

#endif /* OGS_NAS_SECURITY_H */
