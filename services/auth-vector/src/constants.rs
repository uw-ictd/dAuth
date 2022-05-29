pub const MCC: &str = "901";
pub const MNC: &str = "70";

pub const FC_KAUSF: u8 = 0x6A;
pub const FC_KSEAF: u8 = 0x6C;

pub const ID_LENGTH: usize = 7;
pub const K_LENGTH: usize = 16;
pub const OPC_LENGTH: usize = 16;
pub const RAND_LENGTH: usize = 16;
pub const CK_LENGTH: usize = 16;
pub const IK_LENGTH: usize = 16;
pub const KAUSF_LENGTH: usize = 32;
pub const KSEAF_LENGTH: usize = 32;
pub const SQN_LENGTH: usize = 6;
pub const AMF_LENGTH: usize = 2;
pub const MAC_LENGTH: usize = 8;
pub const RES_STAR_LENGTH: usize = 16;
pub const RES_STAR_HASH_LENGTH: usize = 16;
pub const AUTN_LENGTH: usize = SQN_LENGTH + AMF_LENGTH + MAC_LENGTH;

pub const AMF: [u8; AMF_LENGTH] = [0x80, 0x00];