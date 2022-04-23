/*
 * Copyright (C) 2021 Matt Johnson <matt9j@cs.washington.edu>
 *
 * This file is part of dAuth, and extends open5gs
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

#ifndef __AUSF_DAUTH_C_BINDING_H__
#define __AUSF_DAUTH_C_BINDING_H__

#include "ogs-app.h"
#include "ogs-crypt.h"
#include "ogs-sbi.h"

#include "event.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct dauth_context_wrapper {
    void* server_context;
} dauth_context_t;

typedef struct dauth_ue_context {
    void* local_auth_client;
} dauth_ue_context_t;

bool
dauth_context_init(dauth_context_t * const context);

bool
dauth_context_final(dauth_context_t * const context);

bool
wait_for_next_rpc_event(void** tag);

bool
handle_rpc_completion(void* tag);

void
grpc_client_shutdown(void);

bool
ausf_dauth_shim_request_auth_vector(
    ausf_ue_t * const ausf_ue,
    const OpenAPI_authentication_info_t * const authentication_info
    );

bool
ausf_dauth_shim_forward_received_auth_vector(
    ausf_ue_t * const ausf_ue,
    ogs_sbi_stream_t *stream,
    const OpenAPI_authentication_info_t * const authentication_info
    );

bool
ausf_dauth_shim_request_confirm_auth(
    ausf_ue_t * const ausf_ue,
    const uint8_t * const res_star);

bool
ausf_dauth_shim_forward_confirmed_key(
    ausf_ue_t * const ausf_ue,
    ogs_sbi_stream_t *stream);

#ifdef __cplusplus
}
#endif

#endif /* __AUSF_DAUTH_C_BINDING_H__ */
