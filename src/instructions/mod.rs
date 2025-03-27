use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use sync::SyncInstruction;

use crate::state::record::ErRecord;

pub mod sync;
pub mod version;

/// Supported program instructions
#[derive(BorshSerialize, BorshDeserialize)]
pub enum Instruction {
    Register(ErRecord),
    Unregister(Pubkey),
    Sync(SyncInstruction),
}
