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
#include "dauth-local-auth-client.hpp"
#include "grpcpp/impl/codegen/client_context.h"
#include "local_authentication.grpc.pb.h"
#include "local_authentication.pb.h"

#include "core/ogs-core.h"
#include "context.h"
#include "dauth-context-util.hpp"
#include "dauth-c-binding.h"
#include "model/authentication_info.h"
#include "model/authentication_vector.h"


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

bool
dauth_local_auth_client::request_auth_vector(
    const char * const supi,
    const OpenAPI_authentication_info_t * const authentication_info,
    ogs_sbi_stream_t *stream
) {
    if (state_ != client_state::INIT) {
        ogs_error("Bad local client state [%d]", state_);
    }
    ogs_assert(state_ == client_state::INIT);

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

    // Fill request protobuf
    ogs_debug("[%s] Filling d_auth::AKAVectorReq request", supi);
    auth_vector_req_.set_user_id(supi, supi_length);
    auth_vector_req_.set_user_id_type(::d_auth::UserIdKind::SUPI);

    if(authentication_info->resynchronization_info) {
        ogs_debug("[%s] Filling d_auth::AKAResyncInfo request", supi);
        resync_info_.set_auts(authentication_info->resynchronization_info->auts);
        resync_info_.set_auts(authentication_info->resynchronization_info->rand);
        auth_vector_req_.set_allocated_resync_info(&resync_info_);
    }

    ogs_debug("[%s] Sending LocalAuthentication.GetAuthVector request", supi);
    grpc_context_ = std::make_unique<grpc::ClientContext>();
    auth_vector_rpc_ = stub_->PrepareAsyncGetAuthVector(grpc_context_.get(), auth_vector_req_, completion_queue_);

    // Update state before sending externally visible call.
    state_ = client_state::WAITING_AUTH_RESP;
    pending_stream_ = stream;

    auth_vector_rpc_->StartCall();
    auth_vector_rpc_->Finish(&auth_vector_resp_, &grpc_status_, this);

    return true;
}

