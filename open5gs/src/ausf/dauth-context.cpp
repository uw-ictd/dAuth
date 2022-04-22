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

#include "ogs-app.h"

#include "authentication_data.pb.h"
#include "local_authentication.grpc.pb.h"
#include "local_authentication.pb.h"

#include "dauth-context-c-binding.h"
#include "dauth-context.hpp"


#ifdef __cplusplus
extern "C" {
#endif

bool
dauth_context_init(dauth_context_t * context) {
    ogs_assert(context == nullptr);

    context = new dauth_context_t;
    ogs_assert(context);
    if (!context) {
        return false;
    }

    std::shared_ptr<grpc::Channel> channel = grpc::CreateChannel("localhost:50051", grpc::InsecureChannelCredentials());
    dauth_context * internal_context = new dauth_context(channel);

    ogs_assert(internal_context);
    if (!internal_context) {
        delete context;
        return false;
    }

    context->dauth_context_internal = internal_context;
    return true;
}

bool
dauth_context_final(dauth_context_t * const context) {
    ogs_assert(context->dauth_context_internal);
    dauth_context * internal_context = reinterpret_cast<dauth_context*>(context->dauth_context_internal);
    delete internal_context;
    context->dauth_context_internal = nullptr;
    return true;
}

#ifdef __cplusplus
}
#endif

std::unique_ptr<dauth_local::LocalAuthentication::Stub>
dauth_context::makeLocalAuthenticationStub() {
    std::unique_ptr<dauth_local::LocalAuthentication::Stub> stub = dauth_local::LocalAuthentication::NewStub(_channel);
}

dauth_context&
access_dauth_context_internal(dauth_context_t * const context) {
    ogs_assert(context->dauth_context_internal);
    dauth_context * internal_context = reinterpret_cast<dauth_context*>(context->dauth_context_internal);
    return (*internal_context);
}
