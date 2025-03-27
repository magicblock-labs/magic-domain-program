use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::state::features::FeaturesSet;

/// Version 0 of ER domain registry record
#[derive(Debug, BorshSerialize, BorshDeserialize)]
#[cfg_attr(not(feature = "entrypoint"), derive(PartialEq, Eq, Clone))]
pub struct RecordV0 {
    /// Identity of ER node (pubkey from its keypair)
    pub identity: Pubkey,
    /// variable length string
    pub addr: String,
    /// range of up to ~65 seconds should be plenty for all use cases
    pub block_time_ms: u16,
    /// base fee of 65536 lamports per transaction should be enough for all use cases, it's more
    /// than solana validators charge for priority transactions
    pub fees: u16,
    /// this type can represent the combination of 256 features,
    /// which should be enough for any forseeable future
    pub features: FeaturesSet,
}
