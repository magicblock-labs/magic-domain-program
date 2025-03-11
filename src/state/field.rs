use derive_more::{Deref, DerefMut, From};
use solana_program::pubkey::Pubkey;

use crate::{
    consts::tags::{self, FieldTag},
    state::features::FeaturesSet,
};

#[derive(Debug, From, DerefMut, Deref)]
pub struct Identity(pub Pubkey);
#[derive(Debug, From, DerefMut, Deref)]
pub struct BlockTimeMs(pub u16);
#[derive(Debug, From, DerefMut, Deref)]
pub struct Fees(pub u16);
#[derive(Debug, From, DerefMut, Deref, Clone, PartialEq, Eq)]
pub struct Addr(pub Vec<u8>);

pub trait Field: Sized {
    const TAG: FieldTag;

    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    fn serialize(&self, slice: &mut [u8]);
    fn deserialize(slice: &[u8]) -> Option<Self>;
}

impl Field for Identity {
    const TAG: FieldTag = tags::IDENTITY_TAG;
    fn serialize(&self, slice: &mut [u8]) {
        slice[..self.size()].copy_from_slice(self.0.as_ref());
    }
    fn deserialize(slice: &[u8]) -> Option<Self> {
        const LEN: usize = std::mem::size_of::<Pubkey>();
        let array: [u8; LEN] = slice.get(..LEN)?.try_into().ok()?;
        Some(Pubkey::new_from_array(array).into())
    }
}

impl Field for BlockTimeMs {
    const TAG: FieldTag = tags::BLOCK_TIME_MS_TAG;
    fn serialize(&self, slice: &mut [u8]) {
        slice[..self.size()].copy_from_slice(&self.0.to_le_bytes());
    }
    fn deserialize(slice: &[u8]) -> Option<Self> {
        const LEN: usize = std::mem::size_of::<u16>();
        let array: [u8; LEN] = slice.get(..LEN)?.try_into().ok()?;
        Some(u16::from_le_bytes(array).into())
    }
}

impl Field for Fees {
    const TAG: FieldTag = tags::FEES_TAG;
    fn serialize(&self, slice: &mut [u8]) {
        slice[..self.size()].copy_from_slice(&self.0.to_le_bytes());
    }
    fn deserialize(slice: &[u8]) -> Option<Self> {
        const LEN: usize = std::mem::size_of::<u16>();
        let array: [u8; LEN] = slice.get(..LEN)?.try_into().ok()?;
        Some(u16::from_le_bytes(array).into())
    }
}

impl Field for FeaturesSet {
    const TAG: FieldTag = tags::FEATURES_TAG;
    fn serialize(&self, slice: &mut [u8]) {
        slice[..self.size()].copy_from_slice(self.0.as_ref());
    }
    fn deserialize(slice: &[u8]) -> Option<Self> {
        const LEN: usize = std::mem::size_of::<FeaturesSet>();
        let array: [u8; LEN] = slice.get(..LEN)?.try_into().ok()?;
        Self(array).into()
    }
}

impl Field for Addr {
    const TAG: FieldTag = tags::ADDR_TAG;
    fn size(&self) -> usize {
        std::mem::size_of::<u16>() + self.0.len()
    }
    fn serialize(&self, slice: &mut [u8]) {
        slice[..2].copy_from_slice(&(self.0.len() as u16).to_le_bytes());
        slice[2..2 + self.0.len()].copy_from_slice(self.0.as_ref());
    }
    fn deserialize(slice: &[u8]) -> Option<Self> {
        const PREFIX: usize = std::mem::size_of::<u16>();
        let array: [u8; PREFIX] = slice.get(..PREFIX)?.try_into().ok()?;
        let len = u16::from_le_bytes(array) as usize;
        let vector = slice.get(PREFIX..PREFIX + len)?.to_vec();
        Self(vector).into()
    }
}
