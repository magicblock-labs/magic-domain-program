use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program::invoke_signed,
    program_error::ProgramError,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

use crate::{instructions::register::RegisterInstruction, ID};

pub fn process_registration<'a>(
    mut accounts: impl Iterator<Item = &'a AccountInfo<'a>>,
    ix: RegisterInstruction,
) -> Result<(), ProgramError> {
    let payer = next_account_info(&mut accounts)?;
    let pda_account = next_account_info(&mut accounts)?;
    let system_program = next_account_info(&mut accounts)?;

    if !(payer.is_signer && *payer.key == *ix.0.identity) {
        return Err(ProgramError::InvalidArgument);
    }

    if pda_account.lamports() != 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let (pda, bump) = ix.0.pda();

    if pda != *pda_account.key {
        return Err(ProgramError::InvalidArgument);
    }

    let mut data = vec![0; ix.0.serialized_size()];
    ix.0.serialize(&mut data)?;

    let space = data.len();
    let rent = Rent::get()?.minimum_balance(data.len());
    let [s1, s2] = ix.0.seeds();

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
