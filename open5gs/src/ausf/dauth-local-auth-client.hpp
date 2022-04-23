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

#ifndef __AUSF_DAUTH_LOCAL_AUTH_CLIENT_HPP__
#define __AUSF_DAUTH_LOCAL_AUTH_CLIENT_HPP__


#include "grpcpp/impl/codegen/async_unary_call.h"
#include "grpcpp/impl/codegen/completion_queue.h"
#include "ogs-app.h"
#include "ogs-crypt.h"
#include "ogs-sbi.h"

#include "event.h"

#include <memory>

#include <grpcpp/grpcpp.h>
#include <string>
#include "local_authentication.grpc.pb.h"
#include "local_authentication.pb.h"

typedef struct dauth_shim_vector {
    uint8_t rand[OGS_RAND_LEN];
    uint8_t xres_star_hash[OGS_MAX_RES_LEN];
    uint8_t autn[OGS_AUTN_LEN];
} dauth_shim_vector_t;

enum client_state{
    INIT,
    WAITING_AUTH_RESP,
    AUTH_DONE,
    WAITING_CONFIRM_RESP,
    DONE
} ;


class dauth_local_auth_client {
public:
    dauth_local_auth_client(
        std::unique_ptr<dauth_local::LocalAuthentication::Stub> stub,
        grpc::CompletionQueue* queue,
        char* suci
    ):
        state_(client_state::INIT),
        ue_suci_(suci),
        pending_stream_(nullptr),

        stub_(std::move(stub)),
        completion_queue_(queue),
        grpc_status_(),
        grpc_context_(),

        auth_vector_req_(),
        resync_info_(),
        auth_vector_resp_(),
        confirm_auth_req_(),
        confirm_auth_resp_(),

        auth_vector_rpc_(),
        confirm_auth_rpc_()
    {}

    bool
    request_auth_vector(
        const char * const supi,
        const OpenAPI_authentication_info_t * const authentication_info,
        ogs_sbi_stream_t *stream
    );

    bool
    handle_request_auth_vector_res(
        ausf_ue_t * const ausf_ue
    );

    bool
    request_confirm_auth(
        ausf_ue_t * const ausf_ue,
        const uint8_t * const res_star,
        ogs_sbi_stream_t *stream
    );

    bool
    handle_request_confirm_auth_res(
        ausf_ue_t * const ausf_ue
    );

    bool
    notify_rpc_complete(void);

private:
    client_state state_;
    std::string ue_suci_;
    ogs_sbi_stream_t * pending_stream_;

    std::unique_ptr<dauth_local::LocalAuthentication::Stub> stub_;
    // TODO(matt9j) Consider making an explicit shared pointer.
    grpc::CompletionQueue* completion_queue_;
    grpc::Status grpc_status_;
    std::unique_ptr<grpc::ClientContext> grpc_context_;

    // Allocate all messages ahead of time to allow asynchronous read/fill.
    dauth_local::AKAVectorReq auth_vector_req_;
    d_auth::AKAResyncInfo resync_info_;
    dauth_local::AKAVectorResp auth_vector_resp_;
    dauth_local::AKAConfirmReq confirm_auth_req_;
    dauth_local::AKAConfirmResp confirm_auth_resp_;

    std::unique_ptr<grpc::ClientAsyncResponseReader<dauth_local::AKAVectorResp>> auth_vector_rpc_;
    std::unique_ptr<grpc::ClientAsyncResponseReader<dauth_local::AKAConfirmResp>> confirm_auth_rpc_;
};


#endif /* __AUSF_DAUTH_LOCAL_AUTH_CLIENT_HPP__ */
