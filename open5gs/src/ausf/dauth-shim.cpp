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
#include "core/ogs-core.h"
#include "model/authentication_info.h"
#include "model/authentication_vector.h"

#include "local_authentication.grpc.pb.h"
#include "local_authentication.pb.h"

using namespace dauth_local;

// Utility function to compute the length of a c-style null terminated string
// with a maximum possible length.
size_t
bounded_strlen(const char * const str, size_t max_length) {
    const char * const end_pointer = static_cast<const char *>(memchr(str, '\0', max_length));
    if (end_pointer == NULL) {
        return max_length;
    }
    return static_cast<size_t>(end_pointer - str);
}

#ifdef __cplusplus
extern "C" {
#endif

bool
ausf_dauth_shim_request_auth_vector(
    const char * const supi,
    const OpenAPI_authentication_info_t * const authentication_info,
    OpenAPI_authentication_vector_t * const received_vector
) {
    if(!supi) {
        ogs_error("Null supi in auth vector request");
        return false;
    }
    size_t supi_length = bounded_strlen(supi, 128);
    if((supi_length == 0) || (supi_length == 128)) {
        ogs_error("Supi string is malformed");
        return false;
    }
    if(!authentication_info) {
        ogs_error("[%s] No AuthenticationInfo in request", supi);
        return false;
    }

    // TODO(matt9j) Move to a one-time context instead of re-opening each time and making new stubs
    ogs_debug("[%s] Creating gRPC LocalAuthentication stub", supi);
    std::shared_ptr<grpc::Channel> channel = grpc::CreateChannel("localhost:50051", grpc::InsecureChannelCredentials());
    std::unique_ptr<LocalAuthentication::Stub> stub = LocalAuthentication::NewStub(channel);

    // Fill request protobuf
    ogs_debug("[%s] Filling d_auth::AKAVectorReq request", supi);
    AKAVectorReq request;
    request.set_user_id(supi, supi_length);
    request.set_user_id_type(::d_auth::UserIdKind::SUPI);

    d_auth::AKAResyncInfo resync_info;
    if(authentication_info->resynchronization_info) {
        ogs_debug("[%s] Filling d_auth::AKAResyncInfo request", supi);
        resync_info.set_auts(authentication_info->resynchronization_info->auts);
        resync_info.set_auts(authentication_info->resynchronization_info->rand);
        request.set_allocated_resync_info(&resync_info);
    }

    // Allocate response and context memory on the stack
    AKAVectorResp response;
    grpc::ClientContext context;

    ogs_debug("[%s] Sending LocalAuthentication.GetAuthVector request", supi);
    grpc::Status status = stub->GetAuthVector(&context, request, &response);

    // Handle failure
    if (!status.ok()) {
        ogs_error(
            "[%s] LocalAuthentication.GetAuthVector RPC Failed with status [%d]:%s",
            supi,
            status.error_code(),
            status.error_message().c_str()
        );
        return false;
    }
    ogs_info("[%s] LocalAuthentication.GetAuthVector RPC Success", supi);

    // TODO(matt9j) Do the forwarding here...

    return true;
}

#ifdef __cplusplus
}
#endif