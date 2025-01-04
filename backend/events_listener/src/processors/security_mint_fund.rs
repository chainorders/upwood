use chrono::NaiveDateTime;
use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{ContractEvent, OwnedContractName, WasmModule};
use concordium_rust_sdk::types::ContractAddress;
use rust_decimal::Decimal;
use security_mint_fund::types::{AgentRole, Event, UpdateFundState, UpdateFundStateParams};
use shared::db::cis2_security::Agent;
use shared::db::security_mint_fund::{
    InvestmentRecord, InvestmentRecordType, Investor, SecurityMintFund, SecurityMintFundContract,
    SecurityMintFundState,
};
use shared::db_shared::DbConn;
use tracing::{info, instrument, warn};
use uuid::Uuid;

use crate::processors::cis2_utils::{
    rate_to_decimal, ContractAddressToDecimal, TokenAmountToDecimal, TokenIdToDecimal,
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
                Agent::new(
                    agent.address,
                    block_time,
                    contract.to_decimal(),
                    agent.roles.iter().map(agent_role_to_string).collect(),
                )
                .insert(conn)?;
            }
            Event::AgentRemoved(agent) => {
                Agent::delete(conn, contract.to_decimal(), &agent)?;
            }
            Event::FundAdded(event) => {
                SecurityMintFund {
                    id: event.fund_id.into(),
                    contract_address: contract.to_decimal(),
                    token_id: event.token.id.to_decimal(),
                    token_contract_address: event.token.contract.to_decimal(),
                    investment_token_id: event.security_token.id.to_decimal(),
                    investment_token_contract_address: event.security_token.contract.to_decimal(),
                    currency_amount: Decimal::ZERO,
                    token_amount: Decimal::ZERO,
                    receiver_address: None,
                    rate: rate_to_decimal(event.rate.numerator, event.rate.denominator),
                    fund_state: SecurityMintFundState::Open,
                    create_time: block_time,
                    update_time: block_time,
                }
                .insert(conn)?;
                info!("Fund added: {:?}", event.fund_id);
            }
            Event::FundRemoved(fund_id) => {
                SecurityMintFund::delete(conn, fund_id.into())?;
                info!("Fund removed: {:?}", fund_id);
            }
            Event::FundStateUpdated(UpdateFundStateParams {
                fund_id,
                state: fund_state,
            }) => {
                let fund = SecurityMintFund::find(conn, fund_id.into())?.ok_or(
                    ProcessorError::FundNotFound {
                        fund_id:  fund_id.into(),
                        contract: contract.to_decimal(),
                    },
                )?;
                let (fund_state, receiver_address) = to_db_fund_state(fund_state);
                SecurityMintFund {
                    fund_state,
                    receiver_address,
                    update_time: block_time,
                    ..fund
                }
                .update(conn)?;
                info!("Fund state updated: {:?} to {:?}", fund_id, fund_state);
            }
            Event::Invested(event) => {
                let investor = Investor::find(
                    conn,
                    contract.to_decimal(),
                    event.fund_id.into(),
                    &event.investor.to_string(),
                )?
                .map(|investor| Investor {
                    currency_amount: investor.currency_amount + event.currency_amount.to_decimal(),
                    token_amount: investor.token_amount + event.security_amount.to_decimal(),
                    update_time: block_time,
                    ..investor
                })
                .unwrap_or_else(|| Investor {
                    contract_address: contract.to_decimal(),
                    fund_id:          event.fund_id.into(),
                    investor:         event.investor.to_string(),
                    currency_amount:  event.currency_amount.to_decimal(),
                    token_amount:     event.security_amount.to_decimal(),
                    create_time:      block_time,
                    update_time:      block_time,
                })
                .upsert(conn)?;
                InvestmentRecord {
                    id: Uuid::new_v4(),
                    block_height,
                    txn_index,
                    contract_address: contract.to_decimal(),
                    fund_id: event.fund_id.into(),
                    investor: investor.investor.to_string(),
                    currency_amount: event.currency_amount.to_decimal(),
                    token_amount: event.security_amount.to_decimal(),
                    currency_amount_balance: investor.currency_amount,
                    token_amount_balance: investor.token_amount,
                    investment_record_type: InvestmentRecordType::Invested,
                    create_time: block_time,
                }
                .insert(conn)?;
                let fund = SecurityMintFund::find(conn, event.fund_id.into())?.ok_or(
                    ProcessorError::FundNotFound {
                        fund_id:  event.fund_id.into(),
                        contract: contract.to_decimal(),
                    },
                )?;
                SecurityMintFund {
                    currency_amount: fund.currency_amount + event.currency_amount.to_decimal(),
                    token_amount: fund.token_amount + event.security_amount.to_decimal(),
                    update_time: block_time,
                    ..fund
                }
                .update(conn)?;
                info!(
                    "Investment received: fund: {}, from: {}, currency amount: {}, token amount: \
                     {}",
                    event.fund_id,
                    event.investor.to_string(),
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal()
                );
            }
            Event::InvestmentCancelled(event) => {
                let investor = Investor::find(
                    conn,
                    contract.to_decimal(),
                    event.fund_id.into(),
                    &event.investor.to_string(),
                )?
                .map(|investor| Investor {
                    currency_amount: investor.currency_amount - event.currency_amount.to_decimal(),
                    token_amount: investor.token_amount - event.security_amount.to_decimal(),
                    update_time: block_time,
                    ..investor
                })
                .ok_or(ProcessorError::InvestorNotFound {
                    investor: event.investor.to_string(),
                    contract: contract.to_decimal(),
                })?
                .update(conn)?;
                let _ = SecurityMintFund::find(conn, event.fund_id.into())?
                    .map(|fund| SecurityMintFund {
                        currency_amount: fund.currency_amount - event.currency_amount.to_decimal(),
                        token_amount: fund.token_amount - event.security_amount.to_decimal(),
                        update_time: block_time,
                        ..fund
                    })
                    .ok_or(ProcessorError::FundNotFound {
                        fund_id:  event.fund_id.into(),
                        contract: contract.to_decimal(),
                    })?
                    .update(conn)?;
                InvestmentRecord {
                    id: Uuid::new_v4(),
                    block_height,
                    txn_index,
                    contract_address: contract.to_decimal(),
                    fund_id: event.fund_id.into(),
                    investor: event.investor.to_string(),
                    currency_amount: event.currency_amount.to_decimal(),
                    token_amount: event.security_amount.to_decimal(),
                    currency_amount_balance: investor.currency_amount,
                    token_amount_balance: investor.token_amount,
                    investment_record_type: InvestmentRecordType::Cancelled,
                    create_time: block_time,
                }
                .insert(conn)?;
                info!(
                    "Investment cancelled: fund: {}, from: {}, currency amount: {}, token amount: \
                     {}",
                    event.fund_id,
                    event.investor.to_string(),
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal()
                );
            }
            Event::InvestmentClaimed(event) => {
                let investor = Investor::find(
                    conn,
                    contract.to_decimal(),
                    event.fund_id.into(),
                    &event.investor.to_string(),
                )?
                .map(|investor| Investor {
                    currency_amount: investor.currency_amount - event.currency_amount.to_decimal(),
                    token_amount: investor.token_amount - event.security_amount.to_decimal(),
                    update_time: block_time,
                    ..investor
                })
                .ok_or(ProcessorError::InvestorNotFound {
                    investor: event.investor.to_string(),
                    contract: contract.to_decimal(),
                })?
                .update(conn)?;
                SecurityMintFund::find(conn, event.fund_id.into())?
                    .map(|fund| SecurityMintFund {
                        currency_amount: fund.currency_amount - event.currency_amount.to_decimal(),
                        token_amount: fund.token_amount - event.security_amount.to_decimal(),
                        update_time: block_time,
                        ..fund
                    })
                    .ok_or(ProcessorError::FundNotFound {
                        fund_id:  event.fund_id.into(),
                        contract: contract.to_decimal(),
                    })?
                    .update(conn)?;
                InvestmentRecord {
                    id: Uuid::new_v4(),
                    block_height,
                    txn_index,
                    contract_address: contract.to_decimal(),
                    fund_id: event.fund_id.into(),
                    investor: event.investor.to_string(),
                    currency_amount: event.currency_amount.to_decimal(),
                    token_amount: event.security_amount.to_decimal(),
                    currency_amount_balance: investor.currency_amount,
                    token_amount_balance: investor.token_amount,
                    investment_record_type: InvestmentRecordType::Claimed,
                    create_time: block_time,
                }
                .insert(conn)?;
                info!(
                    "Investment claimed: fund: {}, from: {}, currency amount: {}, token amount: {}",
                    event.fund_id,
                    event.investor.to_string(),
                    event.currency_amount.to_decimal(),
                    event.security_amount.to_decimal()
                );
            }
        }
    }

    Ok(())
}

fn agent_role_to_string(r: &AgentRole) -> String {
    match r {
        AgentRole::AddFund => "AddFund".to_string(),
        AgentRole::RemoveFund => "RemoveFund".to_string(),
        AgentRole::UpdateFundState => "UpdateFundState".to_string(),
        AgentRole::Operator => "Operator".to_string(),
    }
}

fn to_db_fund_state(fund_state: UpdateFundState) -> (SecurityMintFundState, Option<String>) {
    match fund_state {
        UpdateFundState::Success(receiver) => (
            SecurityMintFundState::Success,
            Some(receiver.address().to_string()),
        ),
        UpdateFundState::Fail => (SecurityMintFundState::Fail, None),
    }
}
