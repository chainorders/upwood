use core::fmt;

use chrono::NaiveDateTime;
use concordium_cis2::{
    BurnEvent, Cis2Event, IsTokenAmount, IsTokenId, MintEvent, OperatorUpdate, TokenMetadataEvent,
    TransferEvent, UpdateOperatorEvent,
};
use concordium_protocols::concordium_cis2_security::*;
use concordium_rust_sdk::base::contracts_common::{Deserial, Serial};
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use diesel::Connection;
use rust_decimal::Decimal;
use shared::db::cis2_security::{
    Agent, Compliance, IdentityRegistry, Operator, RecoveryRecord, Token, TokenHolder,
    TokenHolderBalanceUpdate, TokenHolderBalanceUpdateType,
};
use shared::db_shared::{DbConn, DbResult};
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::processors::cis2_utils::*;
use crate::processors::ProcessorError;

#[instrument(
    name="cis2_security_process_events",
    skip_all,
    fields(contract = %contract)
)]
pub fn process_events_cis2<T, A>(
    conn: &mut DbConn,
    now: NaiveDateTime,
    contract: &ContractAddress,
    event: Cis2Event<T, A>,
) -> DbResult<()>
where
    T: IsTokenId,
    A: IsTokenAmount+Serial,
{
    match event {
        Cis2Event::Mint(MintEvent {
            token_id,
            owner,
            amount,
        }) => {
            let token_id = token_id.to_decimal();
            let amount = amount.to_decimal();
            let holder = TokenHolder::new(contract.to_decimal(), token_id, &owner, amount, now)
                .upsert(conn)?;
            TokenHolderBalanceUpdate {
                id: Uuid::new_v4(),
                cis2_address: contract.to_decimal(),
                token_id,
                holder_address: owner.to_string(),
                amount,
                frozen_balance: holder.frozen_balance,
                un_frozen_balance: holder.un_frozen_balance,
                update_type: TokenHolderBalanceUpdateType::Mint,
                create_time: now,
            }
            .insert(conn)?;
            Token::update_supply(conn, contract.to_decimal(), token_id, amount, true)?;
            DbResult::Ok(())
        }
        Cis2Event::TokenMetadata(TokenMetadataEvent {
            token_id,
            metadata_url,
        }) => Token::new(
            contract.to_decimal(),
            token_id.to_decimal(),
            false,
            metadata_url.url,
            metadata_url.hash,
            Decimal::ZERO,
            now,
        )
        .upsert(conn),
        Cis2Event::Burn(BurnEvent {
            token_id,
            owner,
            amount,
        }) => {
            let token_id = token_id.to_decimal();
            let amount = amount.to_decimal();
            let holder = TokenHolder::sub_balance_unfrozen(
                conn,
                contract.to_decimal(),
                token_id,
                &owner,
                amount,
            )?;
            TokenHolderBalanceUpdate {
                id: Uuid::new_v4(),
                cis2_address: contract.to_decimal(),
                token_id,
                holder_address: owner.to_string(),
                amount,
                frozen_balance: holder.frozen_balance,
                un_frozen_balance: holder.un_frozen_balance,
                update_type: TokenHolderBalanceUpdateType::Burn,
                create_time: now,
            }
            .insert(conn)?;
            Token::update_supply(conn, contract.to_decimal(), token_id, amount, false)?;
            DbResult::Ok(())
        }
        Cis2Event::Transfer(TransferEvent {
            token_id,
            from,
            to,
            amount,
        }) => {
            let token_id = token_id.to_decimal();
            let amount = amount.to_decimal();
            let holder = TokenHolder::sub_balance_unfrozen(
                conn,
                contract.to_decimal(),
                token_id,
                &from,
                amount,
            )?;
            TokenHolderBalanceUpdate {
                id: Uuid::new_v4(),
                cis2_address: contract.to_decimal(),
                token_id,
                holder_address: holder.holder_address,
                amount,
                frozen_balance: holder.frozen_balance,
                un_frozen_balance: holder.un_frozen_balance,
                update_type: TokenHolderBalanceUpdateType::Transfer,
                create_time: now,
            }
            .insert(conn)?;
            let holder =
                TokenHolder::new(contract.to_decimal(), token_id, &to, amount, now).upsert(conn)?;
            TokenHolderBalanceUpdate {
                id: Uuid::new_v4(),
                cis2_address: contract.to_decimal(),
                token_id,
                holder_address: to.to_string(),
                amount,
                frozen_balance: holder.frozen_balance,
                un_frozen_balance: holder.un_frozen_balance,
                update_type: TokenHolderBalanceUpdateType::Recieved,
                create_time: now,
            }
            .insert(conn)?;
            DbResult::Ok(())
        }
        Cis2Event::UpdateOperator(UpdateOperatorEvent {
            owner,
            operator,
            update,
        }) => {
            let record = Operator::new(contract.to_decimal(), &owner, &operator);
            match update {
                OperatorUpdate::Add => record.insert(conn),
                OperatorUpdate::Remove => record.delete(conn),
            }
        }
    }
}

