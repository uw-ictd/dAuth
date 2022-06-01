use tracing::instrument;

use auth_vector::constants::KSEAF_LENGTH;
use auth_vector::types::Kseaf;

use crate::data::error::DauthError;

pub const TEMPORARY_CONSTANT_THRESHOLD: u8 = 3;
const SHARE_LENGTH: usize = KSEAF_LENGTH + 1;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct KseafShare {
    pub share: [u8; SHARE_LENGTH],
}

impl KseafShare {
    pub fn as_slice<'a>(&'a self) -> &'a [u8] {
        self.share.as_slice()
    }
}

impl TryFrom<Vec<u8>> for KseafShare {
    type Error = DauthError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != SHARE_LENGTH {
            return Err(DauthError::ShamirShareError());
        }

        let res: Result<[u8; SHARE_LENGTH], _> = value.try_into();

        res.and_then(|share_bytes| Ok(KseafShare { share: share_bytes }))
            .or(Err(DauthError::ShamirShareError()))
    }
}

impl TryFrom<&[u8]> for KseafShare {
    type Error = DauthError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != SHARE_LENGTH {
            return Err(DauthError::ShamirShareError());
        }

        let res: Result<[u8; SHARE_LENGTH], _> = value.try_into();

        res.and_then(|share_bytes| Ok(KseafShare { share: share_bytes }))
            .or(Err(DauthError::ShamirShareError()))
    }
}

#[instrument(level = "info")]
pub fn create_shares_from_kseaf<T: rand_0_8::RngCore + std::fmt::Debug>(
    input: &Kseaf,
    share_count: u8,
    threshold_share_count: u8,
    rng: &mut T,
) -> Result<Vec<KseafShare>, DauthError> {
    tracing::info!("Thingin the do");
    if threshold_share_count > share_count {
        tracing::error!(
            share_count,
            threshold_share_count,
            "Invalid share request with threshold greater than total share count."
        );
        return Err(DauthError::ShamirShareError());
    }
    let secret = shamir::SecretData::with_secret_bytes(input, threshold_share_count, rng);

    let mut shares: Vec<KseafShare> = Vec::new();
    for i in 1u8..share_count + 1 {
        let share = secret.get_share(i).or_else(|e| {
            tracing::error!(share_index = i, ?e, "Failed to get share");
            Err(DauthError::ShamirShareError())
        })?;
        shares.push(share.try_into()?);
    }

    Ok(shares)
}

