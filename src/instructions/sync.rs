use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{
    consts::ER_RECORD_SEED,
    state::{features::FeaturesSet, status::ErStatus},
    ID,
};

use super::version::v0::SyncRecordV0;

/// Versioned sync program instruction
#[derive(BorshSerialize, BorshDeserialize)]
pub enum SyncInstruction {
    V0(SyncRecordV0),
}

impl SyncInstruction {
    /// Compute the record PDA for given ER node identity
    pub fn pda(&self) -> Pubkey {
        let seeds = [ER_RECORD_SEED, self.identity().as_ref()];
        Pubkey::find_program_address(&seeds, &ID).0
    }

    /// Returns ER node identity
    pub fn identity(&self) -> &Pubkey {
        match self {
            Self::V0(r) => &r.identity,
        }
    }

    /// Returns ER node address, if set
    pub fn addr(&mut self) -> &mut Option<String> {
        match self {
            Self::V0(v) => &mut v.addr,
        }
    }

    /// Returns ER node fees, if set
    pub fn base_fee(&mut self) -> &mut Option<u16> {
        match self {
            Self::V0(v) => &mut v.base_fee,
        }
    }

    /// Returns ER node block time in ms, if set
    pub fn block_time_ms(&mut self) -> &mut Option<u16> {
        match self {
            Self::V0(v) => &mut v.block_time_ms,
        }
    }

    /// Returns ER node supported features set, if set
    pub fn features(&mut self) -> &mut Option<FeaturesSet> {
        match self {
            Self::V0(v) => &mut v.features,
        }
    }

    pub fn status(&mut self) -> &mut Option<ErStatus> {
        match self {
            Self::V0(v) => &mut v.status,
        }
    }

    pub fn load_average(&mut self) -> &mut Option<u32> {
        match self {
            Self::V0(v) => &mut v.load_average,
        }
    }
}
