use bson::{doc, to_bson};
use chrono::{DateTime, Utc};
use concordium_rust_sdk::{smart_contracts::common::AccountAddress, types::ContractAddress};
use mongodb::Collection;
use serde::{Deserialize, Serialize};

/// Represents a challenge stored in the database.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbChallenge {
    pub challenge:  String,
    pub address:    AccountAddress,
    pub created_at: DateTime<Utc>,
}

/// Represents a database connection.
pub struct Db {
    pub client:            mongodb::Client,
    pub identity_registry: ContractAddress,
    pub agent_address:     AccountAddress,
}

impl Db {
    /// Returns the MongoDB database associated with the verifier.
    fn database(&self) -> mongodb::Database {
        self.client.database(&format!(
            "verifier-{}-{}-{}",
            self.identity_registry.index,
            self.identity_registry.subindex,
            // truncated due to mongodb collection name length limit
            &self.agent_address.to_string()[0..6]
        ))
    }

    /// Returns the MongoDB collection for storing challenges.
    fn challenges(&self) -> Collection<DbChallenge> {
        self.database().collection::<DbChallenge>("challenges")
    }

    /// Inserts a challenge into the database.
    ///
    /// # Arguments
    ///
    /// * `challenge` - The challenge to be inserted.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the insertion is successful, otherwise returns an
    /// `anyhow::Result` with an error.
    pub async fn insert_challenge(&self, challenge: DbChallenge) -> anyhow::Result<()> {
        self.challenges().insert_one(challenge, None).await?;
        Ok(())
    }

    /// Finds a challenge in the database by address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the challenge to be found.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(challenge))` if a matching challenge is found,
    /// `Ok(None)` if no matching challenge is found, otherwise returns an
    /// `anyhow::Result` with an error.
    pub async fn find_challenge(
        &self,
        address: &AccountAddress,
    ) -> anyhow::Result<Option<DbChallenge>> {
        let challenge = self
            .challenges()
            .find_one(
                doc! {
                    "address": to_bson(&address.to_string())?
                },
                None,
            )
            .await?;

        Ok(challenge)
    }
}
