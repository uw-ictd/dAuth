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

#include <cstddef>
#include <grpcpp/grpcpp.h>
#include <memory>
#include <string.h>
#include <string>

#include "authentication_data.pb.h"
#include "dauth-mme-local-auth-client.hpp"
#include "grpcpp/impl/codegen/client_context.h"
#include "local_authentication.grpc.pb.h"
#include "local_authentication.pb.h"

#include "mme-event.h"
#include "nas-path.h"
#include "mme-sm.h"
#include "core/ogs-core.h"
#include "mme-context.h"
#include "dauth-mme-context-util.hpp"
#include "dauth-mme-c-binding.h"
#include "ogs-crypt.h"


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
    mme_ue_t * const mme_ue,
    const ogs_nas_authentication_failure_parameter_t * const resync_info
) {
    ogs_debug("[%s] Entering request auth vector", mme_ue->imsi_bcd);

    if (state_ != client_state::INIT) {
        ogs_error("Bad local client state [%d]", state_);
    }
    ogs_assert(state_ == client_state::INIT);

    // Construct an imsi-type SUPI from the UE's information.
    ogs_assert(mme_ue->imsi_bcd);
    std::string supi = "imsi-";
    supi.append(mme_ue->imsi_bcd);

    // Fill request protobuf
    ogs_debug("[%s] Filling d_auth::AKAVectorReq request", supi.c_str());
    auth_vector_req_.set_user_id(supi);
    auth_vector_req_.set_user_id_type(::d_auth::UserIdKind::SUPI);

    if(resync_info) {
        ogs_debug("[%s] Filling d_auth::AKAResyncInfo request auts", supi.c_str());
        auth_vector_req_.mutable_resync_info()->set_auts(resync_info->auts, resync_info->length);
        ogs_debug("[%s] Setting rand", supi.c_str());
        auth_vector_req_.mutable_resync_info()->set_rand(mme_ue->rand, OGS_RAND_LEN);
    }

    grpc_context_ = std::make_unique<grpc::ClientContext>();
    auth_vector_rpc_ = stub_->PrepareAsyncGetAuthVector(grpc_context_.get(), auth_vector_req_, completion_queue_);

    // Update state before sending externally visible call.
    state_ = client_state::WAITING_AUTH_RESP;

    ogs_info("[%s] Sending LocalAuthentication.GetAuthVector request to dauth", supi.c_str());
    auth_vector_rpc_->StartCall();
    auth_vector_rpc_->Finish(&auth_vector_resp_, &grpc_status_, this);

    return true;
}

// Moved and slightly tweaked from mme-s6a-handler::mme_s6a_handle_aia
bool
dauth_mme::local_auth_client::handle_request_auth_vector_res(
    mme_ue_t * const mme_ue
) {
    ogs_debug("[%s] Handling request auth vector response", mme_ue->imsi_bcd);

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
        ogs_assert(OGS_OK ==
                nas_eps_send_authentication_reject(mme_ue));
        OGS_FSM_TRAN(&mme_ue->sm, &emm_state_exception);
        // Trigger the state machine since we're transitioning states outside a
        // running event context. Possibly leaking the event? Not sure what the
        // error handling should be like also if the new event cannot be created for
        // some reason.
        mme_event_t * entry_event = mme_event_new((mme_event_e) 0);
        entry_event->mme_ue = mme_ue;
        ogs_fsm_dispatch(&mme_ue->sm, entry_event);
        state_ = client_state::INIT;
        return true;
    }

    ogs_debug("[%s] Handle request auth vector RPC completed ok, unpacking response", mme_ue->imsi_bcd);

    if (auth_vector_resp_.error() != AKAVectorResp_ErrorKind::AKAVectorResp_ErrorKind_NO_ERROR) {
        ogs_error(
            "[%s] LocalAuthentication.GetAuthVector RPC succeeded with error status [%d]",
            mme_ue->imsi_bcd,
            auth_vector_resp_.error()
        );
        ogs_assert(OGS_OK ==
                nas_eps_send_authentication_reject(mme_ue));
        OGS_FSM_TRAN(&mme_ue->sm, &emm_state_exception);
        // Trigger the state machine since we're transitioning states outside a
        // running event context. Possibly leaking the event? Not sure what the
        // error handling should be like also if the new event cannot be created for
        // some reason.
        mme_event_t * entry_event = mme_event_new((mme_event_e) 0);
        entry_event->mme_ue = mme_ue;
        ogs_fsm_dispatch(&mme_ue->sm, entry_event);
        state_ = client_state::INIT;
        return true;
    }

    // Debug sanity checks on size.
    ogs_assert(auth_vector_resp_.auth_vector().rand().length() == OGS_RAND_LEN);
    ogs_assert(auth_vector_resp_.auth_vector().xres_hash().length() == DAUTH_XRES_HASH_SIZE);
    ogs_assert(auth_vector_resp_.auth_vector().autn().length() == OGS_AUTN_LEN);

    ogs_assert(mme_ue);

    // Fill local UE state with data received from dauth daemon
    memcpy(mme_ue->xres_hash, auth_vector_resp_.auth_vector().xres_hash().c_str(), DAUTH_XRES_HASH_SIZE);
    memcpy(mme_ue->rand, auth_vector_resp_.auth_vector().rand().c_str(), sizeof(mme_ue->rand));
    memcpy(mme_ue->autn, auth_vector_resp_.auth_vector().autn().c_str(), auth_vector_resp_.auth_vector().autn().length());

    CLEAR_MME_UE_TIMER(mme_ue->t3460);

    if (mme_ue->nas_eps.ksi == OGS_NAS_KSI_NO_KEY_IS_AVAILABLE)
        mme_ue->nas_eps.ksi = 0;

    ogs_info("[%s] LocalAuthentication.GetAuthVector RPC Success, sending auth request to UE", mme_ue->imsi_bcd);

    state_ = client_state::AUTH_DONE;

    ogs_assert(OGS_OK ==
        nas_eps_send_authentication_request(mme_ue));

    return true;
}

