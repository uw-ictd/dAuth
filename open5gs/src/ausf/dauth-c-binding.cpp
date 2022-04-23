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

#include "core/ogs-core.h"
#include "context.h"
#include "dauth-context-util.hpp"
#include "dauth-c-binding.h"
#include "model/authentication_info.h"
#include "model/authentication_vector.h"


using namespace dauth_local;

#ifdef __cplusplus
extern "C" {
#endif

bool
handle_rpc_completion(void *tag) {
    ogs_info("Handling tag %p", tag);
    return true;
}

bool
dauth_context_init(dauth_context_t * const context) {
    std::shared_ptr<grpc::Channel> channel = grpc::CreateChannel("localhost:50051", grpc::InsecureChannelCredentials());

    dauth_server_context * const internal_context = new dauth_server_context(channel);

    ogs_assert(internal_context);
    if (!internal_context) {
        return false;
    }

    context->server_context = internal_context;
    return true;
}

bool
dauth_context_final(dauth_context_t * const context) {
    ogs_assert(context->server_context);
    dauth_server_context * internal_context = reinterpret_cast<dauth_server_context*>(context->server_context);
    delete internal_context;
    context->server_context = nullptr;
    return true;
}

bool
wait_for_next_rpc_event(void** tag) {
    ausf_context_t* ausf_context = ausf_self();
    ogs_assert(ausf_context);
    return access_dauth_server_context(ausf_context->dauth_context).queueWaitNextRpcCompletion(tag);
}

void
grpc_client_shutdown(void) {
    ausf_context_t* ausf_context = ausf_self();
    ogs_assert(ausf_context);
    access_dauth_server_context(ausf_context->dauth_context).queueShutdown();
}

bool
ausf_dauth_shim_request_auth_vector(
    ausf_ue_t * const ausf_ue,
    const OpenAPI_authentication_info_t * const authentication_info
) {
    return false;
}

bool
ausf_dauth_shim_forward_received_auth_vector(
    ausf_ue_t * const ausf_ue,
    ogs_sbi_stream_t *stream,
    const OpenAPI_authentication_info_t * const authentication_info
) {
    return false;
}

bool
ausf_dauth_shim_request_confirm_auth(
    ausf_ue_t * const ausf_ue,
    const uint8_t * const res_star
) {
    return false;
}

bool
ausf_dauth_shim_forward_confirmed_key(
    ausf_ue_t * const ausf_ue,
    ogs_sbi_stream_t *stream
) {
    return false;
}

#ifdef __cplusplus
}
#endif
