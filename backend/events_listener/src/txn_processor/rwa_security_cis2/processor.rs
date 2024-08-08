use super::db;
use crate::txn_listener::EventsProcessor;
use async_trait::async_trait;
use chrono::Utc;
use concordium_cis2::{
    BurnEvent, Cis2Event, IsTokenAmount, IsTokenId, MintEvent, OperatorUpdate, TokenMetadataEvent,
    TransferEvent, UpdateOperatorEvent,
};
use concordium_rust_sdk::{
    base::{
        contracts_common::{Cursor, Deserial, Serial},
        hashes::ModuleReference,
        smart_contracts::{ContractEvent, OwnedContractName},
    },
    cis2,
    common::types::Timestamp,
    types::{Address, ContractAddress},
};
use concordium_rwa_backend_shared::db::*;
use concordium_rwa_utils::concordium_cis2_security::{
    AgentUpdatedEvent, Cis2SecurityEvent, ComplianceAdded, IdentityRegistryAdded, Paused,
    RecoverEvent, TokenDeposited, TokenFrozen,
};
use diesel::Connection;
use log::debug;
use num_bigint::BigUint;
use num_traits::Zero;
use std::marker::PhantomData;

fn cis2<T, A>(
    conn: &mut DbConn,
    now: Timestamp,
    cis2_address: &ContractAddress,
    event: Cis2Event<T, A>,
) -> anyhow::Result<()>
where
    T: IsTokenId + ToString,
    A: IsTokenAmount + Serial, {
    match event {
        Cis2Event::Mint(MintEvent {
            token_id,
            owner,
            amount,
        }) => {
            let token_id = token_id.to_string().parse()?;
            let token_amount = to_cis2_token_amount(&amount)?;
            conn.transaction(|conn| {
                db::insert_holder_or_add_balance(
                    conn,
                    &db::TokenHolder::new(
                        cis2_address,
                        &token_id,
                        &owner,
                        &token_amount,
                        &cis2::TokenAmount(BigUint::zero()),
                        now,
                    ),
                )?;
                db::update_supply(conn, cis2_address, &token_id, &token_amount, true)?;
                DbResult::Ok(())
            })?
        }
        Cis2Event::TokenMetadata(TokenMetadataEvent {
            token_id,
            metadata_url,
        }) => {
            let token_id = token_id.to_string().parse::<cis2::TokenId>()?;
            db::insert_token_or_update_metadata(
                conn,
                &db::Token::new(
                    cis2_address,
                    &token_id,
                    false,
                    metadata_url.url,
                    metadata_url.hash,
                    &cis2::TokenAmount(BigUint::zero()),
                    now,
                ),
            )?;
        }
        Cis2Event::Burn(BurnEvent {
            token_id,
            owner,
            amount,
        }) => {
            let token_id = token_id.to_string().parse::<cis2::TokenId>()?;
            let token_amount = to_cis2_token_amount(&amount)?;
            conn.transaction(|conn| {
                db::update_sub_balance(conn, cis2_address, &token_id, &owner, &token_amount)?;
                db::update_supply(conn, cis2_address, &token_id, &token_amount, false)?;
                DbResult::Ok(())
            })?;
        }
        Cis2Event::Transfer(TransferEvent {
            token_id,
            from,
            to,
            amount,
        }) => {
            let token_id = token_id.to_string().parse::<cis2::TokenId>()?;
            let token_amount = to_cis2_token_amount(&amount)?;
            conn.transaction(|conn| {
                db::update_sub_balance(conn, cis2_address, &token_id, &from, &token_amount)?;
                db::insert_holder_or_add_balance(
                    conn,
                    &db::TokenHolder::new(
                        cis2_address,
                        &token_id,
                        &to,
                        &token_amount,
                        &cis2::TokenAmount(BigUint::zero()),
                        now,
                    ),
                )?;
                DbResult::Ok(())
            })?;
        }
        Cis2Event::UpdateOperator(UpdateOperatorEvent {
            owner,
            operator,
            update,
        }) => {
            let record = db::Operator::new(cis2_address, &owner, &operator);
            match update {
                OperatorUpdate::Add => db::insert_operator(conn, &record)?,
                OperatorUpdate::Remove => db::delete_operator(conn, &record)?,
            }
        }
    }
    Ok(())
}

fn agent_added(
    conn: &mut DbConn,
    agent: Address,
    now: Timestamp,
    cis2_address: &ContractAddress,
) -> anyhow::Result<()> {
    Ok(db::insert_agent(conn, db::Agent::new(agent, now, cis2_address))?)
}

fn agent_removed(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    agent: Address,
) -> anyhow::Result<()> {
    db::remove_agent(conn, cis2_address, &agent)?;
    Ok(())
}

fn compliance_updated(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    compliance_contract: ContractAddress,
) -> anyhow::Result<()> {
    db::upsert_compliance(conn, &db::Compliance::new(cis2_address, &compliance_contract))?;
    Ok(())
}

