pub const VALIDATOR_INFO_SEED: &[u8] = b"validator-info";

pub mod tags {
    pub type FieldTag = u8;

    pub const IDENTITY_TAG: u8 = 0;
    pub const BLOCK_TIME_MS_TAG: u8 = 1;
    pub const FEES_TAG: u8 = 2;
    pub const FEATURES_TAG: u8 = 3;
    pub const ADDR_TAG: u8 = 4;
}

pub mod ix {
    pub const REGISTER_IX: u8 = 0;
    pub const SYNC_RECORD_IX: u8 = 1;
    pub const UNREGISTER_IX: u8 = 2;
}
