use solana_program::program_error::ProgramError;

use crate::{consts::ix, state::record::ErRecord};

pub struct RegisterInstruction(pub ErRecord);

impl RegisterInstruction {
    pub fn deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        ErRecord::deserialize(data).map(Self)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let len = self.0.serialized_size() + 1; // + 1 for ix discriminator
        let mut buffer = vec![0; len];
        buffer[0] = ix::REGISTER_IX;
        self.0
            .serialize(&mut buffer[1..])
            .expect("should always serialize as we have allocated the exact size");
        buffer
    }
}
