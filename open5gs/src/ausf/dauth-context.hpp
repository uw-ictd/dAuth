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

#ifndef AUSF_DAUTH_CONTEXT_HPP
#define AUSF_DAUTH_CONTEXT_HPP

#include <grpcpp/grpcpp.h>
#include <memory>

#include "grpcpp/impl/codegen/completion_queue.h"
#include "local_authentication.grpc.pb.h"

#include "dauth-context-c-binding.h"

class dauth_context {
public:
    dauth_context(
        std::shared_ptr<grpc::Channel> channel
    ):
        _channel(channel),
        _completion_queue()
    {}

    std::unique_ptr<dauth_local::LocalAuthentication::Stub>
    makeLocalAuthenticationStub();

    void
    shutdownQueue();

    bool
    waitNextRpcCompletion(void** tag);

private:
    std::shared_ptr<grpc::Channel> _channel;
    grpc::CompletionQueue _completion_queue;
};

dauth_context&
access_dauth_context(dauth_context_t * const context);

#endif /* AUSF_DAUTH_CONTEXT_HPP */
