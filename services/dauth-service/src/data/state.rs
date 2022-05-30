use auth_vector::types::Rand;

#[derive(Clone, Debug)]
pub enum AuthSource {
    HomeNetwork,
    BackupNetwork,
}

#[derive(Clone, Debug)]
pub struct AuthState {
    pub rand: Rand,
    pub source: AuthSource,
}
