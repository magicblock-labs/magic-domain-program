use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::{
    consts::{ix, VALIDATOR_INFO_SEED},
    serde::FieldSerializer,
    state::{
        features::FeaturesSet,
        field::{self, Field},
        record::RecordBuilder,
    },
};

pub struct SyncRecordInstruction {
    pub identity: field::Identity,
    pub block_time_ms: Option<field::BlockTimeMs>,
    pub fees: Option<field::Fees>,
    pub features: Option<FeaturesSet>,
    pub addr: Option<field::Addr>,
}

impl SyncRecordInstruction {
    pub fn pda(&self) -> Pubkey {
        let seeds = [VALIDATOR_INFO_SEED, self.identity.as_ref()];
        Pubkey::find_program_address(&seeds, &crate::ID).0
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        let builder = RecordBuilder::populate(data, true)?;
        Ok(Self {
            identity: builder
                .identity
                .ok_or(ProgramError::InvalidInstructionData)?,
            block_time_ms: builder.block_time_ms,
            features: builder.features,
            fees: builder.fees,
            addr: builder.addr,
        })
    }

    fn serialized_size(&self) -> usize {
        macro_rules! size {
            ($field: ident) => {
                self.$field
                    .as_ref()
                    .map(|f| f.size() + 1)
                    .unwrap_or_default()
            };
        }
        self.identity.size()
            + 1
            + size!(block_time_ms)
            + size!(fees)
            + size!(features)
            + size!(addr)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let len = self.serialized_size() + 1; // + 1 for ix discriminator
        let mut buffer = vec![0; len];

        buffer[0] = ix::SYNC_RECORD_IX;

        let mut serializer = FieldSerializer::new(&mut buffer[1..]);
        serializer.write_field(&self.identity);
        if let Some(ref btms) = self.block_time_ms {
            serializer.write_field(btms);
        }
        if let Some(ref fees) = self.fees {
            serializer.write_field(fees);
        }
        if let Some(ref features) = self.features {
            serializer.write_field(features);
        }
        if let Some(ref addr) = self.addr {
            serializer.write_field(addr);
        }

        buffer
    }
}
