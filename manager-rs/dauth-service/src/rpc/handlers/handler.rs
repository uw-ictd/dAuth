use std::sync::Arc;

use crate::data::context::DauthContext;

/// Handles all RPC calls to the dAuth service.
pub struct DauthHandler {
    pub context: Arc<DauthContext>,
}
