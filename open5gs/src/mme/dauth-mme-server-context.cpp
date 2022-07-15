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

#include "mme-context.h"
#include "dauth-mme-server-context.hpp"

std::unique_ptr<dauth_local::LocalAuthentication::Stub>
dauth_mme::server_context::makeLocalAuthenticationStub(void) {
    std::unique_ptr<dauth_local::LocalAuthentication::Stub> stub = dauth_local::LocalAuthentication::NewStub(_channel);
    return stub;
}

void
dauth_mme::server_context::queueShutdown(void) {
    _completion_queue.Shutdown();
}

bool
dauth_mme::server_context::queueWaitNextRpcCompletion(void** tag) {
    bool ok = false;
    bool queue_success = _completion_queue.Next(tag, &ok);
    if (queue_success) {
        ogs_assert(ok);
    }
    return queue_success;
}
