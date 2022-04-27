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

#ifndef __AUSF_DAUTH_SERVER_CONTEXT_HPP__
#define __AUSF_DAUTH_SERVER_CONTEXT_HPP__

#include <memory>

#include <grpcpp/grpcpp.h>
#include "grpcpp/impl/codegen/completion_queue.h"
#include "local_authentication.grpc.pb.h"

class dauth_server_context {
public:
    dauth_server_context(
        std::shared_ptr<grpc::Channel> channel
    ):
        _channel(channel),
        _completion_queue()
    {}

    std::unique_ptr<dauth_local::LocalAuthentication::Stub>
    makeLocalAuthenticationStub(void);

    void
    queueShutdown(void);

    bool
    queueWaitNextRpcCompletion(void** tag);

    inline
    grpc::CompletionQueue&
    completionQueue(void) {
        return _completion_queue;
    }

private:
    std::shared_ptr<grpc::Channel> _channel;
    grpc::CompletionQueue _completion_queue;
};

#endif /* __AUSF_DAUTH_SERVER_CONTEXT_HPP__ */
