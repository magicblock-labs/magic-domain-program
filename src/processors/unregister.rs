use borsh::BorshDeserialize;
use solana_program::pubkey::Pubkey;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
};

use crate::state::record::ErRecord;
use crate::ID;

/// Unregisters given ER node, by removing its record from domain registry
pub fn process_unregistration<'a>(
    mut accounts: impl Iterator<Item = &'a AccountInfo<'a>>,
    node_id: Pubkey,
) -> Result<(), ProgramError> {
    let payer = next_account_info(&mut accounts)?;
    let pda_account = next_account_info(&mut accounts)?;
    let system_program = next_account_info(&mut accounts)?;

    if *pda_account.owner != ID {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if !(payer.is_signer && *payer.key == node_id) {
        return Err(ProgramError::InvalidArgument);
    }

    if pda_account.lamports() == 0 {
        return Err(ProgramError::UninitializedAccount);
    }
    let data = pda_account.try_borrow_data()?;
    let record = ErRecord::try_from_slice(&data).map_err(|_| ProgramError::InvalidAccountData)?;
    drop(data);

    if node_id != *record.identity() {
        return Err(ProgramError::InvalidArgument);
    }

    let (pda, _) = record.pda();

    if pda != *pda_account.key {
        return Err(ProgramError::InvalidArgument);
    }

    let payer_balance = payer.lamports();
    let pda_balance = pda_account.lamports();
    **payer.try_borrow_mut_lamports()? = payer_balance
        .checked_add(pda_balance)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    **pda_account.try_borrow_mut_lamports()? = 0;

    pda_account.assign(system_program.key);
    pda_account.realloc(0, false)?;

    Ok(())
}
