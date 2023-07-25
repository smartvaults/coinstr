// Copyright (c) 2022-2023 Coinstr
// Distributed under the MIT software license

use core::fmt;

pub use keechain_core::types::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd)]
pub enum Priority {
    /// High: confirm in 1 blocks
    High,
    /// Medium: confirm in 6 blocks
    #[default]
    Medium,
    /// Low: confirm in 12 blocks
    Low,
    /// Target blocks
    Custom(u8),
}

impl Priority {
    pub fn target_blocks(&self) -> u8 {
        match self {
            Self::High => 1,
            Self::Medium => 6,
            Self::Low => 12,
            Self::Custom(target) => *target,
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::High => write!(f, "High (10 - 20 minutes)"),
            Self::Medium => write!(f, "Medium (20 - 60 minutes)"),
            Self::Low => write!(f, "Low (1 - 2 hours)"),
            Self::Custom(target) => write!(f, "Custom ({target} blocks)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum FeeRate {
    /// Target blocks
    Priority(Priority),
    /// sat/vByte
    Rate(f32),
}

impl FeeRate {
    pub fn min_relay_fee() -> Self {
        Self::Rate(1.0)
    }

    /// Check if fee is valid
    pub fn is_valid(&self) -> bool {
        if let Self::Rate(rate) = self {
            if *rate < 1.0 {
                return false;
            }
        }

        true
    }
}

impl Default for FeeRate {
    fn default() -> Self {
        Self::Priority(Priority::default())
    }
}

impl Eq for FeeRate {}

impl fmt::Display for FeeRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Priority(priority) => write!(f, "{priority}"),
            Self::Rate(rate) => write!(f, "{rate} sat/vByte"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Amount {
    Max,
    Custom(u64),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_fee_rate() {
        assert!(FeeRate::Priority(Priority::High).is_valid());
        assert!(FeeRate::Rate(1.5).is_valid());
        assert!(FeeRate::Rate(180.2).is_valid());
    }

    #[test]
    fn test_invalid_fee_rate() {
        assert!(!FeeRate::Rate(0.0).is_valid());
        assert!(!FeeRate::Rate(0.9).is_valid());
        assert!(!FeeRate::Rate(-10.0).is_valid());
    }
}
