use super::{features::FeaturesSet, status::ErStatus, version::v0::RecordV0};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{consts::ER_RECORD_SEED, ID};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
#[cfg_attr(not(feature = "entrypoint"), derive(PartialEq, Eq, Clone))]
pub enum ErRecord {
    V0(RecordV0),
}

impl ErRecord {
    /// Computes record's PDA for the given ER node
    pub fn pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&self.seeds(), &ID)
    }

    /// Returns an array of seeds for the record's PDA derivation
    pub fn seeds(&self) -> [&[u8]; 2] {
        [ER_RECORD_SEED, self.identity().as_ref()]
    }

    /// Returns identity pubkey of the given ER
    pub fn identity(&self) -> &Pubkey {
        match self {
            Self::V0(r) => &r.identity,
        }
    }

    /// Returns FQDN address for the given ER node
    pub fn addr(&self) -> &str {
        match self {
            Self::V0(v) => &v.addr,
        }
    }

    /// Returns transaction fees of given ER node
    pub fn base_fee(&self) -> u16 {
        match self {
            Self::V0(v) => v.base_fee,
        }
    }

    /// Returns supported set of features by the given ER node
    pub fn features(&self) -> &FeaturesSet {
        match self {
            Self::V0(v) => &v.features,
        }
    }

    /// Returns the block time in ms of the given ER node
    pub fn block_time_ms(&self) -> u16 {
        match self {
            Self::V0(v) => v.block_time_ms,
        }
    }

    /// Returns current status of the given ER node
    pub fn status(&self) -> ErStatus {
        match self {
            Self::V0(v) => v.status,
        }
    }

    /// Returns load average of the given ER node
    pub fn load_average(&self) -> u32 {
        match self {
            Self::V0(v) => v.load_average,
        }
    }

    /// Returns 3 digit country code of the given ER node
    pub fn country_code(&self) -> CountryCode {
        match self {
            Self::V0(v) => v.country_code,
        }
    }

    /// Updates the FQDN address in the given ER record
    pub fn set_addr(&mut self, addr: String) {
        match self {
            Self::V0(v) => v.addr = addr,
        }
    }

    /// Updates base transaction fee in the given ER record
    pub fn set_base_fee(&mut self, base_fee: u16) {
        match self {
            Self::V0(v) => v.base_fee = base_fee,
        }
    }

    /// Updates the features set in the given ER record
    pub fn set_features(&mut self, features: FeaturesSet) {
        match self {
            Self::V0(v) => v.features = features,
        }
    }

    /// Updates block time in ms in the given ER record
    pub fn set_block_time_ms(&mut self, block_time_ms: u16) {
        match self {
            Self::V0(v) => v.block_time_ms = block_time_ms,
        }
    }

    /// Updates node status for the given ER record
    pub fn set_status(&mut self, status: ErStatus) {
        match self {
            Self::V0(v) => v.status = status,
        }
    }

    /// Updates the load average for the given ER record
    pub fn set_load_average(&mut self, load_average: u32) {
        match self {
            Self::V0(v) => v.load_average = load_average,
        }
    }

    /// Updates the country code for the given ER record
    pub fn set_country_code(&mut self, country_code: CountryCode) {
        match self {
            Self::V0(v) => v.country_code = country_code,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CountryCode([u8; 3]);

impl<S: AsRef<[u8]>> From<S> for CountryCode {
    fn from(value: S) -> Self {
        const LEN: usize = std::mem::size_of::<CountryCode>();
        let mut buf = [0u8; LEN];
        buf.copy_from_slice(&value.as_ref()[..LEN]);
        Self(buf)
    }
}
