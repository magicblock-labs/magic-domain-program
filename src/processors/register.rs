use borsh::BorshSerialize;
use solana_program::msg;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program::invoke_signed,
    program_error::ProgramError,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

use crate::{state::record::ErRecord, ID};

/// Registers ER node in domain registry, by creating a record (PDA) with all the relevant ER information
pub fn process_registration<'a>(
    mut accounts: impl Iterator<Item = &'a AccountInfo<'a>>,
    record: ErRecord,
) -> Result<(), ProgramError> {
    let payer = next_account_info(&mut accounts)?;
    let pda_account = next_account_info(&mut accounts)?;
    let system_program = next_account_info(&mut accounts)?;

    if !payer.is_signer {
        msg!("transaction payer should be signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if payer.key != record.identity() {
        msg!("transaction payer should be the same as ER node identity");
        return Err(ProgramError::InvalidArgument);
    }

    if pda_account.lamports() != 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let (pda, bump) = record.pda();

    if pda != *pda_account.key {
        msg!(
            "pubkey for registry record pda doesn't match provided one {} != {}",
            pda,
            pda_account.key
        );
        return Err(ProgramError::InvalidArgument);
    }

    let mut data = Vec::new();
    record.serialize(&mut data)?;

    let space = data.len();
    let rent = Rent::get()?.minimum_balance(data.len());
    let [s1, s2] = record.seeds();

    let create_pda_ix = create_account(payer.key, &pda, rent, space as u64, &ID);
    invoke_signed(
        &create_pda_ix,
        &[payer.clone(), pda_account.clone(), system_program.clone()],
        &[&[s1, s2, &[bump]]],
    )?;

    let mut borrowed = pda_account.try_borrow_mut_data()?;
    borrowed.copy_from_slice(&data);

    Ok(())
}
