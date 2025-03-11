use crate::{instructions::Instruction, processors::*};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

solana_program::entrypoint!(process);

pub fn process<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    if *program_id != crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    let ix = Instruction::deserialize(data)?;
    let accounts = accounts.iter();
    match ix {
        Instruction::Register(ix) => register::process_registration(accounts, ix),
        Instruction::SyncRecord(ix) => sync::process_sync_record(accounts, ix),
        Instruction::Unregister(ix) => unregister::process_unregistration(accounts, ix),
    }
}
