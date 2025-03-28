use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
};

use crate::{instructions::sync::SyncInstruction, state::record::ErRecord, ID};

/// Synchronize updated ER information with existing domain registry record
pub fn process_sync_record<'a>(
    mut accounts: impl Iterator<Item = &'a AccountInfo<'a>>,
    mut ix: SyncInstruction,
) -> Result<(), ProgramError> {
    let payer = next_account_info(&mut accounts)?;
    let pda_account = next_account_info(&mut accounts)?;

    if *pda_account.owner != ID {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if !(payer.is_signer && payer.key == ix.identity()) {
        return Err(ProgramError::InvalidArgument);
    }

    if pda_account.lamports() == 0 {
        return Err(ProgramError::UninitializedAccount);
    }

    let pda = ix.pda();

    if pda != *pda_account.key {
        return Err(ProgramError::InvalidArgument);
    }

    let mut data = pda_account.try_borrow_mut_data()?;
    let mut record =
        ErRecord::try_from_slice(&data).map_err(|_| ProgramError::InvalidAccountData)?;

    if let Some(addr) = ix.addr().take() {
        record.set_addr(addr);
    }
    if let Some(fees) = ix.fees().take() {
        record.set_fees(fees);
    }
    if let Some(block_time_ms) = ix.block_time_ms().take() {
        record.set_block_time_ms(block_time_ms);
    }
    if let Some(features) = ix.features().take() {
        record.set_features(features);
    }

    record.serialize(&mut *data)?;

    Ok(())
}
