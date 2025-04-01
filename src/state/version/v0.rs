use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::state::{features::FeaturesSet, status::ErStatus};

/// Version 0 of ER domain registry record
#[derive(Debug, BorshSerialize, BorshDeserialize)]
#[cfg_attr(not(feature = "entrypoint"), derive(PartialEq, Eq, Clone))]
pub struct RecordV0 {
    /// Identity of ER node (pubkey from its keypair)
    pub identity: Pubkey,
    /// Current status of ER node
    pub status: ErStatus,
    /// Block time of given ER node in ms
    pub block_time_ms: u16,
    /// Base fee charged by ER node per transaction
    pub base_fee: u16,
    /// A bitmap of all possible combination of custom features that the ER node supports
    pub features: FeaturesSet,
    /// An average value, which is acts as an indicator
    /// of how loaded the given ER node currently is
    pub load_average: u32,
    /// Variable length string representing FQDN
    pub addr: String,
}
