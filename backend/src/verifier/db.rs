use bson::{doc, to_bson};
use chrono::{DateTime, Utc};
use concordium_rust_sdk::{smart_contracts::common::AccountAddress, types::ContractAddress};
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbChallenge {
    pub challenge:  String,
    pub address:    AccountAddress,
    pub created_at: DateTime<Utc>,
}

pub struct Db {
    pub client:            mongodb::Client,
    pub identity_registry: ContractAddress,
    pub agent_address:     AccountAddress,
}

impl Db {
    fn database(&self) -> mongodb::Database {
        self.client.database(&format!(
            "verifier-{}-{}-{}",
            self.identity_registry.index,
            self.identity_registry.subindex,
            // truncated due to mongodb collection name length limit
            &self.agent_address.to_string()[0..6]
        ))
    }

    fn challenges(&self) -> Collection<DbChallenge> {
        self.database().collection::<DbChallenge>("challenges")
    }

    pub async fn insert_challenge(&self, challenge: DbChallenge) -> anyhow::Result<()> {
        self.challenges().insert_one(challenge, None).await?;
        Ok(())
    }

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
