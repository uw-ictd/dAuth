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
#include <string>

#include "authentication_data.pb.h"
#include "dauth-mme-local-auth-client.hpp"
#include "grpcpp/impl/codegen/client_context.h"
#include "local_authentication.grpc.pb.h"
#include "local_authentication.pb.h"

#include "nas-path.h"
#include "mme-sm.h"
#include "core/ogs-core.h"
#include "mme-context.h"
#include "dauth-mme-context-util.hpp"
#include "dauth-mme-c-binding.h"


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
dauth_mme::local_auth_client::request_auth_vector(
    mme_ue_t * const mme_ue
) {
    if (state_ != client_state::INIT) {
        ogs_error("Bad local client state [%d]", state_);
    }
    ogs_assert(state_ == client_state::INIT);

    ogs_assert(mme_ue->imsi_bcd);
    std::string supi = "imsi-";
    supi.append(mme_ue->imsi_bcd);

    // Fill request protobuf
    ogs_debug("[%s] Filling d_auth::AKAVectorReq request", supi.c_str());
    auth_vector_req_.set_user_id(supi);
    auth_vector_req_.set_user_id_type(::d_auth::UserIdKind::SUPI);

    // TODO(matt9j) Add a resync hook
    // if(authentication_info->resynchronization_info) {
    //     ogs_debug("[%s] Filling d_auth::AKAResyncInfo request", supi);
    //     resync_info_.set_auts(authentication_info->resynchronization_info->auts);
    //     resync_info_.set_auts(authentication_info->resynchronization_info->rand);
    //     auth_vector_req_.set_allocated_resync_info(&resync_info_);
    // }

    ogs_debug("[%s] Sending LocalAuthentication.GetAuthVector request", supi.c_str());
    grpc_context_ = std::make_unique<grpc::ClientContext>();
    auth_vector_rpc_ = stub_->PrepareAsyncGetAuthVector(grpc_context_.get(), auth_vector_req_, completion_queue_);

    // Update state before sending externally visible call.
    state_ = client_state::WAITING_AUTH_RESP;

    auth_vector_rpc_->StartCall();
    auth_vector_rpc_->Finish(&auth_vector_resp_, &grpc_status_, this);

    return true;
}

// Moved and slightly tweaked from mme-s6a-handler::mme_s6a_handle_aia
bool
dauth_mme::local_auth_client::handle_request_auth_vector_res(
    mme_ue_t * const mme_ue
) {
    if (state_ != client_state::WAITING_AUTH_RESP) {
        ogs_error("Bad local client state [%d]", state_);
    }
    ogs_assert(state_ == client_state::WAITING_AUTH_RESP);

    // Handle failure
    if (!grpc_status_.ok()) {
        ogs_error(
            "[%s] LocalAuthentication.GetAuthVector RPC Failed with status [%d]:%s",
            mme_ue->imsi_bcd,
            grpc_status_.error_code(),
            grpc_status_.error_message().c_str()
        );
        return false;
    }
    ogs_info("[%s] LocalAuthentication.GetAuthVector RPC Success", mme_ue->imsi_bcd);

    if (auth_vector_resp_.error() != AKAVectorResp_ErrorKind::AKAVectorResp_ErrorKind_NO_ERROR) {
        ogs_error(
            "[%s] LocalAuthentication.GetAuthVector RPC succeeded with error status [%d]",
            mme_ue->imsi_bcd,
            auth_vector_resp_.error()
        );
        return false;
    }

    // Debug sanity checks on size.
    ogs_assert(auth_vector_resp_.auth_vector().rand().length() == OGS_RAND_LEN);
    ogs_assert(auth_vector_resp_.auth_vector().xres_star_hash().length() == OGS_MAX_RES_LEN);
    ogs_assert(auth_vector_resp_.auth_vector().autn().length() == OGS_AUTN_LEN);

    ogs_diam_e_utran_vector_t *e_utran_vector = NULL;

    ogs_assert(mme_ue);

    // TODO(matt9j) Need to fill with actual valid LTE values.
    mme_ue->xres_len = e_utran_vector->xres_len;
    memcpy(mme_ue->xres, auth_vector_resp_.auth_vector().xres_star_hash().c_str(), sizeof(mme_ue->xres));
    // memcpy(mme_ue->kasme, e_utran_vector->kasme, OGS_SHA256_DIGEST_SIZE);
    memcpy(mme_ue->rand, auth_vector_resp_.auth_vector().rand().c_str(), sizeof(mme_ue->rand));
    memcpy(mme_ue->autn, auth_vector_resp_.auth_vector().autn().c_str(), auth_vector_resp_.auth_vector().autn().length());

    CLEAR_MME_UE_TIMER(mme_ue->t3460);

    if (mme_ue->nas_eps.ksi == OGS_NAS_KSI_NO_KEY_IS_AVAILABLE)
        mme_ue->nas_eps.ksi = 0;

    ogs_assert(OGS_OK ==
        nas_eps_send_authentication_request(mme_ue));

    return true;
}

