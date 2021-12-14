use crate::constants;

pub type Id = String;
pub type K = [u8; constants::K_LENGTH];
pub type Opc = [u8; constants::OPC_LENGTH];
pub type Sqn = [u8; constants::SQN_LENGTH];
pub type Mac = [u8; constants::MAC_LENGTH];
pub type ResStar = [u8; constants::RES_STAR_LENGTH];
pub type Ck = [u8; constants::CK_LENGTH];
pub type Ik = [u8; constants::IK_LENGTH];
pub type Kausf = [u8; constants::KAUSF_LENGTH];
pub type Kseaf = [u8; constants::KSEAF_LENGTH];

pub type HresStar = [u8; constants::RES_STAR_HASH_LENGTH];
pub type Rand = [u8; constants::RAND_LENGTH];
pub type Autn = [u8; constants::AUTN_LENGTH];
