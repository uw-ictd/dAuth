mod test_core;
mod test_dauth;
mod test_directory;

pub use test_core::TestCore;
pub use test_dauth::TestDauth;
pub use test_directory::TestDirectory;

/// Known functional K.
pub const TEST_K: &str = "465B5CE8B199B49FAA5F0A2EE238A6BC";
/// Known functional OPC.
pub const TEST_OPC: &str = "E8ED289DEBA952E4283B54E88E6183CA";
