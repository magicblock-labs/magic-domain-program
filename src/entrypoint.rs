use crate::{instructions::Instruction, processors::*};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

solana_program::entrypoint!(process);

/// Main program entrypoint for processing supported instructions
pub fn process<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    if *program_id != crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    let ix = Instruction::try_from_slice(data)?;
    let accounts = accounts.iter();
    match ix {
        Instruction::Register(record) => register::process_registration(accounts, record),
        Instruction::Sync(ix) => sync::process_sync_record(accounts, ix),
        Instruction::Unregister(node_id) => unregister::process_unregistration(accounts, node_id),
    }
}
