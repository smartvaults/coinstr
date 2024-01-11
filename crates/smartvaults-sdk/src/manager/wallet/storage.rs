// Copyright (c) 2022-2024 Smart Vaults
// Distributed under the MIT software license

use nostr_sdk::hashes::sha256::Hash as Sha256Hash;
use smartvaults_core::bdk::chain::{Append, PersistBackend};
use smartvaults_sdk_sqlite::{Error as DbError, Store, StoreEncryption};
use thiserror::Error;
use tokio::runtime::Handle;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Store(#[from] DbError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct SmartVaultsWalletStorage {
    descriptor_hash: Sha256Hash,
    db: Store,
}

impl SmartVaultsWalletStorage {
    pub fn new(descriptor_hash: Sha256Hash, db: Store) -> Self {
        Self {
            descriptor_hash,
            db,
        }
    }
}

impl<K> PersistBackend<K> for SmartVaultsWalletStorage
where
    K: Default + Clone + Append + StoreEncryption + Send + 'static,
{
    type LoadError = Error;
    type WriteError = Error;

    fn write_changes(&mut self, changeset: &K) -> Result<(), Self::WriteError> {
        if changeset.is_empty() {
            return Ok(());
        }

        let handle = Handle::current();
        let _ = handle.enter();
        futures::executor::block_on(async {
            match self.db.get_changeset::<K>(self.descriptor_hash).await.ok() {
                Some(mut keychain_store) => {
                    keychain_store.append(changeset.clone());
                    self.db
                        .save_changeset(self.descriptor_hash, keychain_store.clone())
                        .await?
                }
                None => {
                    self.db
                        .save_changeset(self.descriptor_hash, changeset.clone())
                        .await?
                }
            };

            Ok(())
        })
    }

    fn load_from_persistence(&mut self) -> Result<K, Self::LoadError> {
        let handle = Handle::current();
        let _ = handle.enter();
        futures::executor::block_on(async {
            match self.db.get_changeset::<K>(self.descriptor_hash).await {
                Ok(k) => Ok(k),
                Err(DbError::NotFound(_)) => {
                    tracing::warn!("Change set not found, using the default one");
                    Ok(K::default())
                }
                Err(e) => {
                    tracing::error!("Impossible to load changeset: {e}");
                    Ok(K::default())
                }
            }
        })
    }
}
