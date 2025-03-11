use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
};

use crate::{instructions::sync::SyncRecordInstruction, state::record::ErRecord, ID};

pub fn process_sync_record<'a>(
    mut accounts: impl Iterator<Item = &'a AccountInfo<'a>>,
    ix: SyncRecordInstruction,
) -> Result<(), ProgramError> {
    let payer = next_account_info(&mut accounts)?;
    let pda_account = next_account_info(&mut accounts)?;

    if *pda_account.owner != ID {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if !(payer.is_signer && *payer.key == *ix.identity) {
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
    let mut record = ErRecord::deserialize(&data).map_err(|_| ProgramError::InvalidAccountData)?;

    if let Some(addr) = ix.addr {
        record.addr = addr
    }
    if let Some(fees) = ix.fees {
        record.fees = fees
    }
    if let Some(block_time_ms) = ix.block_time_ms {
        record.block_time_ms = block_time_ms
    }
    if let Some(features) = ix.features {
        record.features = features
    }
    let new_len = record.serialized_size();
    if new_len != data.len() {
        pda_account.realloc(new_len, false)?;
        data = pda_account.try_borrow_mut_data()?;
    }

    record.serialize(*data)?;

    Ok(())
}
