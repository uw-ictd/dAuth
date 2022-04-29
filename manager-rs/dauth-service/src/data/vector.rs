use auth_vector::types::{Autn, HresStar, Id, Rand};

use crate::data::error::DauthError;
use crate::rpc::dauth::common::AuthVector5G;
use crate::rpc::dauth::local::{AkaVectorReq, AkaVectorResp};

#[derive(Debug)]
pub struct AuthVectorReq {
    pub user_id: Id,
}

#[derive(Debug)]
pub struct AuthVectorRes {
    pub user_id: Id,
    pub seqnum: i64,
    pub xres_star_hash: HresStar,
    pub autn: Autn,
    pub rand: Rand,
}

impl AuthVectorReq {
    pub fn from_req(req: AkaVectorReq) -> Result<AuthVectorReq, DauthError> {
        tracing::info!("Converting id");
        match req.user_id_type() {
            crate::rpc::dauth::common::UserIdKind::Supi => {
                let id_string = std::str::from_utf8(req.user_id.as_slice())?;
                tracing::debug!("Converted id {:?}", id_string);
                Ok(AuthVectorReq {
                    user_id: id_string.to_string(),
                })
            }
            crate::rpc::dauth::common::UserIdKind::Unknown => Err(DauthError::InvalidMessageError(
                "user_id_kind is unknown".to_owned(),
            )),
        }
    }
}

impl AuthVectorRes {
    pub fn to_resp(&self) -> AkaVectorResp {
        AkaVectorResp {
            user_id: self.user_id.clone().into_bytes(),
            user_id_type: 1,
            error: 0,
            auth_vector: Some(AuthVector5G {
                rand: self.rand.to_vec(),
                xres_star_hash: self.xres_star_hash.to_vec(),
                autn: self.autn.to_vec(),
                seqnum: self.seqnum,
            }),
        }
    }

    pub fn from_av5_g(user_id: &str, vector: AuthVector5G) -> Result<AuthVectorRes, DauthError> {
        Ok(AuthVectorRes {
            user_id: user_id.to_string(),
            seqnum: vector.seqnum,
            xres_star_hash: vector.xres_star_hash[..].try_into()?,
            autn: vector.autn[..].try_into()?,
            rand: vector.rand[..].try_into()?,
        })
    }
}

use shamir::SecretData;

const SHARE_LENGTH: usize = 17;
#[derive(Debug, PartialEq)]
pub struct AutnShare {
    pub share: [u8; SHARE_LENGTH],
}

impl TryFrom<Vec<u8>> for AutnShare {
    type Error = DauthError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != SHARE_LENGTH {
            return Err(DauthError::ShamirShareError());
        }

        let res: Result<[u8; SHARE_LENGTH], _> = value.try_into();

        res.and_then(|share_bytes| Ok(AutnShare { share: share_bytes }))
            .or(Err(DauthError::ShamirShareError()))
    }
}

pub fn create_shares_from_autn<T: rand_0_8::RngCore>(
    input: Autn,
    share_count: u8,
    rng: &mut T,
) -> Result<Vec<AutnShare>, DauthError> {
    let secret = SecretData::with_secret_bytes(&input, 3, rng);

    let mut shares: Vec<AutnShare> = Vec::new();
    for i in 1u8..share_count + 1 {
        println!("Getting share {}", i);
        let share = secret.get_share(i).or_else(|e| {
            println!("Got error {:?}", e);
            Err(DauthError::ShamirShareError())
        })?;
        // shares.push(AutnShare::try_from(share)?);
        println!("Got share {}", i);
        shares.push(share.try_into()?);
    }
    println!("Returning shares");
    Ok(shares)
}

pub fn recover_autn_from_shares(input: &Vec<AutnShare>) -> Result<Autn, DauthError> {
    let byte_inputs: Vec<Vec<u8>> = input.iter().map(|x| x.share.to_vec()).collect();
    println!("Generated byte inputs {:?}", byte_inputs);

    let res = SecretData::recover_secret_bytes(3, byte_inputs);
    println!("Got res:{:?}", res);

    match res {
        Some(value) => Ok(value.try_into().unwrap()),
        None => Err(DauthError::ShamirShareError()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::hex;
    use auth_vector::types::Autn;

    #[test]
    fn test_vector_split() {
        let mut rng = rand_0_8::thread_rng();
        let autn: Autn = hex::decode("562d716dbd058b475cfecdbb48ed038f")
            .unwrap()
            .try_into()
            .unwrap();
        let res = create_shares_from_autn(autn, 5, &mut rng).unwrap();
        println!("{:?}", res);

        let res_autn: Autn = recover_autn_from_shares(&res).unwrap();
        assert_eq!(autn, res_autn);
    }

    #[test]
    #[should_panic(expected = "ShamirShareError")]
    fn test_invalid_vector() {
        let mut rng = rand_0_8::thread_rng();
        let autn: Autn = hex::decode("562d716dbd058b475cfecdbb48ed038f")
            .unwrap()
            .try_into()
            .unwrap();
        let mut res = create_shares_from_autn(autn, 5, &mut rng).unwrap();
        println!("{:?}", res);

        // Corrupt one of the shares, technically possible for the test to fail
        // since the rand is non-deterministic, but should be cryptographically
        // rare.
        res[1] = AutnShare {
            share: [
                2, 102, 144, 138, 212, 69, 222, 118, 11, 242, 176, 156, 89, 234, 255, 82, 79,
            ],
        };

        assert_ne!(autn, recover_autn_from_shares(&res).unwrap());

        // Corrupt a share index, should always cause an error.
        res[2] = AutnShare {
            share: [
                2, 102, 144, 138, 212, 69, 222, 118, 11, 242, 176, 156, 89, 234, 255, 82, 79,
            ],
        };
        recover_autn_from_shares(&res).unwrap();
    }

    #[test]
    fn test_vector_creation_deterministic() {
        let mut rng = rand_0_8::rngs::mock::StepRng::new(1, 1);
        let autn: Autn = hex::decode("562d716dbd058b475cfecdbb48ed038f")
            .unwrap()
            .try_into()
            .unwrap();
        let res = create_shares_from_autn(autn, 5, &mut rng).unwrap();
        println!("{:?}", res);

        let expected_res = vec![
            AutnShare {
                share: [
                    1, 87, 47, 114, 105, 184, 3, 140, 79, 85, 244, 198, 183, 69, 227, 12, 159,
                ],
            },
            AutnShare {
                share: [
                    2, 84, 41, 119, 101, 183, 9, 133, 87, 78, 234, 219, 163, 82, 241, 29, 175,
                ],
            },
            AutnShare {
                share: [
                    3, 85, 43, 116, 97, 178, 15, 130, 95, 71, 224, 208, 175, 95, 255, 18, 191,
                ],
            },
            AutnShare {
                share: [
                    4, 82, 37, 125, 125, 169, 29, 151, 103, 120, 214, 225, 139, 124, 213, 63, 207,
                ],
            },
            AutnShare {
                share: [
                    5, 83, 39, 126, 121, 172, 27, 144, 111, 113, 220, 234, 135, 113, 219, 48, 223,
                ],
            },
        ];

        for (i, share) in res.iter().enumerate() {
            assert_eq!(share, &expected_res[i]);
        }
    }
}
