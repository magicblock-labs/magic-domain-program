use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{consts::ER_RECORD_SEED, state::features::FeaturesSet, ID};

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
    pub fn addr(&mut self) -> Option<String> {
        match self {
            Self::V0(v) => v.addr.take(),
        }
    }

    /// Returns ER node fees, if set
    pub fn fees(&mut self) -> Option<u16> {
        match self {
            Self::V0(v) => v.fees.take(),
        }
    }

    /// Returns ER node block time in ms, if set
    pub fn block_time_ms(&mut self) -> Option<u16> {
        match self {
            Self::V0(v) => v.block_time_ms.take(),
        }
    }

    /// Returns ER node supported features set, if set
    pub fn features(&mut self) -> Option<FeaturesSet> {
        match self {
            Self::V0(v) => v.features.take(),
        }
    }
}
