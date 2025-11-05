use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;

#[inline(always)]
pub fn resize(target_account: &AccountInfo, new_len: usize) -> ProgramResult {
    #[cfg(not(feature = "disable-realloc"))]
    {
        #[allow(deprecated)]
        target_account.realloc(new_len, false)
    }

    #[cfg(feature = "disable-realloc")]
    {
        target_account.resize(new_len)
    }
}

#[cfg(not(feature = "modular-sdk"))]
pub mod solana {
    pub use solana_program::system_instruction as system_instruction;
}

#[cfg(feature = "modular-sdk")]
pub mod solana {
    pub use solana_system_interface::instruction as system_instruction;
}