use std::ops::BitOrAssign;

use borsh::{BorshDeserialize, BorshSerialize};

/// Number of bytes used for feature flags, we set this value
/// to 32, this gives 256 bits, i.e. 256 different features
const FEATURESET_BYTES: usize = 32;

/// Bit map of supported features
#[derive(Debug, Default, BorshSerialize, BorshDeserialize, PartialEq, Eq, Clone)]
pub struct FeaturesSet([u8; FEATURESET_BYTES]);

/// Individual custom extra feature supported by validator
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Feature {
    Randomness = 0,
    HighResClock = 1,
}

impl FeaturesSet {
    const SEGMENT: usize = u8::BITS as usize;

    /// Enables given feature in featureset
    pub fn activate(mut self, feature: Feature) -> Self {
        let (segment, offset) = self.locate(feature);
        segment.bitor_assign(1 << offset);
        self
    }

    #[cfg(test)]
    fn deactivate(&mut self, feature: Feature) {
        use std::ops::{BitAndAssign, Not};
        let (segment, offset) = self.locate(feature);
        segment.bitand_assign((1u8 << offset).not());
    }

    fn locate(&mut self, feature: Feature) -> (&mut u8, u8) {
        let index = feature as usize / Self::SEGMENT;
        let offset = feature as usize % Self::SEGMENT;
        // SAFETY: feature cannot exceed 255 (repr(u8)), 0..255 / 64 <= 3
        let segment = unsafe { self.0.get_unchecked_mut(index) };
        (segment, offset as u8)
    }

    /// Returns true if given featureset has requested feature enabled
    pub fn contains(&mut self, feature: Feature) -> bool {
        let (segment, offset) = self.locate(feature);
        (*segment & (1 << offset)) >> offset == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_features_op() {
        let mut features = FeaturesSet::default()
            .activate(Feature::Randomness)
            .activate(Feature::HighResClock);
        assert!(features.contains(Feature::Randomness));
        features.deactivate(Feature::Randomness);
        assert!(!features.contains(Feature::Randomness));
        assert!(features.contains(Feature::HighResClock));
    }
}
