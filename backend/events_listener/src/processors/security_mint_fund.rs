use chrono::NaiveDateTime;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use diesel::Connection;
use rust_decimal::Decimal;
use security_mint_fund::types::{
    AgentRole, Event, FundAddedEvent, InvestedEvent, UpdateFundState, UpdateFundStateParams,
};
use shared::db::cis2_security::Agent;
use shared::db::security_mint_fund::{
    InvestmentRecord, InvestmentRecordType, Investor, SecurityMintFund, SecurityMintFundContract,
    SecurityMintFundState,
};
use shared::db_shared::DbConn;
use tracing::{info, instrument, warn};
use uuid::Uuid;

use crate::processors::cis2_utils::{
    ContractAddressToDecimal, RateToDecimal, TokenAmountToDecimal, TokenIdToDecimal,
};
use crate::processors::ProcessorError;

pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../contracts/security-mint-fund/contract.wasm.v1"
    ))
    .expect("Failed to parse security-mint-fund module")
    .get_module_ref()
}

pub fn contract_name() -> OwnedContractName {
    OwnedContractName::new_unchecked("init_security_mint_fund".to_string())
}

#[allow(clippy::too_many_arguments)]
#[instrument(
    name="mint_fund",
    skip_all,
    fields(contract = %contract, events = events.len())
)]
pub fn process_events(
    conn: &mut DbConn,
    block_height: Decimal,
    block_time: NaiveDateTime,
    txn_index: Decimal,
    _txn_sender: &str,
    _txn_instigator: &str,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError> {
    for event in events {
        let event = event.parse::<Event>().expect("Failed to parse event");

        match event {
            Event::Initialized(event) => {
                SecurityMintFundContract {
                    contract_address:                contract.to_decimal(),
                    currency_token_id:               event.id.to_decimal(),
                    currency_token_contract_address: event.contract.to_decimal(),
                    create_time:                     block_time,
                }
                .insert(conn)?;
                info!("Initialized mint fund contract: {}", contract.to_decimal());
            }
            Event::AgentAdded(agent) => {
                let roles = agent
                    .roles
                    .iter()
                    .map(|r| Some(role_to_string(r)))
                    .collect();
                let agent =
                    match Agent::find(conn, contract.to_decimal(), &agent.address.to_string())? {
                        Some(agent) => Agent { roles, ..agent }.update(conn)?,
                        None => Agent {
                            cis2_address: contract.to_decimal(),
                            agent_address: agent.address.to_string(),
                            roles,
                        }
                        .insert(conn)?,
                    };

                info!(
                    "Agent added: {}, roles: {}",
                    agent.agent_address,
                    agent
                        .roles
                        .into_iter()
                        .flatten()
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }
            Event::AgentRemoved(agent) => {
                Agent::find(conn, contract.to_decimal(), &agent.to_string())?
                    .ok_or(ProcessorError::Cis2AgentNotFound {
                        contract: contract.to_decimal(),
                        agent:    agent.to_string(),
                    })?
                    .delete(conn)?;
                info!("Agent removed: {}", agent.to_string());
            }
            Event::FundAdded(FundAddedEvent {
                rate,
                security_token,
                token,
            }) => {
                conn.transaction::<_, ProcessorError, _>(|conn| {
                    let contract = SecurityMintFundContract::find(conn, contract.to_decimal())?
                        .ok_or(ProcessorError::SecurityMintFundContractNotFound {
                            contract: contract.to_decimal(),
                        })?;
                    SecurityMintFund {
                        contract_address: contract.contract_address,
                        token_id: token.id.to_decimal(),
                        token_contract_address: token.contract.to_decimal(),
                        investment_token_id: security_token.id.to_decimal(),
                        investment_token_contract_address: security_token.contract.to_decimal(),
                        currency_token_id: contract.currency_token_id,
                        currency_token_contract_address: contract.currency_token_contract_address,
                        currency_amount: Decimal::ZERO,
                        token_amount: Decimal::ZERO,
                        receiver_address: None,
                        rate_numerator: rate.numerator.into(),
                        rate_denominator: rate.denominator.into(),
                        fund_state: SecurityMintFundState::Open,
                        create_time: block_time,
                        update_time: block_time,
                    }
                    .insert(conn)?;
                    Ok(())
                })?;

                info!(
                    "Fund added: {}/{}, rate: {}, security token: {}/{}, contract: {}",
                    token.id.to_decimal(),
                    token.contract.to_decimal(),
                    rate.to_decimal(),
                    security_token.id.to_decimal(),
                    security_token.contract.to_decimal(),
                    contract.to_decimal()
                );
            }
            Event::FundRemoved(security_token) => {
                SecurityMintFund::delete(
                    conn,
                    contract.to_decimal(),
                    security_token.id.to_decimal(),
                    security_token.contract.to_decimal(),
                )?;
                info!(
                    "Fund removed: {}/{}, contract: {}",
                    security_token.id.to_decimal(),
                    security_token.contract.to_decimal(),
                    contract.to_decimal()
                );
            }
            Event::FundStateUpdated(UpdateFundStateParams {
                security_token,
                state: fund_state,
            }) => {
                conn.transaction::<_, ProcessorError, _>(|conn| {
                    let fund = SecurityMintFund::find(
                        conn,
                        contract.to_decimal(),
                        security_token.id.to_decimal(),
                        security_token.contract.to_decimal(),
                    )?
                    .ok_or(ProcessorError::FundNotFound {
                        security_token_id: security_token.id.to_decimal(),
                        security_token_contract_address: security_token.contract.to_decimal(),
                        contract: contract.to_decimal(),
                    })?;
                    let (fund_state, receiver_address) = to_db_fund_state(&fund_state);
                    SecurityMintFund {
                        fund_state,
                        receiver_address,
                        update_time: block_time,
                        ..fund
                    }
                    .update(conn)?;
                    Ok(())
                })?;

                info!(
                    "Fund state updated: {}/{}, state: {:?}, contract: {}",
                    security_token.id.to_decimal(),
                    security_token.contract.to_decimal(),
                    fund_state,
                    contract.to_decimal()
                );
            }
            Event::Invested(InvestedEvent {
                currency_amount,
                security_amount,
                security_token,
                investor,
            }) => {
                conn.transaction::<_, ProcessorError, _>(|conn| {
                    let contract = SecurityMintFundContract::find(conn, contract.to_decimal())?
                        .ok_or(ProcessorError::SecurityMintFundContractNotFound {
                            contract: contract.to_decimal(),
                        })?;
                    let investor = Investor::find(
                        conn,
                        contract.contract_address,
                        security_token.id.to_decimal(),
                        security_token.contract.to_decimal(),
                        &investor.to_string(),
                    )?
                    .map(|investor| Investor {
                        currency_amount: investor.currency_amount + currency_amount.to_decimal(),
                        token_amount: investor.token_amount + security_amount.to_decimal(),
                        currency_amount_total: investor.currency_amount_total
                            + currency_amount.to_decimal(),
                        update_time: block_time,
                        ..investor
                    })
                    .unwrap_or_else(|| Investor {
                        contract_address: contract.contract_address,
                        investment_token_id: security_token.id.to_decimal(),
                        investment_token_contract_address: security_token.contract.to_decimal(),
                        investor: investor.to_string(),
                        currency_amount: currency_amount.to_decimal(),
                        token_amount: security_amount.to_decimal(),
                        currency_amount_total: currency_amount.to_decimal(),
                        currency_token_id: contract.currency_token_id,
                        currency_token_contract_address: contract.currency_token_contract_address,
                        token_amount_total: 0.into(),
                        create_time: block_time,
                        update_time: block_time,
                    })
                    .upsert(conn)?;

                    InvestmentRecord {
                        id: Uuid::new_v4(),
                        block_height,
                        txn_index,
                        contract_address: contract.contract_address,
                        investment_token_id: security_token.id.to_decimal(),
                        investment_token_contract_address: security_token.contract.to_decimal(),
                        currency_token_id: contract.currency_token_id,
                        currency_token_contract_address: contract.currency_token_contract_address,
                        investor: investor.investor.to_string(),
                        currency_amount: currency_amount.to_decimal(),
                        token_amount: security_amount.to_decimal(),
                        currency_amount_balance: investor.currency_amount,
                        token_amount_balance: investor.token_amount,
                        investment_record_type: InvestmentRecordType::Invested,
                        create_time: block_time,
                    }
                    .insert(conn)?;

                    SecurityMintFund::find(
                        conn,
                        contract.contract_address,
                        security_token.id.to_decimal(),
                        security_token.contract.to_decimal(),
                    )?
                    .map(|mut fund| {
                        fund.currency_amount += currency_amount.to_decimal();
                        fund.token_amount += security_amount.to_decimal();
                        fund.update_time = block_time;
                        fund
                    })
                    .ok_or(ProcessorError::FundNotFound {
                        contract: contract.contract_address,
                        security_token_id: security_token.id.to_decimal(),
                        security_token_contract_address: security_token.contract.to_decimal(),
                    })?
                    .update(conn)?;

                    Ok(())
                })?;

                info!(
                    "Investment received: fund: {}/{}, from: {}, currency amount: {}, token \
                     amount: {}",
                    security_token.id.to_decimal(),
                    security_token.contract.to_decimal(),
                    investor.to_string(),
                    currency_amount.to_decimal(),
                    security_amount.to_decimal()
                );
            }
            Event::InvestmentCancelled(InvestedEvent {
                currency_amount,
                security_amount,
                security_token,
                investor,
            }) => {
                let currency_amount = currency_amount.to_decimal();
                let security_amount = security_amount.to_decimal();
                let security_token_id = security_token.id.to_decimal();
                let security_token_contract_address = security_token.contract.to_decimal();

                conn.transaction::<_, ProcessorError, _>(|conn| {
                    let contract = SecurityMintFundContract::find(conn, contract.to_decimal())?
                        .ok_or(ProcessorError::SecurityMintFundContractNotFound {
                            contract: contract.to_decimal(),
                        })?;

                    let investor = Investor::find(
                        conn,
                        contract.contract_address,
                        security_token_id,
                        security_token_contract_address,
                        &investor.to_string(),
                    )?
                    .map(|investor| Investor {
                        currency_amount: investor.currency_amount - currency_amount,
                        token_amount: investor.token_amount - security_amount,
                        currency_amount_total: investor.currency_amount_total - currency_amount,
                        update_time: block_time,
                        ..investor
                    })
                    .ok_or(ProcessorError::InvestorNotFound {
                        investor: investor.to_string(),
                        contract: contract.contract_address,
                    })?
                    .update(conn)?;

                    let _ = SecurityMintFund::find(
                        conn,
                        contract.contract_address,
                        security_token_id,
                        security_token_contract_address,
                    )?
                    .map(|fund| SecurityMintFund {
                        currency_amount: fund.currency_amount - currency_amount,
                        token_amount: fund.token_amount - security_amount,
                        update_time: block_time,
                        ..fund
                    })
                    .ok_or(ProcessorError::FundNotFound {
                        security_token_id,
                        security_token_contract_address,
                        contract: contract.contract_address,
                    })?
                    .update(conn)?;

                    InvestmentRecord {
                        id: Uuid::new_v4(),
                        block_height,
                        txn_index,
                        contract_address: contract.contract_address,
                        investor: investor.investor.to_string(),
                        investment_token_id: security_token_id,
                        investment_token_contract_address: security_token_contract_address,
                        currency_token_id: contract.currency_token_id,
                        currency_token_contract_address: contract.currency_token_contract_address,
                        currency_amount,
                        token_amount: security_amount,
                        currency_amount_balance: investor.currency_amount,
                        token_amount_balance: investor.token_amount,
                        investment_record_type: InvestmentRecordType::Cancelled,
                        create_time: block_time,
                    }
                    .insert(conn)?;

                    Ok(())
                })?;

                info!(
                    "Investment cancelled: fund: {}/{}, from: {}, currency amount: {}, token \
                     amount: {}",
                    security_token_id,
                    security_token_contract_address,
                    investor.to_string(),
                    currency_amount,
                    security_amount
                );
            }
            Event::InvestmentClaimed(InvestedEvent {
                currency_amount,
                security_amount,
                security_token,
                investor,
            }) => {
                let currency_amount = currency_amount.to_decimal();
                let security_amount = security_amount.to_decimal();
                let security_token_id = security_token.id.to_decimal();
                let security_token_contract_address = security_token.contract.to_decimal();

                conn.transaction::<_, ProcessorError, _>(|conn| {
                    let contract = SecurityMintFundContract::find(conn, contract.to_decimal())?
                        .ok_or(ProcessorError::SecurityMintFundContractNotFound {
                            contract: contract.to_decimal(),
                        })?;

                    let investor = Investor::find(
                        conn,
                        contract.contract_address,
                        security_token_id,
                        security_token_contract_address,
                        &investor.to_string(),
                    )?
                    .map(|investor| Investor {
                        currency_amount: investor.currency_amount - currency_amount,
                        token_amount: investor.token_amount - security_amount,
                        token_amount_total: investor.token_amount_total + security_amount,
                        update_time: block_time,
                        ..investor
                    })
                    .ok_or(ProcessorError::InvestorNotFound {
                        investor: investor.to_string(),
                        contract: contract.contract_address,
                    })?
                    .update(conn)?;

                    SecurityMintFund::find(
                        conn,
                        contract.contract_address,
                        security_token_id,
                        security_token_contract_address,
                    )?
                    .map(|fund| SecurityMintFund {
                        currency_amount: fund.currency_amount - currency_amount,
                        token_amount: fund.token_amount - security_amount,
                        update_time: block_time,
                        ..fund
                    })
                    .ok_or(ProcessorError::FundNotFound {
                        security_token_id,
                        security_token_contract_address,
                        contract: contract.contract_address,
                    })?
                    .update(conn)?;

                    InvestmentRecord {
                        id: Uuid::new_v4(),
                        block_height,
                        txn_index,
                        contract_address: contract.contract_address,
                        investment_token_id: security_token_id,
                        investment_token_contract_address: security_token_contract_address,
                        currency_token_id: contract.currency_token_id,
                        currency_token_contract_address: contract.currency_token_contract_address,
                        investor: investor.investor.to_string(),
                        currency_amount,
                        token_amount: security_amount,
                        currency_amount_balance: investor.currency_amount,
                        token_amount_balance: investor.token_amount,
                        investment_record_type: InvestmentRecordType::Claimed,
                        create_time: block_time,
                    }
                    .insert(conn)?;

                    Ok(())
                })?;

                info!(
                    "Investment claimed: fund: {}/{}, from: {}, currency amount: {}, token \
                     amount: {}",
                    security_token_id,
                    security_token_contract_address,
                    investor.to_string(),
                    currency_amount,
                    security_amount
                );
            }
        }
    }

    Ok(())
}

fn role_to_string(r: &AgentRole) -> String {
    match r {
        AgentRole::AddFund => "AddFund".to_string(),
        AgentRole::RemoveFund => "RemoveFund".to_string(),
        AgentRole::UpdateFundState => "UpdateFundState".to_string(),
        AgentRole::Operator => "Operator".to_string(),
    }
}

fn to_db_fund_state(fund_state: &UpdateFundState) -> (SecurityMintFundState, Option<String>) {
    match fund_state {
        UpdateFundState::Success(receiver) => (
            SecurityMintFundState::Success,
            Some(receiver.address().to_string()),
        ),
        UpdateFundState::Fail => (SecurityMintFundState::Fail, None),
    }
}