fn identity_registry_updated(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    identity_registry_contract: ContractAddress,
) -> anyhow::Result<()> {
    db::upsert_identity_registry(
        conn,
        &db::IdentityRegistry::new(cis2_address, &identity_registry_contract),
    )?;
    Ok(())
}

fn token_paused<T>(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: T,
) -> anyhow::Result<()>
where
    T: IsTokenId + ToString, {
    db::update_token_paused(conn, cis2_address, &to_cis2_token_id(&token_id)?, true)?;
    Ok(())
}

fn token_unpaused<T>(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: T,
) -> anyhow::Result<()>
where
    T: IsTokenId + ToString, {
    db::update_token_paused(conn, cis2_address, &to_cis2_token_id(&token_id)?, false)?;
    Ok(())
}

fn account_recovered(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    lost_account: concordium_rust_sdk::types::Address,
    new_account: concordium_rust_sdk::types::Address,
) -> anyhow::Result<()> {
    let updated_rows = conn.transaction(|conn| {
        db::insert_recovery_record(
            conn,
            &db::RecoveryRecord::new(cis2_address, &lost_account, &new_account),
        )?;
        db::update_replace_holder(conn, cis2_address, &lost_account, &new_account)
    })?;
    debug!("account recovery, {} token ids updated", updated_rows);
    Ok(())
}

fn token_frozen<T, A>(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: T,
    address: concordium_rust_sdk::types::Address,
    amount: A,
) -> anyhow::Result<()>
where
    T: IsTokenId + ToString,
    A: IsTokenAmount, {
    db::update_balance_frozen(
        conn,
        cis2_address,
        &to_cis2_token_id(&token_id)?,
        &address,
        &to_cis2_token_amount(&amount)?,
        true,
    )?;

    Ok(())
}

fn token_un_frozen<T, A>(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    token_id: T,
    address: concordium_rust_sdk::types::Address,
    amount: A,
) -> anyhow::Result<()>
where
    T: IsTokenId + ToString,
    A: IsTokenAmount, {
    db::update_balance_frozen(
        conn,
        cis2_address,
        &to_cis2_token_id(&token_id)?,
        &address,
        &to_cis2_token_amount(&amount)?,
        false,
    )?;

    Ok(())
}

fn cis2_deposited<T, A>(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    deposited_contract: &ContractAddress,
    deposited_token_id: &T,
    deposited_holder_address: &Address,
    deposited_token_amount: &A,
) -> anyhow::Result<()>
where
    T: IsTokenId + ToString,
    A: IsTokenAmount, {
    db::insert_or_add_deposit_amount(
        conn,
        &db::Cis2Deposit::new(
            cis2_address,
            deposited_contract,
            &to_cis2_token_id(deposited_token_id)?,
            deposited_holder_address,
            &to_cis2_token_amount(deposited_token_amount)?,
        ),
    )?;

    Ok(())
}

fn cis2_withdrawn<T, A>(
    conn: &mut DbConn,
    cis2_address: &ContractAddress,
    deposited_contract: &ContractAddress,
    deposited_token_id: &T,
    deposited_holder_address: &Address,
    deposited_token_amount: &A,
) -> anyhow::Result<()>
where
    T: IsTokenId + ToString,
    A: IsTokenAmount, {
    db::update_sub_deposit_amount(
        conn,
        &db::Cis2Deposit::new(
            cis2_address,
            deposited_contract,
            &to_cis2_token_id(deposited_token_id)?,
            deposited_holder_address,
            &to_cis2_token_amount(deposited_token_amount)?,
        ),
    )?;

    Ok(())
}

fn to_cis2_token_amount<A>(amount: &A) -> Result<cis2::TokenAmount, anyhow::Error>
where
    A: IsTokenAmount + Serial, {
    let mut bytes = vec![];
    amount
        .serial(&mut bytes)
        .map_err(|_| anyhow::Error::msg("error serializing amount to bytes"))?;
    let mut cursor: Cursor<_> = Cursor::new(bytes);
    let amount = cis2::TokenAmount::deserial(&mut cursor)?;

    Ok(amount)
}

fn to_cis2_token_id<T>(token_id: &T) -> Result<cis2::TokenId, anyhow::Error>
where
    T: IsTokenId + ToString, {
    let token_id = token_id.to_string().parse()?;
    Ok(token_id)
}