pub fn process_events<T, A, R>(
    conn: &mut DbConn,
    now: NaiveDateTime,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError>
where
    T: IsTokenId+fmt::Debug,
    A: IsTokenAmount+fmt::Debug,
    R: Deserial+fmt::Debug+std::string::ToString,
{
    for event in events {
        let parsed_event = event
            .parse::<Cis2SecurityEvent<T, A, R>>()
            .expect("Failed to parse event");
        debug!("Event: {}/{}", contract.index, contract.subindex);
        debug!("{:#?}", parsed_event);

        process_parsed_event(conn, contract, parsed_event, now)?;
    }

    Ok(())
}

pub fn process_parsed_event<T, A, R>(
    conn: &mut r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
    contract: &ContractAddress,
    parsed_event: Cis2SecurityEvent<T, A, R>,
    now: NaiveDateTime,
) -> Result<(), ProcessorError>
where
    T: IsTokenId+fmt::Debug,
    A: IsTokenAmount+fmt::Debug,
    R: Deserial+fmt::Debug+std::string::ToString,
{
    match parsed_event {
        Cis2SecurityEvent::AgentAdded(AgentUpdatedEvent { agent, roles }) => Agent::new(
            agent,
            now,
            contract.to_decimal(),
            roles.iter().map(|r| r.to_string()).collect(),
        )
        .insert(conn)?,
        Cis2SecurityEvent::AgentRemoved(AgentUpdatedEvent { agent, roles: _ }) => {
            Agent::delete(conn, contract.to_decimal(), &agent)?
        }
        Cis2SecurityEvent::ComplianceAdded(ComplianceAdded(compliance_contract)) => {
            Compliance::new(contract.to_decimal(), compliance_contract.to_decimal()).upsert(conn)?
        }
        Cis2SecurityEvent::IdentityRegistryAdded(IdentityRegistryAdded(
            identity_registry_contract,
        )) => IdentityRegistry::new(
            contract.to_decimal(),
            identity_registry_contract.to_decimal(),
        )
        .upsert(conn)?,
        Cis2SecurityEvent::Paused(Paused { token_id }) => {
            Token::update_paused(conn, contract.to_decimal(), token_id.to_decimal(), true)?
        }
        Cis2SecurityEvent::UnPaused(Paused { token_id }) => {
            Token::update_paused(conn, contract.to_decimal(), token_id.to_decimal(), false)?
        }
        Cis2SecurityEvent::Recovered(RecoverEvent {
            lost_account,
            new_account,
        }) => {
            let updated_rows = conn.transaction(|conn| {
                RecoveryRecord::new(contract.to_decimal(), &lost_account, &new_account)
                    .insert(conn)?;
                TokenHolder::replace(conn, contract.to_decimal(), &lost_account, &new_account)
            })?;
            debug!("account recovery, {} token ids updated", updated_rows);
        }
        Cis2SecurityEvent::TokenFrozen(TokenFrozen {
            address,
            amount,
            token_id,
        }) => {
            let holder = TokenHolder::update_balance_frozen(
                conn,
                contract.to_decimal(),
                token_id.to_decimal(),
                &address,
                amount.to_decimal(),
                true,
            )?;
            TokenHolderBalanceUpdate {
                id:                Uuid::new_v4(),
                cis2_address:      contract.to_decimal(),
                token_id:          token_id.to_decimal(),
                holder_address:    address.to_string(),
                amount:            amount.to_decimal(),
                frozen_balance:    holder.frozen_balance,
                un_frozen_balance: holder.un_frozen_balance,
                update_type:       TokenHolderBalanceUpdateType::Freeze,
                create_time:       now,
            }
            .insert(conn)?;
        }
        Cis2SecurityEvent::TokenUnFrozen(TokenFrozen {
            address,
            amount,
            token_id,
        }) => {
            let holder = TokenHolder::update_balance_frozen(
                conn,
                contract.to_decimal(),
                token_id.to_decimal(),
                &address,
                amount.to_decimal(),
                false,
            )?;
            TokenHolderBalanceUpdate {
                id:                Uuid::new_v4(),
                cis2_address:      contract.to_decimal(),
                token_id:          token_id.to_decimal(),
                holder_address:    address.to_string(),
                amount:            amount.to_decimal(),
                frozen_balance:    holder.frozen_balance,
                un_frozen_balance: holder.un_frozen_balance,
                update_type:       TokenHolderBalanceUpdateType::UnFreeze,
                create_time:       now,
            }
            .insert(conn)?;
        }
        Cis2SecurityEvent::Cis2(e) => process_events_cis2(conn, now, contract, e)?,
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use concordium_cis2::{TokenAmountU64, TokenAmountU8};
    use num_traits::FromPrimitive;
    use rust_decimal::Decimal;

    use crate::processors::cis2_utils::TokenAmountToDecimal;

    #[test]
    fn token_amount_conversions() {
        let amount = TokenAmountU8(0).to_decimal();
        assert_eq!(amount, Decimal::ZERO);
        assert_eq!(amount.to_string(), "0");

        let amount: u8 = 255;
        let token_amount = TokenAmountU8(amount).to_decimal();
        assert_eq!(token_amount, Decimal::from_u8(amount).unwrap());
        assert_eq!(token_amount.to_string(), "255");

        let amount: u64 = 255;
        let token_amount = TokenAmountU64(amount).to_decimal();
        assert_eq!(token_amount, Decimal::from_u64(amount).unwrap());
        assert_eq!(token_amount.to_string(), "255");
    }
}