#[instrument(level = "debug")]
pub fn recover_kseaf_from_shares(
    input: &Vec<KseafShare>,
    threshold: u8,
) -> Result<Kseaf, DauthError> {
    let byte_inputs: Vec<Vec<u8>> = input.iter().map(|x| x.share.to_vec()).collect();

    let res = shamir::SecretData::recover_secret_bytes(threshold, byte_inputs);

    match res {
        Some(value) => Ok(value.try_into().unwrap()),
        None => Err(DauthError::ShamirShareError()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::hex;
    use auth_vector::types::Kseaf;
    use test_log::test;

    #[test]
    fn test_vector_split() {
        let mut rng = rand_0_8::thread_rng();
        let kseaf: Kseaf =
            hex::decode("562d716dbd058b475cfecdbb48ed038f562d716dbd058b475cfecdbb48ed038f")
                .unwrap()
                .try_into()
                .unwrap();
        let res = create_shares_from_kseaf(&kseaf, 5, 3, &mut rng).unwrap();

        let res_kseaf: Kseaf = recover_kseaf_from_shares(&res, 3).unwrap();
        assert_eq!(kseaf, res_kseaf);
    }

    #[test]
    fn test_vector_recovery() {
        let mut rng = rand_0_8::thread_rng();
        let kseaf: Kseaf =
            hex::decode("562d716dbd058b475cfecdbb48ed038f562d716dbd058b475cfecdbb48ed038f")
                .unwrap()
                .try_into()
                .unwrap();
        let mut res = create_shares_from_kseaf(&kseaf, 5, 3, &mut rng).unwrap();

        // Remove two arbitrary shares of the 5.
        res.remove(2);
        res.remove(1);

        let res_kseaf: Kseaf = recover_kseaf_from_shares(&res, 3).unwrap();
        assert_eq!(kseaf, res_kseaf);
    }

    #[test]
    #[should_panic(expected = "ShamirShareError")]
    fn test_invalid_vector() {
        let mut rng = rand_0_8::thread_rng();
        let kseaf: Kseaf =
            hex::decode("562d716dbd058b475cfecdbb48ed038f562d716dbd058b475cfecdbb48ed038f")
                .unwrap()
                .try_into()
                .unwrap();
        let mut res = create_shares_from_kseaf(&kseaf, 5, 3, &mut rng).unwrap();
        println!("{:?}", res);

        // Corrupt one of the shares, technically possible for the test to fail
        // since the rand is non-deterministic, but should be cryptographically
        // rare.
        res[1] = KseafShare {
            share: [
                2, 102, 144, 138, 212, 69, 222, 118, 11, 242, 176, 156, 89, 234, 255, 82, 79, 102,
                144, 138, 212, 69, 222, 118, 11, 242, 176, 156, 89, 234, 255, 82, 79,
            ],
        };

        assert_ne!(kseaf, recover_kseaf_from_shares(&res, 3).unwrap());

        // Corrupt a share index, should always cause an error.
        res[2] = KseafShare {
            share: [
                2, 102, 144, 138, 212, 69, 222, 118, 11, 242, 176, 156, 89, 234, 255, 82, 79, 102,
                144, 138, 212, 69, 222, 118, 11, 242, 176, 156, 89, 234, 255, 82, 79,
            ],
        };
        recover_kseaf_from_shares(&res, 3).unwrap();
    }

    #[test]
    fn test_vector_creation_deterministic() {
        let mut rng = rand_0_8::rngs::mock::StepRng::new(1, 1);
        let kseaf: Kseaf =
            hex::decode("562d716dbd058b475cfecdbb48ed038f562d716dbd058b475cfecdbb48ed038f")
                .unwrap()
                .try_into()
                .unwrap();
        let res = create_shares_from_kseaf(&kseaf, 5, 3, &mut rng).unwrap();
        println!("{:?}", res);

        let expected_res = vec![
            KseafShare {
                share: [
                    1, 87, 47, 114, 105, 184, 3, 140, 79, 85, 244, 198, 183, 69, 227, 12, 159, 71,
                    63, 98, 121, 168, 19, 156, 95, 69, 228, 214, 167, 85, 243, 28, 175,
                ],
            },
            KseafShare {
                share: [
                    2, 84, 41, 119, 101, 183, 9, 133, 87, 78, 234, 219, 163, 82, 241, 29, 175, 116,
                    9, 87, 69, 151, 41, 165, 119, 110, 202, 251, 131, 114, 209, 61, 207,
                ],
            },
            KseafShare {
                share: [
                    3, 85, 43, 116, 97, 178, 15, 130, 95, 71, 224, 208, 175, 95, 255, 18, 191, 101,
                    27, 68, 81, 130, 63, 178, 111, 119, 208, 224, 159, 111, 207, 34, 239,
                ],
            },
            KseafShare {
                share: [
                    4, 82, 37, 125, 125, 169, 29, 151, 103, 120, 214, 225, 139, 124, 213, 63, 207,
                    18, 101, 61, 61, 233, 93, 215, 39, 56, 150, 161, 203, 60, 149, 127, 15,
                ],
            },
            KseafShare {
                share: [
                    5, 83, 39, 126, 121, 172, 27, 144, 111, 113, 220, 234, 135, 113, 219, 48, 223,
                    3, 119, 46, 41, 252, 75, 192, 63, 33, 140, 186, 215, 33, 139, 96, 47,
                ],
            },
        ];

        for (i, share) in res.iter().enumerate() {
            assert_eq!(share, &expected_res[i]);
        }
    }
}
