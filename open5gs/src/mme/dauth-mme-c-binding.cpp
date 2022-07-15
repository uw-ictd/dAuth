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

#include <grpcpp/grpcpp.h>
#include <memory>
#include <string.h>

#include "authentication_data.pb.h"
#include "local_authentication.grpc.pb.h"
#include "local_authentication.pb.h"

#include "mme-context.h"
#include "dauth-mme-c-binding.h"
#include "dauth-mme-context-util.hpp"
#include "dauth-mme-local-auth-client.hpp"
#include "ogs-crypt.h"

#ifdef __cplusplus
extern "C" {
#endif

bool
handle_rpc_completion(void *tag) {
    ogs_debug("Handling tag %p", tag);
    dauth_mme::local_auth_client* client = reinterpret_cast<dauth_mme::local_auth_client*>(tag);
    return client->notify_rpc_complete();
}

bool
dauth_context_init(dauth_mme_context_t * const context) {
    std::shared_ptr<grpc::Channel> channel = grpc::CreateChannel("localhost:50051", grpc::InsecureChannelCredentials());

    dauth_mme::server_context * const internal_context = new dauth_mme::server_context(channel);

    ogs_assert(internal_context);
    if (!internal_context) {
        return false;
    }

    context->server_context = internal_context;
    return true;
}

bool
dauth_context_final(dauth_mme_context_t * const context) {
    ogs_assert(context->server_context);
    dauth_mme::server_context * internal_context = reinterpret_cast<dauth_mme::server_context*>(context->server_context);
    delete internal_context;
    context->server_context = nullptr;
    return true;
}

bool
wait_for_next_rpc_event(void** tag) {
    mme_context_t* ausf_context = mme_self();
    ogs_assert(ausf_context);
    return access_dauth_server_context(ausf_context->dauth_context).queueWaitNextRpcCompletion(tag);
}

void
grpc_client_shutdown(void) {
    mme_context_t* ausf_context = mme_self();
    ogs_assert(ausf_context);
    access_dauth_server_context(ausf_context->dauth_context).queueShutdown();
}

bool
mme_dauth_shim_request_auth_vector(
    mme_ue_t * const mme_ue
) {
    return mme_dauth_shim_request_auth_vector_resync(mme_ue, NULL);
}

bool
mme_dauth_shim_request_auth_vector_resync(
    mme_ue_t * const mme_ue,
    const ogs_nas_authentication_failure_parameter_t * const resync_info
) {
    ogs_error("Received dauth client request auth vector when currently unimplemented");

    // Allocate a new client for this UE if one does not exist already.
    if (mme_ue->dauth_context.local_auth_client == nullptr) {
        mme_context_t* mme_context = mme_self();
        ogs_assert(mme_context);

        mme_ue->dauth_context.local_auth_client = new dauth_mme::local_auth_client(
            access_dauth_server_context(mme_context->dauth_context).makeLocalAuthenticationStub(),
            &access_dauth_server_context(mme_context->dauth_context).completionQueue(),
            mme_ue->imsi,
            mme_ue->imsi_len
        );
        ogs_assert(mme_ue->dauth_context.local_auth_client);
        if (!mme_ue->dauth_context.local_auth_client) {
            return false;
        }
    }

    dauth_mme::local_auth_client& client = access_dauth_local_auth_client_context(mme_ue->dauth_context);

    if (client.in_progress()) {
        ogs_error("Received dauth client request when another request already in progress");
        if (!client.abort_current_state(mme_ue)) {
            return false;
        }
    }

    return client.request_auth_vector(mme_ue, resync_info);
}

bool
mme_dauth_shim_request_confirm_auth(
    mme_ue_t * const mme_ue,
    const uint8_t * const res
) {
    ogs_assert(mme_ue->dauth_context.local_auth_client);
    if (!mme_ue->dauth_context.local_auth_client) {
        return false;
    }

    dauth_mme::local_auth_client& client = access_dauth_local_auth_client_context(mme_ue->dauth_context);

    return client.request_confirm_auth(mme_ue, res);
}

// Compute res_hash from res
void mme_dauth_shim_compute_res_hash(uint8_t *rand, uint8_t *res, uint8_t res_len, uint8_t *res_hash)
{
    ogs_assert(rand);
    ogs_assert(res);
    ogs_assert(res_hash);
    ogs_assert(res_len <= OGS_MAX_RES_LEN);
    ogs_assert(OGS_SHA256_DIGEST_SIZE >= DAUTH_XRES_HASH_SIZE);

    uint8_t message[OGS_RAND_LEN + OGS_MAX_RES_LEN];
    uint8_t output[OGS_SHA256_DIGEST_SIZE];

    // Since res is variable length, be sure to consistently init the entire
    // message buffer.
    memset(message, 0, OGS_RAND_LEN + OGS_MAX_RES_LEN);
    memcpy(message, rand, OGS_RAND_LEN);
    memcpy(message+OGS_RAND_LEN, res, res_len);

    ogs_sha256(message, OGS_RAND_LEN+OGS_MAX_RES_LEN, output);

    // Use the least significant bits of the result.
    memcpy(res_hash, output+(OGS_SHA256_DIGEST_SIZE - DAUTH_XRES_HASH_SIZE), OGS_SHA256_DIGEST_SIZE);
}

#ifdef __cplusplus
}
#endif
