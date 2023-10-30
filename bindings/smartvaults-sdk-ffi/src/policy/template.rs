// Copyright (c) 2022-2023 Smart Vaults
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use smartvaults_sdk::core::bitcoin::absolute;
use smartvaults_sdk::core::miniscript::DescriptorPublicKey;
use smartvaults_sdk::core::{self, bitcoin};

use crate::error::Result;
use crate::Descriptor;

pub struct RelativeLockTime {
    inner: bitcoin::Sequence,
}

impl Deref for RelativeLockTime {
    type Target = bitcoin::Sequence;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl RelativeLockTime {
    pub fn from_blocks(blocks: u16) -> Self {
        Self {
            inner: bitcoin::Sequence::from_height(blocks),
        }
    }
}

pub struct AbsoluteLockTime {
    inner: absolute::LockTime,
}

impl Deref for AbsoluteLockTime {
    type Target = absolute::LockTime;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AbsoluteLockTime {
    pub fn from_height(height: u32) -> Result<Self> {
        Ok(Self {
            inner: absolute::LockTime::from_height(height)?,
        })
    }

    pub fn from_timestamp(timestamp: u32) -> Result<Self> {
        Ok(Self {
            inner: absolute::LockTime::from_time(timestamp)?,
        })
    }
}

pub struct Locktime {
    inner: core::Locktime,
}

impl Deref for Locktime {
    type Target = core::Locktime;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Locktime {
    pub fn absolute(absolute: Arc<AbsoluteLockTime>) -> Self {
        Self {
            inner: core::Locktime::After(**absolute),
        }
    }

    pub fn relative(relative: Arc<RelativeLockTime>) -> Self {
        Self {
            inner: core::Locktime::Older(**relative),
        }
    }
}

pub struct DecayingTime {
    inner: core::DecayingTime,
}

impl Deref for DecayingTime {
    type Target = core::DecayingTime;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DecayingTime {
    pub fn single(locktime: Arc<Locktime>) -> Self {
        Self {
            inner: core::DecayingTime::Single(**locktime),
        }
    }

    pub fn multiple(locktimes: Vec<Arc<Locktime>>) -> Self {
        Self {
            inner: core::DecayingTime::Multiple(locktimes.into_iter().map(|l| **l).collect()),
        }
    }
}

pub struct RecoveryTemplate {
    inner: core::RecoveryTemplate,
}

impl Deref for RecoveryTemplate {
    type Target = core::RecoveryTemplate;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl RecoveryTemplate {
    pub fn new(threshold: u64, keys: Vec<Arc<Descriptor>>, locktime: Arc<Locktime>) -> Self {
        let keys: Vec<DescriptorPublicKey> = keys
            .into_iter()
            .map(|k| k.as_ref().deref().clone())
            .collect();
        Self {
            inner: core::RecoveryTemplate::new(threshold as usize, keys, **locktime),
        }
    }
}

pub struct PolicyTemplate {
    inner: core::PolicyTemplate,
}

impl Deref for PolicyTemplate {
    type Target = core::PolicyTemplate;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<core::PolicyTemplate> for PolicyTemplate {
    fn from(inner: core::PolicyTemplate) -> Self {
        Self { inner }
    }
}

impl PolicyTemplate {
    pub fn multisig(threshold: u64, keys: Vec<Arc<Descriptor>>) -> Self {
        let keys: Vec<DescriptorPublicKey> = keys
            .into_iter()
            .map(|k| k.as_ref().deref().clone())
            .collect();
        Self {
            inner: core::PolicyTemplate::multisig(threshold as usize, keys),
        }
    }

    pub fn recovery(my_key: Arc<Descriptor>, recovery: Arc<RecoveryTemplate>) -> Self {
        Self {
            inner: core::PolicyTemplate::recovery(
                my_key.as_ref().deref().clone(),
                recovery.as_ref().deref().clone(),
            ),
        }
    }

    pub fn hold(my_key: Arc<Descriptor>, locktime: Arc<Locktime>) -> Self {
        Self {
            inner: core::PolicyTemplate::hold(my_key.as_ref().deref().clone(), **locktime),
        }
    }

    pub fn decaying(
        start_threshold: u64,
        keys: Vec<Arc<Descriptor>>,
        time: Arc<DecayingTime>,
    ) -> Self {
        let keys: Vec<DescriptorPublicKey> = keys
            .into_iter()
            .map(|k| k.as_ref().deref().clone())
            .collect();
        Self {
            inner: core::PolicyTemplate::decaying(
                start_threshold as usize,
                keys,
                time.as_ref().deref().clone(),
            ),
        }
    }
}
