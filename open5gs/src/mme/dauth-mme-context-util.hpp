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

#ifndef __MME_DAUTH_MME_CONTEXT_UTIL_HPP__
#define __MME_DAUTH_MME_CONTEXT_UTIL_HPP__

#include "dauth-mme-c-binding.h"
#include "dauth-mme-server-context.hpp"
#include "dauth-mme-local-auth-client.hpp"

inline
dauth_mme::server_context&
access_dauth_server_context(const dauth_mme_context_t context) {
    ogs_assert(context.server_context);
    dauth_mme::server_context * internal_context = reinterpret_cast<dauth_mme::server_context*>(context.server_context);
    return (*internal_context);
}

inline
dauth_mme::local_auth_client&
access_dauth_local_auth_client_context(const dauth_mme_ue_context_t context) {
    ogs_assert(context.local_auth_client);
    dauth_mme::local_auth_client * internal_context = reinterpret_cast<dauth_mme::local_auth_client*>(context.local_auth_client);
    return (*internal_context);
}

#endif /* __AUSF_DAUTH_CONTEXT_UTIL_HPP__ */