bool
dauth_mme::local_auth_client::abort_current_state(
    mme_ue_t * const mme_ue
) {
    if (state_ == client_state::AUTH_DONE) {
        ogs_debug("Resetting state from AUTH_DONE since no messages in flight");
        state_ = client_state::INIT;
        return true;
    }

    ogs_warn("Tried to abort but cannot since in state {%d}", state_);
    return false;
}

bool
dauth_mme::local_auth_client::request_confirm_auth(
    mme_ue_t * const mme_ue,
    const uint8_t * const res,
    size_t res_len
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

    if(!res) {
        ogs_error("[%s] No res in confirm auth request", supi.c_str());
        return false;
    }

    // Fill request protobuf
    ogs_debug("[%s] Filling d_auth::AKAConfirmReq request", supi.c_str());
    confirm_auth_req_.set_user_id(supi);
    confirm_auth_req_.set_user_id_type(::d_auth::UserIdKind::SUPI);
    confirm_auth_req_.set_res(res, res_len);

    ogs_debug("[%s] Sending LocalAuthentication.ConfirmAuth request", supi.c_str());
    grpc_context_ = std::make_unique<grpc::ClientContext>();
    confirm_auth_rpc_ = stub_->PrepareAsyncConfirmAuth(grpc_context_.get(), confirm_auth_req_, completion_queue_);

    // Update state before sending externally visible request.
    state_ = client_state::WAITING_CONFIRM_RESP;

    ogs_info("[%s] Sending LocalAuthentication.ConfirmAuth request to dauth", supi.c_str());
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
        return false;
    }

    // Handle failure
    if (!grpc_status_.ok()) {
        ogs_error(
            "[%s] LocalAuthentication.GetAuthVector RPC Failed with status [%d]:%s",
            mme_ue->imsi_bcd,
            grpc_status_.error_code(),
            grpc_status_.error_message().c_str()
        );

        ogs_assert(OGS_OK ==
                nas_eps_send_authentication_reject(mme_ue));
        OGS_FSM_TRAN(&mme_ue->sm, &emm_state_exception);
        // Trigger the state machine since we're transitioning states outside a
        // running event context. Possibly leaking the event? Not sure what the
        // error handling should be like also if the new event cannot be created for
        // some reason.
        mme_event_t * entry_event = mme_event_new((mme_event_e) 0);
        entry_event->mme_ue = mme_ue;
        ogs_fsm_dispatch(&mme_ue->sm, entry_event);
        state_ = client_state::INIT;

        return true;
    }

    ogs_assert(confirm_auth_resp_.has_kasme());
    ogs_assert(confirm_auth_resp_.kasme().length() == OGS_SHA256_DIGEST_SIZE);
    memcpy(mme_ue->kasme, confirm_auth_resp_.kasme().c_str(), confirm_auth_resp_.kasme().length());

    ogs_info("[%s] LocalAuthentication.ConfirmAuth RPC Success. Transitioning to security mode update state", mme_ue->imsi_bcd);
    // Update state before sending externally visible response.
    state_ = client_state::DONE;
    OGS_FSM_TRAN(&mme_ue->sm, &emm_state_security_mode);
    // Trigger the state machine since we're transitioning states outside a
    // running event context. Possibly leaking the event? Not sure what the
    // error handling should be like also if the new event cannot be created for
    // some reason.
    mme_event_t * entry_event = mme_event_new((mme_event_e) 0);
    entry_event->mme_ue = mme_ue;
    ogs_fsm_dispatch(&mme_ue->sm, entry_event);
    state_ = client_state::INIT;

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
            return false;
    }

    ogs_assert(false);
}

bool
dauth_mme::local_auth_client::in_progress(void) {
    return (state_ != client_state::INIT);
}
