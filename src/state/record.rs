use solana_program::pubkey::Pubkey;
use solana_program::{log, program_error::ProgramError};

use crate::{
    consts::{tags, VALIDATOR_INFO_SEED},
    serde::{FieldDeserializer, FieldSerializer},
    ID,
};

use super::{
    features::FeaturesSet,
    field::{self, Field},
};

/// Registry entry for ER validator node. Each new field addition should
/// bump the RegistryRecord version, and update the serde logic
#[derive(Debug)]
#[repr(C)]
#[cfg_attr(not(feature = "entrypoint"), derive(PartialEq, Eq, Clone))]
pub struct ErRecord {
    pub identity: field::Identity,
    /// range of up to ~65 seconds should be plenty for all use cases
    pub block_time_ms: field::BlockTimeMs,
    /// base fee of 65536 lamports per transaction should be enough for all use cases, it's more
    /// than solana validators charge for priority transactions
    pub fees: field::Fees,
    /// this type can represent the combination of 256 features,
    /// which should be enough for any forseeable future
    pub features: FeaturesSet,
    /// FQDN, which can be used to reach the node from the internet
    pub addr: field::Addr,
}

impl ErRecord {
    pub fn pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), &ID)
    }

    pub fn seeds(&self) -> [&[u8]; 2] {
        [VALIDATOR_INFO_SEED, self.identity.as_ref()]
    }

    /// Total size of serialized ErRecord
    pub fn serialized_size(&self) -> usize {
        // NOTE: every time a new field is added, this formula should be updated
        // + 1 is for tag per each field
        self.identity.size()
            + 1
            + self.block_time_ms.size()
            + 1
            + self.fees.size()
            + 1
            + self.features.size()
            + 1
            + self.addr.size()
            + 1
    }

    pub fn serialize(&self, buffer: &mut [u8]) -> Result<(), ProgramError> {
        let mut serializer = FieldSerializer::new(buffer);
        serializer.write_field(&self.identity);
        serializer.write_field(&self.block_time_ms);
        serializer.write_field(&self.fees);
        serializer.write_field(&self.features);
        serializer.write_field(&self.addr);
        Ok(())
    }

    pub fn deserialize(buffer: &[u8]) -> Result<Self, ProgramError> {
        let builder = RecordBuilder::populate(buffer, true)?;

        macro_rules! extract {
            ($field: ident) => {
                builder
                    .$field
                    .ok_or(ProgramError::InvalidAccountData)
                    .inspect_err(|_| {
                        log::msg!("failed to deserialize {} field", stringify!($field))
                    })?
            };
        }

        Ok(Self {
            identity: extract!(identity),
            block_time_ms: extract!(block_time_ms),
            fees: extract!(fees),
            features: extract!(features),
            addr: extract!(addr),
        })
    }
}

#[derive(Default)]
pub struct RecordBuilder {
    pub identity: Option<field::Identity>,
    pub block_time_ms: Option<field::BlockTimeMs>,
    pub fees: Option<field::Fees>,
    pub features: Option<FeaturesSet>,
    pub addr: Option<field::Addr>,
}

impl RecordBuilder {
    /// Creates a new record builder, populating all the known fields with deserialized data
    /// _args_:
    /// *buffer* - serialized representation of ErRecord
    /// *error* - boolean flag indicating whether to error on deserialization failure, this should be
    ///         always set to true within program code, but clients most likely will prefer to keep it set
    ///         to false, so that any new fields, that client is not aware of, can be safely ignored.
    pub fn populate(buffer: &[u8], error: bool) -> Result<Self, ProgramError> {
        let mut builder = Self::default();
        let mut deserializer = FieldDeserializer::new(buffer);
        while let Some(tag) = deserializer.read_tag() {
            match tag {
                tags::IDENTITY_TAG => {
                    builder.identity = deserializer.read_field();
                }
                tags::BLOCK_TIME_MS_TAG => {
                    builder.block_time_ms = deserializer.read_field();
                }
                tags::FEES_TAG => {
                    builder.fees = deserializer.read_field();
                }
                tags::FEATURES_TAG => {
                    builder.features = deserializer.read_field();
                }
                tags::ADDR_TAG => {
                    builder.addr = deserializer.read_field();
                }
                _ if error => return Err(ProgramError::InvalidAccountData),
                _ => break,
            }
        }
        Ok(builder)
    }
}
