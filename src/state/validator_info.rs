use std::net::SocketAddrV4;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{consts::VALIDATOR_INFO_SEED, ID};

use super::features::FeaturesSet;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
#[cfg_attr(feature = "no-entrypoint", derive(PartialEq, Eq, Clone))]
pub struct ValidatorInfo {
    pub identity: Pubkey,
    /// NOTE: usage of variable length URLs introduces a lot of headache,
    /// so we settle on fixed length IPv4 + Port representation
    pub addr: SocketAddrV4,
    /// range of up to ~65 seconds should be plenty for all use cases
    pub block_time_ms: u16,
    /// base fee of 65536 lamports per transaction should be enough for all use cases, it's more
    /// than solana validators charge for priority transactions
    pub fees: u16,
    /// this type can represent the combination of 256 features,
    /// which should be enough for any forseeable future
    pub features: FeaturesSet,
}

impl ValidatorInfo {
    pub fn pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), &ID)
    }

    pub fn seeds(&self) -> [&[u8]; 2] {
        [VALIDATOR_INFO_SEED, self.identity.as_ref()]
    }
}
