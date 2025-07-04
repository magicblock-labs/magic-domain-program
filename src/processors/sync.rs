use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program::invoke,
    program_error::ProgramError,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{instructions::sync::SyncInstruction, state::record::ErRecord, ID};

/// Synchronize updated ER information with existing domain registry record
pub fn process_sync_record<'a>(
    mut accounts: impl Iterator<Item = &'a AccountInfo<'a>>,
    mut ix: SyncInstruction,
) -> Result<(), ProgramError> {
    let payer = next_account_info(&mut accounts)?;
    let pda_account = next_account_info(&mut accounts)?;
    let system_program = next_account_info(&mut accounts)?;

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
    if let Some(base_fee) = ix.base_fee().take() {
        record.set_base_fee(base_fee);
    }
    if let Some(block_time_ms) = ix.block_time_ms().take() {
        record.set_block_time_ms(block_time_ms);
    }
    if let Some(features) = ix.features().take() {
        record.set_features(features);
    }
    if let Some(status) = ix.status().take() {
        record.set_status(status);
    }
    if let Some(load_average) = ix.load_average().take() {
        record.set_load_average(load_average);
    }
    let new_size = borsh::object_length(&record)?;
    let old_size = data.len();
    if old_size == new_size {
        return Ok(record.serialize(&mut *data)?);
    };
    drop(data);
    let rent_new = Rent::get()?.minimum_balance(new_size);
    let rent_old = Rent::get()?.minimum_balance(old_size);
    if rent_new > rent_old {
        invoke(
            &system_instruction::transfer(payer.key, pda_account.key, rent_new - rent_old),
            &[payer.clone(), pda_account.clone(), system_program.clone()],
        )?;
    } else {
        **pda_account.try_borrow_mut_lamports()? -= rent_old - rent_new;
        **payer.try_borrow_mut_lamports()? += rent_old - rent_new;
    }
    pda_account.realloc(new_size, false)?;
    data = pda_account.try_borrow_mut_data()?;
    record.serialize(&mut *data)?;

    Ok(())
}
