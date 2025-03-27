use super::{features::FeaturesSet, version::v0::RecordV0};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{consts::ER_RECORD_SEED, ID};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
#[cfg_attr(not(feature = "entrypoint"), derive(PartialEq, Eq, Clone))]
pub enum ErRecord {
    V0(RecordV0),
}

impl ErRecord {
    /// Computes record's PDA for given ER node
    pub fn pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), &ID)
    }

    /// Returns an array of seeds for record PDA derivation
    pub fn seeds(&self) -> [&[u8]; 2] {
        [ER_RECORD_SEED, self.identity().as_ref()]
    }

    /// Returns identity pubkey of given ER
    pub fn identity(&self) -> &Pubkey {
        match self {
            Self::V0(r) => &r.identity,
        }
    }

    /// Returns FQDN address for given ER node
    pub fn addr(&self) -> &str {
        match self {
            Self::V0(v) => &v.addr,
        }
    }

    /// Returns transaction fees of given ER node
    pub fn fees(&self) -> u16 {
        match self {
            Self::V0(v) => v.fees,
        }
    }

    /// Returns supported set of features by given ER node
    pub fn features(&self) -> FeaturesSet {
        match self {
            Self::V0(v) => v.features.clone(),
        }
    }

    /// Returns block time in ms of given ER node
    pub fn block_time_ms(&self) -> u16 {
        match self {
            Self::V0(v) => v.block_time_ms,
        }
    }

    /// Updates FQDN address in given ER record
    pub fn set_addr(&mut self, addr: String) {
        match self {
            Self::V0(v) => v.addr = addr,
        }
    }

    /// Updates fees in given ER record
    pub fn set_fees(&mut self, fees: u16) {
        match self {
            Self::V0(v) => v.fees = fees,
        }
    }

    /// Updates features set in given ER record
    pub fn set_features(&mut self, features: FeaturesSet) {
        match self {
            Self::V0(v) => v.features = features,
        }
    }

    /// Updates block time in ms in given ER record
    pub fn set_block_time_ms(&mut self, block_time_ms: u16) {
        match self {
            Self::V0(v) => v.block_time_ms = block_time_ms,
        }
    }
}
