use crate::errors::PasskeyError;
use crate::passkey::{StoredChallenge, StoredCredential};
use async_trait::async_trait;
use std::env;

mod memory;
mod postgres;
mod redis;
mod sqlite;

#[derive(Clone, Debug)]
pub enum ChallengeStoreType {
    Memory,
    Sqlite { url: String },
    Postgres { url: String },
    Redis { url: String },
}

#[derive(Clone, Debug)]
pub enum CredentialStoreType {
    Memory,
    Sqlite { url: String },
    Postgres { url: String },
    Redis { url: String },
}

impl ChallengeStoreType {
    pub fn from_env() -> Result<Self, PasskeyError> {
        dotenv::dotenv().ok();

        let store_type = env::var("PASSKEY_CHALLENGE_STORE")
            .unwrap_or_else(|_| "memory".to_string())
            .to_lowercase();

        match store_type.as_str() {
            "memory" => Ok(ChallengeStoreType::Memory),
            "sqlite" => {
                let url = env::var("PASSKEY_CHALLENGE_SQLITE_URL")
                    .unwrap_or_else(|_| "PASSKEY_CHALLENGE_SQLITE_URL not set".to_string());
                Ok(ChallengeStoreType::Sqlite { url })
            }
            "postgres" => {
                let url = env::var("PASSKEY_CHALLENGE_POSTGRES_URL").map_err(|_| {
                    PasskeyError::Storage("PASSKEY_CHALLENGE_POSTGRES_URL not set".to_string())
                })?;
                Ok(ChallengeStoreType::Postgres { url })
            }
            "redis" => {
                let url = env::var("PASSKEY_CHALLENGE_REDIS_URL").map_err(|_| {
                    PasskeyError::Storage("PASSKEY_CHALLENGE_REDIS_URL not set".to_string())
                })?;
                Ok(ChallengeStoreType::Redis { url })
            }
            _ => Err(PasskeyError::Storage(format!(
                "Unknown challenge store type: {}",
                store_type
            ))),
        }
    }

    pub(crate) async fn create_store(&self) -> Result<Box<dyn ChallengeStore>, PasskeyError> {
        match self {
            ChallengeStoreType::Memory => Ok(Box::new(memory::InMemoryChallengeStore::new())),
            ChallengeStoreType::Sqlite { url } => {
                Ok(Box::new(sqlite::SqliteChallengeStore::connect(url).await?))
            }
            ChallengeStoreType::Postgres { url } => Ok(Box::new(
                postgres::PostgresChallengeStore::connect(url).await?,
            )),
            ChallengeStoreType::Redis { url } => {
                Ok(Box::new(redis::RedisChallengeStore::connect(url).await?))
            }
        }
    }
}

impl CredentialStoreType {
    pub fn from_env() -> Result<Self, PasskeyError> {
        dotenv::dotenv().ok();

        let store_type = env::var("PASSKEY_CREDENTIAL_STORE")
            .unwrap_or_else(|_| "memory".to_string())
            .to_lowercase();

        match store_type.as_str() {
            "memory" => Ok(CredentialStoreType::Memory),
            "sqlite" => {
                let url = env::var("PASSKEY_CREDENTIAL_SQLITE_URL")
                    .unwrap_or_else(|_| "PASSKEY_CREDENTIAL_SQLITE_URL not set".to_string());
                Ok(CredentialStoreType::Sqlite { url })
            }
            "postgres" => {
                let url = env::var("PASSKEY_CREDENTIAL_POSTGRES_URL").map_err(|_| {
                    PasskeyError::Storage("PASSKEY_CREDENTIAL_POSTGRES_URL not set".to_string())
                })?;
                Ok(CredentialStoreType::Postgres { url })
            }
            "redis" => {
                let url = env::var("PASSKEY_CREDENTIAL_REDIS_URL").map_err(|_| {
                    PasskeyError::Storage("PASSKEY_CREDENTIAL_REDIS_URL not set".to_string())
                })?;
                Ok(CredentialStoreType::Redis { url })
            }
            _ => Err(PasskeyError::Storage(format!(
                "Unknown credential store type: {}",
                store_type
            ))),
        }
    }

    pub(crate) async fn create_store(&self) -> Result<Box<dyn CredentialStore>, PasskeyError> {
        match self {
            CredentialStoreType::Memory => Ok(Box::new(memory::InMemoryCredentialStore::new())),
            CredentialStoreType::Sqlite { url } => {
                Ok(Box::new(sqlite::SqliteCredentialStore::connect(url).await?))
            }
            CredentialStoreType::Postgres { url } => Ok(Box::new(
                postgres::PostgresCredentialStore::connect(url).await?,
            )),
            CredentialStoreType::Redis { url } => {
                Ok(Box::new(redis::RedisCredentialStore::connect(url).await?))
            }
        }
    }
}

#[async_trait]
pub(crate) trait ChallengeStore: Send + Sync + 'static {
    /// Initialize the store. This is called when the store is created.
    async fn init(&self) -> Result<(), PasskeyError>;

    async fn store_challenge(
        &mut self,
        challenge_id: String,
        challenge: StoredChallenge,
    ) -> Result<(), PasskeyError>;

    async fn get_challenge(
        &self,
        challenge_id: &str,
    ) -> Result<Option<StoredChallenge>, PasskeyError>;

    async fn remove_challenge(&mut self, challenge_id: &str) -> Result<(), PasskeyError>;
}

#[async_trait]
pub(crate) trait CredentialStore: Send + Sync + 'static {
    /// Initialize the store. This is called when the store is created.
    async fn init(&self) -> Result<(), PasskeyError>;

    async fn store_credential(
        &mut self,
        credential_id: String,
        credential: StoredCredential,
    ) -> Result<(), PasskeyError>;

    async fn get_credential(
        &self,
        credential_id: &str,
    ) -> Result<Option<StoredCredential>, PasskeyError>;

    async fn get_credentials_by_username(
        &self,
        username: &str,
    ) -> Result<Vec<StoredCredential>, PasskeyError>;

    async fn update_credential_counter(
        &mut self,
        credential_id: &str,
        new_counter: u32,
    ) -> Result<(), PasskeyError>;

    async fn get_all_credentials(&self) -> Result<Vec<StoredCredential>, PasskeyError>;
}
