use poem_openapi::Object;
use rust_decimal::Decimal;

use super::cis2_security::{holders_count_by_token, Agent, Token};
use super::txn_listener::ListenerContract;
use crate::db_shared::{DbConn, DbResult};

#[derive(serde::Serialize, Object)]
pub struct TokenDetails {
    pub supply:          Decimal,
    pub holder_count:    u64,
    pub token_id:        String,
    pub contract_agents: Vec<Agent>,
    pub contract:        ListenerContract,
}

impl TokenDetails {
    pub fn find(contract: Decimal, db_conn: &mut DbConn) -> DbResult<Option<TokenDetails>> {
        let token_id = Decimal::ZERO;
        let contract = ListenerContract::find(db_conn, contract)?;
        let contract = match contract {
            Some(contract_details) => contract_details,
            None => return Ok(None),
        };
        let token_details = Token::find(db_conn, contract.contract_address, token_id)?;
        let token_details = match token_details {
            None => return Ok(None),
            Some(td) => td,
        };
        let (contract_agents, _) = Agent::list(db_conn, contract.contract_address, i64::MAX, 0)?;
        let holder_count = holders_count_by_token(db_conn, contract.contract_address, token_id)?;

        Ok(Some(TokenDetails {
            supply: token_details.supply,
            holder_count: holder_count as u64,
            token_id: token_id.to_string(),
            contract_agents,
            contract,
        }))
    }
}
