use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::state::{features::FeaturesSet, record::CountryCode, status::ErStatus};

/// Sync instruction data, version 0
#[derive(BorshSerialize, BorshDeserialize)]
pub struct SyncRecordV0 {
    pub identity: Pubkey,
    pub status: Option<ErStatus>,
    pub block_time_ms: Option<u16>,
    pub base_fee: Option<u16>,
    pub features: Option<FeaturesSet>,
    pub load_average: Option<u32>,
    pub country_code: Option<CountryCode>,
    pub addr: Option<String>,
}
