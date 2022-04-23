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


#include "ogs-app.h"
#include "ogs-crypt.h"
#include "ogs-sbi.h"

#include "event.h"

#include <memory>

#include <grpcpp/grpcpp.h>
#include "local_authentication.grpc.pb.h"

typedef struct dauth_shim_vector {
    uint8_t rand[OGS_RAND_LEN];
    uint8_t xres_star_hash[OGS_MAX_RES_LEN];
    uint8_t autn[OGS_AUTN_LEN];
} dauth_shim_vector_t;


class dauth_local_auth_client {
public:
    dauth_local_auth_client(

    )
    {}

    bool
    request_auth_vector(
        const char * const supi,
        const OpenAPI_authentication_info_t * const authentication_info
    );

    bool
    handle_request_auth_vector_res(
        ausf_ue_t * const ausf_ue,
        ogs_sbi_stream_t *stream,
        const OpenAPI_authentication_info_t * const authentication_info
    );

    bool
    request_confirm_auth(
        ausf_ue_t * const ausf_ue,
        const uint8_t * const res_star
    );

    bool
    handle_request_confirm_auth_res(
        ausf_ue_t * const ausf_ue,
        ogs_sbi_stream_t *stream
    );

private:
    dauth_shim_vector_t received_vector;

};


#endif /* __AUSF_DAUTH_LOCAL_AUTH_CLIENT_HPP__ */
