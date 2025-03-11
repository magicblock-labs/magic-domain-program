use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::consts::ix;

pub struct UnregisterInstruction(pub Pubkey);

impl UnregisterInstruction {
    pub fn deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        const LEN: usize = std::mem::size_of::<Pubkey>();
        let array: [u8; LEN] = data
            .get(..LEN)
            .ok_or(ProgramError::InvalidInstructionData)?
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        Ok(Self(Pubkey::new_from_array(array)))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![ix::UNREGISTER_IX];
        buffer.extend_from_slice(self.0.as_ref());
        buffer
    }
}