// Moved and slightly tweaked from nudm_handler::ausf_nudm_ueau_handle_get
bool
dauth_local_auth_client::handle_request_auth_vector_res(
    ausf_ue_t * const ausf_ue
) {
    if (state_ != client_state::WAITING_AUTH_RESP) {
        ogs_error("Bad local client state [%d]", state_);
    }
    ogs_assert(state_ == client_state::WAITING_AUTH_RESP);

    // Handle failure
    if (!grpc_status_.ok()) {
        ogs_error(
            "[%s] LocalAuthentication.GetAuthVector RPC Failed with status [%d]:%s",
            ausf_ue->supi,
            grpc_status_.error_code(),
            grpc_status_.error_message().c_str()
        );
        return false;
    }
    ogs_info("[%s] LocalAuthentication.GetAuthVector RPC Success", ausf_ue->supi);

    if (auth_vector_resp_.error() != AKAVectorResp_ErrorKind::AKAVectorResp_ErrorKind_NO_ERROR) {
        ogs_error(
            "[%s] LocalAuthentication.GetAuthVector RPC succeeded with error status [%d]",
            ausf_ue->supi,
            auth_vector_resp_.error()
        );
        return false;
    }

    // Debug sanity checks on size.
    ogs_assert(auth_vector_resp_.auth_vector().rand().length() == OGS_RAND_LEN);
    ogs_assert(auth_vector_resp_.auth_vector().xres_star_hash().length() == OGS_MAX_RES_LEN);
    ogs_assert(auth_vector_resp_.auth_vector().autn().length() == OGS_AUTN_LEN);

    ogs_sbi_server_t *server = NULL;

    ogs_sbi_message_t sendmsg;
    ogs_sbi_header_t header;
    ogs_sbi_response_t *response = NULL;

    char hxres_star_string[OGS_KEYSTRLEN(OGS_MAX_RES_LEN)];
    char rand_string[OGS_KEYSTRLEN(OGS_RAND_LEN)];
    char autn_string[OGS_KEYSTRLEN(OGS_AUTN_LEN)];

    OpenAPI_ue_authentication_ctx_t UeAuthenticationCtx;
    OpenAPI_av5g_aka_t AV5G_AKA;
    OpenAPI_map_t *LinksValueScheme = NULL;
    OpenAPI_links_value_schema_t LinksValueSchemeValue;

    ogs_assert(ausf_ue);
    ogs_assert(pending_stream_);
    server = ogs_sbi_server_from_stream(pending_stream_);
    ogs_assert(server);

    ausf_ue->auth_type = OpenAPI_auth_type_5G_AKA;

    memcpy(ausf_ue->rand, auth_vector_resp_.auth_vector().rand().c_str(), sizeof(ausf_ue->rand));
    memcpy(ausf_ue->hxres_star, auth_vector_resp_.auth_vector().xres_star_hash().c_str(), sizeof(ausf_ue->hxres_star));
    // NOTE: Missing kausf, which open5gs has received from the UDM at this point
    char autn_byte_buffer[OGS_AUTN_LEN];
    memcpy(autn_byte_buffer, auth_vector_resp_.auth_vector().autn().c_str(), auth_vector_resp_.auth_vector().autn().length());

    memset(&UeAuthenticationCtx, 0, sizeof(UeAuthenticationCtx));

    UeAuthenticationCtx.auth_type = ausf_ue->auth_type;

    // Convert received binary crypto values to ascii strings of hex values as
    // needed for the SBI interface.
    memset(&AV5G_AKA, 0, sizeof(AV5G_AKA));
    ogs_hex_to_ascii(
        ausf_ue->rand,
        sizeof(ausf_ue->rand),
        rand_string,
        sizeof(rand_string)
    );
    AV5G_AKA.rand = rand_string;

    ogs_hex_to_ascii(
        autn_byte_buffer,
        sizeof(autn_byte_buffer),
        autn_string,
        sizeof(autn_string)
    );
    AV5G_AKA.autn = autn_string;

    ogs_hex_to_ascii(
        ausf_ue->hxres_star,
        sizeof(ausf_ue->hxres_star),
        hxres_star_string,
        sizeof(hxres_star_string)
    );
    AV5G_AKA.hxres_star = hxres_star_string;

    UeAuthenticationCtx._5g_auth_data = &AV5G_AKA;

    memset(&LinksValueSchemeValue, 0, sizeof(LinksValueSchemeValue));

    memset(&header, 0, sizeof(header));
    header.service.name = (char *)OGS_SBI_SERVICE_NAME_NAUSF_AUTH;
    header.api.version = (char *)OGS_SBI_API_V1;
    header.resource.component[0] =
            (char *)OGS_SBI_RESOURCE_NAME_UE_AUTHENTICATIONS;
    header.resource.component[1] = ausf_ue->ctx_id;
    header.resource.component[2] =
            (char *)OGS_SBI_RESOURCE_NAME_5G_AKA_CONFIRMATION;
    LinksValueSchemeValue.href = ogs_sbi_server_uri(server, &header);
    LinksValueScheme = OpenAPI_map_create(
        const_cast<char*>(OGS_SBI_RESOURCE_NAME_5G_AKA),
        &LinksValueSchemeValue);

    UeAuthenticationCtx._links = OpenAPI_list_create();
    OpenAPI_list_add(UeAuthenticationCtx._links, LinksValueScheme);

    memset(&sendmsg, 0, sizeof(sendmsg));

    memset(&header, 0, sizeof(header));
    header.service.name = (char *)OGS_SBI_SERVICE_NAME_NAUSF_AUTH;
    header.api.version = (char *)OGS_SBI_API_V1;
    header.resource.component[0] =
            (char *)OGS_SBI_RESOURCE_NAME_UE_AUTHENTICATIONS;
    header.resource.component[1] = ausf_ue->ctx_id;

    sendmsg.http.location = ogs_sbi_server_uri(server, &header);
    sendmsg.http.content_type = (char *)OGS_SBI_CONTENT_3GPPHAL_TYPE;

    sendmsg.UeAuthenticationCtx = &UeAuthenticationCtx;

    response = ogs_sbi_build_response(&sendmsg,
        OGS_SBI_HTTP_STATUS_CREATED);
    ogs_assert(response);

    // Update state before sending externally visible response.
    state_ = client_state::AUTH_DONE;
    ogs_assert(true == ogs_sbi_server_send_response(pending_stream_, response));

    OpenAPI_list_free(UeAuthenticationCtx._links);
    OpenAPI_map_free(LinksValueScheme);

    ogs_free(LinksValueSchemeValue.href);
    ogs_free(sendmsg.http.location);
    pending_stream_ = nullptr;

    return true;
}

bool
dauth_local_auth_client::request_confirm_auth(
    ausf_ue_t * const ausf_ue,
    const uint8_t * const res_star,
    ogs_sbi_stream_t *stream
) {
    if (state_ != client_state::AUTH_DONE) {
        ogs_error("Bad local client state [%d]", state_);
    }
    ogs_assert(state_ == client_state::AUTH_DONE);

    if(!ausf_ue) {
        ogs_error("Null UE in auth confirm request");
        return false;
    }

    const char* const supi = ausf_ue->supi;
    if(!supi) {
        ogs_error("Null supi in auth confirm request");
        return false;
    }

    size_t supi_length = bounded_strlen(supi, 128);
    if((supi_length == 0) || (supi_length == 128)) {
        ogs_error("Supi string is malformed");
        return false;
    }
    if(!res_star) {
        ogs_error("[%s] No res_star in confirm auth request", supi);
        return false;
    }

    // Fill request protobuf
    ogs_debug("[%s] Filling d_auth::AKAConfirmReq request", supi);
    confirm_auth_req_.set_user_id(supi, supi_length);
    confirm_auth_req_.set_user_id_type(::d_auth::UserIdKind::SUPI);
    confirm_auth_req_.set_res_star(res_star, OGS_MAX_RES_LEN);

    ogs_debug("[%s] Sending LocalAuthentication.ConfirmAuth request", supi);
    grpc_context_ = std::make_unique<grpc::ClientContext>();
    confirm_auth_rpc_ = stub_->PrepareAsyncConfirmAuth(grpc_context_.get(), confirm_auth_req_, completion_queue_);

    // Update state before sending externally visible request.
    state_ = client_state::WAITING_CONFIRM_RESP;
    pending_stream_ = stream;

    confirm_auth_rpc_->StartCall();
    confirm_auth_rpc_->Finish(&confirm_auth_resp_, &grpc_status_, this);

    return true;
}

