use borsh::{BorshDeserialize, BorshSerialize};

/// Possible states for ER node to be in
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Copy, PartialEq, Eq)]
#[borsh(use_discriminant = true)]
pub enum ErStatus {
    /// node is active and accepting new delegations
    Active = 0,
    /// node is active but not accepting new delegations, only serving existing ones
    Draining = 1,
    /// node has been taken offline, e.g. for maintenance
    Offline = 2,
}
