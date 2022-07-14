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

#ifndef __MME_DAUTH_MME_C_BINDING_H__
#define __MME_DAUTH_MME_C_BINDING_H__

#include "ogs-app.h"
#include "ogs-crypt.h"
#include "ogs-nas-eps.h"

#include "mme-event.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct dauth_mme_context_wrapper {
    void* server_context;
} dauth_mme_context_t;

typedef struct dauth_mme_ue_context {
    void* local_auth_client;
} dauth_mme_ue_context_t;

bool
dauth_context_init(dauth_mme_context_t * const context);

bool
dauth_context_final(dauth_mme_context_t * const context);

bool
wait_for_next_rpc_event(void** tag);

bool
handle_rpc_completion(void* tag);

void
grpc_client_shutdown(void);

bool
mme_dauth_shim_request_auth_vector(
    mme_ue_t * const mme_ue
    );

bool
mme_dauth_shim_request_auth_vector_resync(
    mme_ue_t * const mme_ue,
    const ogs_nas_authentication_failure_parameter_t * const resync_info
    );

bool
mme_dauth_shim_request_confirm_auth(
    mme_ue_t * const mme_ue,
    const uint8_t * const res_star
    );

void mme_dauth_shim_compute_res_hash(uint8_t *rand, uint8_t *res, uint8_t res_len, uint8_t *res_hash);

#ifdef __cplusplus
}
#endif

#endif /* __MME_DAUTH_MME_C_BINDING_H__ */
