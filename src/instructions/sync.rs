use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{
    consts::ER_RECORD_SEED,
    state::{features::FeaturesSet, record::CountryCode, status::ErStatus},
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

    /// Returns identity pubkey of the ER node
    pub fn identity(&self) -> &Pubkey {
        match self {
            Self::V0(r) => &r.identity,
        }
    }

    /// Returns address of the ER node, if set
    pub fn addr(&mut self) -> &mut Option<String> {
        match self {
            Self::V0(v) => &mut v.addr,
        }
    }

    /// Returns base transaction fee charged by ER node, if set
    pub fn base_fee(&mut self) -> &mut Option<u16> {
        match self {
            Self::V0(v) => &mut v.base_fee,
        }
    }

    /// Returns the block time in ms of the given ER node, if set
    pub fn block_time_ms(&mut self) -> &mut Option<u16> {
        match self {
            Self::V0(v) => &mut v.block_time_ms,
        }
    }

    /// Returns the features set supported by ER node, if set
    pub fn features(&mut self) -> &mut Option<FeaturesSet> {
        match self {
            Self::V0(v) => &mut v.features,
        }
    }

    /// Returns the status of ER node, if set
    pub fn status(&mut self) -> &mut Option<ErStatus> {
        match self {
            Self::V0(v) => &mut v.status,
        }
    }

    /// Returns last observed average load on the given ER node, if set
    pub fn load_average(&mut self) -> &mut Option<u32> {
        match self {
            Self::V0(v) => &mut v.load_average,
        }
    }

    pub fn country_code(&mut self) -> &mut Option<CountryCode> {
        match self {
            Self::V0(v) => &mut v.country_code,
        }
    }
}