bool
dauth_mme::local_auth_client::request_confirm_auth(
    mme_ue_t * const mme_ue,
    const uint8_t * const res
) {
    if (state_ != client_state::AUTH_DONE) {
        ogs_error("Bad local client state [%d]", state_);
    }
    ogs_assert(state_ == client_state::AUTH_DONE);

    if(!mme_ue) {
        ogs_error("Null UE in auth confirm request");
        return false;
    }

    ogs_assert(mme_ue->imsi_bcd);
    std::string supi = "imsi-";
    supi.append(mme_ue->imsi_bcd);

    //TODO(matt9j) Need actual values.
    const uint8_t * const res_star = res;

    if(!res_star) {
        ogs_error("[%s] No res_star in confirm auth request", supi.c_str());
        return false;
    }

    // Fill request protobuf
    ogs_debug("[%s] Filling d_auth::AKAConfirmReq request", supi.c_str());
    confirm_auth_req_.set_user_id(supi);
    confirm_auth_req_.set_user_id_type(::d_auth::UserIdKind::SUPI);
    confirm_auth_req_.set_res_star(res_star, OGS_MAX_RES_LEN);

    ogs_debug("[%s] Sending LocalAuthentication.ConfirmAuth request", supi.c_str());
    grpc_context_ = std::make_unique<grpc::ClientContext>();
    confirm_auth_rpc_ = stub_->PrepareAsyncConfirmAuth(grpc_context_.get(), confirm_auth_req_, completion_queue_);

    // Update state before sending externally visible request.
    state_ = client_state::WAITING_CONFIRM_RESP;

    confirm_auth_rpc_->StartCall();
    confirm_auth_rpc_->Finish(&confirm_auth_resp_, &grpc_status_, this);

    return true;
}

// Moved and slightly tweaked from nudm_handler::ausf_nudm_ueau_handle_result_confirmation_inform
bool
dauth_mme::local_auth_client::handle_request_confirm_auth_res(
    mme_ue_t * const mme_ue
) {
    if (state_ != client_state::WAITING_CONFIRM_RESP) {
        ogs_error("Bad local client state [%d]", state_);
    }
    ogs_assert(state_ == client_state::WAITING_CONFIRM_RESP);

    // Handle failure
    if (!grpc_status_.ok()) {
        ogs_error(
            "[%s] LocalAuthentication.GetAuthVector RPC Failed with status [%d]:%s",
            mme_ue->imsi_bcd,
            grpc_status_.error_code(),
            grpc_status_.error_message().c_str()
        );
        return false;
    }
    ogs_info("[%s] LocalAuthentication.ConfirmAuth RPC Success", mme_ue->imsi_bcd);

    // TODO(matt9j) Update all stored ue key parameters... then...
    OGS_FSM_TRAN(&mme_ue->sm, &emm_state_security_mode);

    // TODO(matt9j) or if things fail...
    // ogs_assert(OGS_OK ==
    //             nas_eps_send_authentication_reject(mme_ue));
    // OGS_FSM_TRAN(&mme_ue->sm, &emm_state_exception);


    // TODO(matt9j) check if the res actually matches the hashed xres, and set
    // the auth result accordingly.
    // if (AuthEvent->success == true)
    //     ausf_ue->auth_result = OpenAPI_auth_result_AUTHENTICATION_SUCCESS;
    // else
    //     ausf_ue->auth_result = OpenAPI_auth_result_AUTHENTICATION_FAILURE;

    // Store the supplied kseaf in the local ue context

    // ogs_assert(confirm_auth_resp_.kseaf().length() == 32);
    // memcpy(mme_ue->kseaf, confirm_auth_resp_.kseaf().c_str(), confirm_auth_resp_.kseaf().length());
    // mme_ue->auth_result = OpenAPI_auth_result_AUTHENTICATION_SUCCESS;

    // ogs_sbi_message_t sendmsg;
    // ogs_sbi_response_t *response = NULL;

    // char kseaf_string[OGS_KEYSTRLEN(OGS_SHA256_DIGEST_SIZE)];

    // OpenAPI_confirmation_data_response_t ConfirmationDataResponse;

    // ogs_assert(mme_ue);
    // ogs_assert(pending_stream_);

    // memset(&ConfirmationDataResponse, 0, sizeof(ConfirmationDataResponse));

    // ConfirmationDataResponse.auth_result = ausf_ue->auth_result;
    // ConfirmationDataResponse.supi = ausf_ue->supi;

    // // TODO(matt9j) Double check kseaf derivation on the rust side of the world.

    // // ogs_kdf_kseaf(ausf_ue->serving_network_name,
    // //         ausf_ue->kausf, ausf_ue->kseaf);
    // ogs_hex_to_ascii(mme_ue->kseaf, sizeof(mme_ue->kseaf),
    //         kseaf_string, sizeof(kseaf_string));
    // ConfirmationDataResponse.kseaf = kseaf_string;

    // memset(&sendmsg, 0, sizeof(sendmsg));

    // sendmsg.ConfirmationDataResponse = &ConfirmationDataResponse;

    // response = ogs_sbi_build_response(&sendmsg, OGS_SBI_HTTP_STATUS_OK);
    // ogs_assert(response);

    // // Update state before sending externally visible response.
    // state_ = client_state::DONE;
    // ogs_assert(true == ogs_sbi_server_send_response(pending_stream_, response));
    // pending_stream_ = nullptr;
    // state_ = client_state::INIT;

    return true;
}

bool
dauth_mme::local_auth_client::notify_rpc_complete(void) {
    // Lookup UE by ID in case table was rehashed
    mme_ue_t *mme_ue = mme_ue_find_by_imsi(ue_imsi_, ue_imsi_len_);
    ogs_assert(mme_ue);

    switch (state_) {
        case WAITING_AUTH_RESP: {
            return handle_request_auth_vector_res(mme_ue);
        }
        case WAITING_CONFIRM_RESP: {
            return handle_request_confirm_auth_res(mme_ue);
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
dauth_mme::local_auth_client::in_progress(void) {
    return (state_ != client_state::INIT);
}
