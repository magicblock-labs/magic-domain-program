use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::state::features::FeaturesSet;

/// Sync instruction data, version 0
#[derive(BorshSerialize, BorshDeserialize)]
pub struct SyncRecordV0 {
    pub identity: Pubkey,
    pub addr: Option<String>,
    pub block_time_ms: Option<u16>,
    pub fees: Option<u16>,
    pub features: Option<FeaturesSet>,
}
