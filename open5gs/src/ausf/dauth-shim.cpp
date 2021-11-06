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

#include "core/ogs-core.h"

#include "example.grpc.pb.h"
#include "example.pb.h"

#ifdef __cplusplus
extern "C" {
#endif

bool ausf_dauth_shim_request_auth_vector(void) {
    // TODO(matt9j) Move to a one-time context instead of re-opening each time and making new stubs
    std::shared_ptr<grpc::Channel> channel = grpc::CreateChannel("localhost:50051", grpc::InsecureChannelCredentials());

    example_proto::HelloReq request;
    request.set_message("Fishsticks");
    example_proto::HelloResp response;

    grpc::ClientContext context;

    std::unique_ptr<example_proto::ExampleCall::Stub> stub = example_proto::ExampleCall::NewStub(channel);

    grpc::Status status = stub->SayHello(&context, request, &response);

    // Handle failure
    if (!status.ok()) {
        std::cout << "Failure :.(" <<
            status.error_code() <<
            ": " <<
            status.error_message() <<
            std::endl;
        return false;
    }
    ogs_info("RPC Success! %s", response.response().c_str());
    return true;
}

#ifdef __cplusplus
}
#endif