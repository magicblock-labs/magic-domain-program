use std::ops::BitOrAssign;

use borsh::{BorshDeserialize, BorshSerialize};

#[cfg(feature = "no-entrypoint")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(feature = "no-entrypoint", derive(Clone))]
pub struct FeaturesSet([u64; 4]);

/// A first approximation of features supported by ER validator
#[derive(Clone, Copy)]
#[cfg_attr(feature = "no-entrypoint", derive(Serialize))]
#[repr(u8)]
pub enum Feature {
    Randomness = 0,
    // .. and nothing else comes to mind, 256 should be more than enough, may be we should even
    // reduce it to 128, so that no space is wasted
}

impl FeaturesSet {
    const SEGMENT: usize = u64::BITS as usize;

    pub fn activate(mut self, feature: Feature) -> Self {
        let (segment, offset) = self.locate(feature);
        segment.bitor_assign(1 << offset);
        self
    }

    #[cfg(test)]
    fn deactivate(&mut self, feature: Feature) {
        use std::ops::{BitAndAssign, Not};
        let (segment, offset) = self.locate(feature);
        segment.bitand_assign((1_u64 << offset).not());
    }

    fn locate(&mut self, feature: Feature) -> (&mut u64, u64) {
        let index = feature as usize / Self::SEGMENT;
        let offset = feature as usize % Self::SEGMENT;
        // SAFETY: feature cannot exceed 255 (repr(u8)), 0..255 / 64 <= 3
        let segment = unsafe { self.0.get_unchecked_mut(index) };
        (segment, offset as u64)
    }

    pub fn contains(&mut self, feature: Feature) -> bool {
        let (segment, offset) = self.locate(feature);
        (*segment & (1 << offset)) >> offset == 1
    }
}

#[cfg(test)]
#[test]
fn test_features_op() {
    let mut features = FeaturesSet::default().activate(Feature::Randomness);
    assert!(features.contains(Feature::Randomness));
    features.deactivate(Feature::Randomness);
    assert!(!features.contains(Feature::Randomness));
}
