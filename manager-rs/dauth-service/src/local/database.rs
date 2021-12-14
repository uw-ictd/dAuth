use std::sync::Arc;

use auth_vector::types::{ResStar, Kseaf};

use crate::data::{
    context::DauthContext,
    error::DauthError,
    vector::{AuthVectorReq, AuthVectorRes},
};

/// Removes and returns vector if at least one exists.
pub fn auth_vector_next(
    context: Arc<DauthContext>,
    av_request: &AuthVectorReq,
) -> Result<AuthVectorRes, DauthError> {
    tracing::info!("Database next: {:?}", av_request);

    match context
        .local_context
        .database
        .lock()
        .unwrap()
        .get_mut(&av_request.user_id)
    {
        Some(queue) => match queue.pop_front() {
            Some(av_result) => {
                tracing::info!("Auth vector found: {:?}", av_result);
                Ok(av_result)
            }
            None => Err(DauthError::NotFoundError(format!("No vectors found"))),
        },
        None => {
            tracing::warn!("User not in database: {:?}", av_request);
            Err(DauthError::NotFoundError(format!("User not in database")))
        }
    }
}

/// Deletes a vector if found.
pub fn auth_vector_delete(
    _context: Arc<DauthContext>,
    av_result: &AuthVectorRes,
) -> Result<(), DauthError> {
    tracing::info!("Database delete: {:?}", av_result);
    todo!()
}

/// Removes and returns a kseaf value
pub fn kseaf_get(
    context: Arc<DauthContext>,
    xres_star_hash: &ResStar,
) -> Result<Kseaf, DauthError> {
    tracing::info!("Kseaf get: {:?}", xres_star_hash);

    let mut map = context.local_context.kseaf_map.lock().unwrap();

    match map.get(xres_star_hash) {
        Some(kseaf) => {
            let kseaf = kseaf.clone();
            map.remove(xres_star_hash);
            Ok(kseaf)
        }
        None => {
            tracing::error!("KSEAF not found with UUID: {:?}", xres_star_hash);
            Err(DauthError::NotFoundError(format!(
                "KSEAF not found with UUID: {:?}",
                xres_star_hash
            )))
        }
    }
}

/// Adds a kseaf value with the given xres_star_hash
pub fn kseaf_put(context: Arc<DauthContext>, xres_star: &ResStar, kseaf: &Kseaf) {
    tracing::info!("Kseaf put: {:?} - {:?}", xres_star, kseaf);
    context
        .local_context
        .kseaf_map
        .lock()
        .unwrap()
        .insert(xres_star.clone(), kseaf.clone());
}
