use derive_more::From;
use solana_program::program_error::ProgramError;

use register::RegisterInstruction;
use sync::SyncRecordInstruction;
use unregister::UnregisterInstruction;

use crate::consts::ix;

pub mod register;
pub mod sync;
pub mod unregister;

#[derive(From)]
pub enum Instruction {
    Register(RegisterInstruction),
    Unregister(UnregisterInstruction),
    SyncRecord(SyncRecordInstruction),
}

impl Instruction {
    pub fn deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        (!data.is_empty())
            .then_some(())
            .ok_or(ProgramError::InvalidInstructionData)?;

        match data[0] {
            ix::REGISTER_IX => RegisterInstruction::deserialize(&data[1..]).map(Self::from),
            ix::SYNC_RECORD_IX => SyncRecordInstruction::deserialize(&data[1..]).map(Self::from),
            ix::UNREGISTER_IX => UnregisterInstruction::deserialize(&data[1..]).map(Self::from),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Self::Register(ix) => ix.serialize(),
            Self::SyncRecord(ix) => ix.serialize(),
            Self::Unregister(ix) => ix.serialize(),
        }
    }
}