type Event<T, A> = Cis2SecurityEvent<T, A>;
pub fn process_events<T, A>(
    conn: &mut DbConn,
    now: Timestamp,
    cis2_address: &ContractAddress,
    events: &[ContractEvent],
) -> anyhow::Result<()>
where
    T: IsTokenId + std::fmt::Debug + ToString,
    A: IsTokenAmount + std::fmt::Debug, {
    for event in events {
        let parsed_event = event.parse::<Event<T, A>>()?;
        debug!("Event: {}/{}", cis2_address.index, cis2_address.subindex);
        debug!("{:#?}", parsed_event);

        match parsed_event {
            Event::Deposited(TokenDeposited {
                token_id,
                owner,
                amount,
            }) => cis2_deposited(
                conn,
                cis2_address,
                &token_id.contract,
                &token_id.id,
                &Address::Account(owner),
                &amount,
            )?,
            Event::Withdraw(TokenDeposited {
                token_id,
                owner,
                amount,
            }) => cis2_withdrawn(
                conn,
                cis2_address,
                &token_id.contract,
                &token_id.id,
                &Address::Account(owner),
                &amount,
            )?,
            Event::AgentAdded(AgentUpdatedEvent {
                agent,
            }) => agent_added(conn, agent, now, cis2_address)?,
            Event::AgentRemoved(AgentUpdatedEvent {
                agent,
            }) => agent_removed(conn, cis2_address, agent)?,
            Event::ComplianceAdded(ComplianceAdded(compliance_contract)) => {
                compliance_updated(conn, cis2_address, compliance_contract)?
            }
            Event::IdentityRegistryAdded(IdentityRegistryAdded(identity_registry_contract)) => {
                identity_registry_updated(conn, cis2_address, identity_registry_contract)?
            }
            Event::Paused(Paused {
                token_id,
            }) => token_paused(conn, cis2_address, token_id)?,
            Event::UnPaused(Paused {
                token_id,
            }) => token_unpaused(conn, cis2_address, token_id)?,
            Event::Recovered(RecoverEvent {
                lost_account,
                new_account,
            }) => account_recovered(conn, cis2_address, lost_account, new_account)?,
            Event::TokenFrozen(TokenFrozen {
                address,
                amount,
                token_id,
            }) => token_frozen(conn, cis2_address, token_id, address, amount)?,
            Event::TokenUnFrozen(TokenFrozen {
                address,
                amount,
                token_id,
            }) => token_un_frozen(conn, cis2_address, token_id, address, amount)?,
            Event::Cis2(e) => cis2(conn, now, cis2_address, e)?,
        }
    }

    Ok(())
}

pub struct RwaSecurityCIS2Processor<T, A> {
    pub pool:                  DbPool,
    /// Module reference of the contract.
    pub module_ref:            ModuleReference,
    /// Name of the contract.
    pub contract_name:         OwnedContractName,
    pub _phantom_token_id:     PhantomData<T>,
    pub _phantom_token_amount: PhantomData<A>,
}

impl<T, A> RwaSecurityCIS2Processor<T, A> {
    pub fn new(
        pool: DbPool,
        module_ref: ModuleReference,
        contract_name: OwnedContractName,
    ) -> Self {
        Self {
            pool,
            module_ref,
            contract_name,
            _phantom_token_id: Default::default(),
            _phantom_token_amount: Default::default(),
        }
    }
}

#[async_trait]
impl<T, A> EventsProcessor for RwaSecurityCIS2Processor<T, A>
where
    T: IsTokenId + Send + Sync + std::fmt::Debug + ToString,
    A: IsTokenAmount + Send + Sync + std::fmt::Debug,
{
    /// Returns the name of the contract this processor is responsible for.
    ///
    /// # Returns
    ///
    /// * A reference to the `OwnedContractName` of the contract.
    fn contract_name(&self) -> &OwnedContractName { &self.contract_name }

    /// Returns the module reference of the contract this processor is
    /// responsible for.
    ///
    /// # Returns
    ///
    /// * A reference to the `ModuleReference` of the contract.
    fn module_ref(&self) -> &ModuleReference { &self.module_ref }

    /// Processes the events of the contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - A reference to the `ContractAddress` of the contract
    ///   whose events are to be processed.
    /// * `events` - A slice of `ContractEvent`s to be processed.
    ///
    /// # Returns
    ///
    /// * A Result indicating the success or failure of the operation.
    async fn process_events(
        &mut self,
        contract: &ContractAddress,
        events: &[ContractEvent],
    ) -> anyhow::Result<u64> {
        let mut conn = self.pool.get()?;
        let now = Timestamp {
            millis: Utc::now().timestamp_millis() as u64,
        };
        process_events::<T, A>(&mut conn, now, contract, events)?;
        Ok(events.len() as u64)
    }
}

#[cfg(test)]
mod tests {
    use concordium_cis2::{TokenAmountU64, TokenAmountU8};
    use concordium_rust_sdk::cis2;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    use super::to_cis2_token_amount;

    #[test]
    fn token_amount_conversions() {
        let amount = to_cis2_token_amount(&TokenAmountU8(0)).expect("should convert token amount");
        assert_eq!(amount, cis2::TokenAmount(BigUint::from_u8(0).unwrap()));
        assert_eq!(amount.to_string(), "0");

        let amount: u8 = 255;
        let token_amount =
            to_cis2_token_amount(&TokenAmountU8(amount)).expect("should convert token amount");
        assert_eq!(token_amount, cis2::TokenAmount(BigUint::from_u8(amount).unwrap()));
        assert_eq!(token_amount.to_string(), "255");

        let amount: u64 = 255;
        let token_amount =
            to_cis2_token_amount(&TokenAmountU64(amount)).expect("should convert token amount");
        assert_eq!(token_amount, cis2::TokenAmount(BigUint::from_u64(amount).unwrap()));
        assert_eq!(token_amount.to_string(), "255");
    }
}
