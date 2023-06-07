use tokio::sync::Mutex;
use tonic::transport::{Channel, Endpoint};
use tonic::Request;

use auth_vector::generate_vector_with_rand;
use auth_vector::{
    data::AuthVectorData,
    types::{ResStar, Sqn},
};

use dauth_service::data::error::DauthError;
use dauth_service::rpc::dauth::common::UserIdKind;
use dauth_service::rpc::dauth::local::local_authentication_client::LocalAuthenticationClient;
use dauth_service::rpc::dauth::local::{
    aka_confirm_req::Response, aka_confirm_resp::Key, AkaConfirmReq, AkaVectorReq,
};
use dauth_service::{data::config::UserInfoConfig, rpc::dauth::common::AuthVector5G};

use crate::{TEST_K, TEST_OPC};

/// Represents a test/mock version of a cellular core.
/// Can be used to request auth vectors, confirm keys, etc.
pub struct TestCore {
    /// Client for accessing the local dauth instance
    client: Mutex<LocalAuthenticationClient<Channel>>,
}

impl TestCore {
    pub async fn new(host: &str) -> Result<Self, DauthError> {
        let endpoint =
            Endpoint::from_shared(format!("http://{}:50051", host)).expect("Invalid host");
        let client = LocalAuthenticationClient::connect(endpoint).await?;

        Ok(Self {
            client: Mutex::new(client),
        })
    }

    /// Usex to rederive important auth info, like xres/xres*.
    /// This is necessary since requesting a confirm key requires
    /// res/xres, but the auth vector only contains the hash.
    ///
    /// This function utilizes the K and OPc values that are used
    /// for every user in the testing environment.
    fn rederive_auth_info(&self, auth_vector: &AuthVector5G) -> Result<AuthVectorData, DauthError> {
        let user_info = UserInfoConfig {
            user_id: "".to_string(), // Not needed
            k: TEST_K.to_string(),
            opc: TEST_OPC.to_string(),
            sqn_max: 0,      // Not needed
            backups: vec![], // Not needed
        };

        Ok(generate_vector_with_rand(
            "901",
            "70",
            &user_info.get_k()?,
            &user_info.get_opc()?,
            &auth_vector.rand[..].try_into()?,
            &Sqn::try_from(auth_vector.seqnum)?,
        ))
    }

    /// Requests an auth vector from the configured dauth instance.
    /// Returns the resulting value, or None if no auth vector could be
    /// retrieved.
    pub async fn request_auth(&self, user_id: &str) -> Result<AuthVector5G, DauthError> {
        let res = self
            .client
            .lock()
            .await
            .get_auth_vector(Request::new(AkaVectorReq {
                user_id_type: UserIdKind::Supi as i32,
                user_id: user_id.as_bytes().to_vec(),
                resync_info: None,
            }))
            .await?
            .into_inner()
            .auth_vector;

        match res {
            Some(av) => Ok(av),
            None => Err(DauthError::NotFoundError(
                "Failed to get auth vector".to_string(),
            )),
        }
    }

    /// Requests a confirm key from the configured dauth instance.
    /// Returns the resulting value, or None if no confirm key could be
    /// retrieved.
    pub async fn request_confirm(
        &self,
        user_id: &str,
        res_star: &ResStar,
    ) -> Result<Vec<u8>, DauthError> {
        let key = self
            .client
            .lock()
            .await
            .confirm_auth(Request::new(AkaConfirmReq {
                user_id_type: UserIdKind::Supi as i32,
                user_id: user_id.as_bytes().to_vec(),
                response: Some(Response::ResStar(res_star.to_vec())),
            }))
            .await?
            .into_inner()
            .key;

        if let Some(key) = key {
            match key {
                Key::Kseaf(res) => Ok(res),
                Key::Kasme(res) => {
                    // TODO: potentially treat 4G differently? Don't allow?
                    Ok(res)
                }
            }
        } else {
            Err(DauthError::NotFoundError(
                "Failed to get confirm key".to_string(),
            ))
        }
    }

    /// Attempts to run through the entire authentication process for a user,
    /// acting in the same way as a cellular core. Intended as the main external
    /// function for testing.
    pub async fn auth_user(&self, user_id: &str) -> Result<(), DauthError> {
        let av = self.request_auth(user_id).await?;
        let auth_info = self.rederive_auth_info(&av)?;

        match self.request_confirm(user_id, &auth_info.xres_star).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