// Moved and slightly tweaked from nudm_handler::ausf_nudm_ueau_handle_result_confirmation_inform
bool
dauth_local_auth_client::handle_request_confirm_auth_res(
    ausf_ue_t * const ausf_ue
) {
    if (state_ != client_state::WAITING_CONFIRM_RESP) {
        ogs_error("Bad local client state [%d]", state_);
    }
    ogs_assert(state_ == client_state::WAITING_CONFIRM_RESP);

    // Handle failure
    if (!grpc_status_.ok()) {
        ogs_error(
            "[%s] LocalAuthentication.GetAuthVector RPC Failed with status [%d]:%s",
            ausf_ue->supi,
            grpc_status_.error_code(),
            grpc_status_.error_message().c_str()
        );
        return false;
    }
    ogs_info("[%s] LocalAuthentication.ConfirmAuth RPC Success", ausf_ue->supi);

    // TODO(matt9j) check if the res actually matches the hashed xres, and set
    // the auth result accordingly.
    // if (AuthEvent->success == true)
    //     ausf_ue->auth_result = OpenAPI_auth_result_AUTHENTICATION_SUCCESS;
    // else
    //     ausf_ue->auth_result = OpenAPI_auth_result_AUTHENTICATION_FAILURE;

    // Store the supplied kseaf in the local ue context
    ogs_assert(confirm_auth_resp_.kseaf().length() == 32);
    memcpy(ausf_ue->kseaf, confirm_auth_resp_.kseaf().c_str(), confirm_auth_resp_.kseaf().length());
    ausf_ue->auth_result = OpenAPI_auth_result_AUTHENTICATION_SUCCESS;

    ogs_sbi_message_t sendmsg;
    ogs_sbi_response_t *response = NULL;

    char kseaf_string[OGS_KEYSTRLEN(OGS_SHA256_DIGEST_SIZE)];

    OpenAPI_confirmation_data_response_t ConfirmationDataResponse;

    ogs_assert(ausf_ue);
    ogs_assert(pending_stream_);

    memset(&ConfirmationDataResponse, 0, sizeof(ConfirmationDataResponse));

    ConfirmationDataResponse.auth_result = ausf_ue->auth_result;
    ConfirmationDataResponse.supi = ausf_ue->supi;

    // TODO(matt9j) Double check kseaf derivation on the rust side of the world.

    // ogs_kdf_kseaf(ausf_ue->serving_network_name,
    //         ausf_ue->kausf, ausf_ue->kseaf);
    ogs_hex_to_ascii(ausf_ue->kseaf, sizeof(ausf_ue->kseaf),
            kseaf_string, sizeof(kseaf_string));
    ConfirmationDataResponse.kseaf = kseaf_string;

    memset(&sendmsg, 0, sizeof(sendmsg));

    sendmsg.ConfirmationDataResponse = &ConfirmationDataResponse;

    response = ogs_sbi_build_response(&sendmsg, OGS_SBI_HTTP_STATUS_OK);
    ogs_assert(response);

    // Update state before sending externally visible response.
    state_ = client_state::DONE;
    ogs_assert(true == ogs_sbi_server_send_response(pending_stream_, response));
    pending_stream_ = nullptr;
    state_ = client_state::INIT;

    return true;
}

bool
dauth_local_auth_client::notify_rpc_complete(void) {
    // Lookup UE by ID in case table was rehashed
    ausf_ue_t *ausf_ue = ausf_ue_find_by_suci(ue_suci_.c_str());
    ogs_assert(ausf_ue);

    switch (state_) {
        case WAITING_AUTH_RESP: {
            return handle_request_auth_vector_res(ausf_ue);
        }
        case WAITING_CONFIRM_RESP: {
            return handle_request_confirm_auth_res(ausf_ue);
        }
        case INIT:
        case AUTH_DONE:
        case DONE:
        default:
            ogs_error("Received rpc completion notify in bad state [%d]", state_);
            ogs_assert(false);
    }

    return false;
}

bool
dauth_local_auth_client::in_progress(void) {
    return (state_ != client_state::INIT);
}
