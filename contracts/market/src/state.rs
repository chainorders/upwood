use std::ops::Sub;

use super::types::{ExchangeRate, *};
use concordium_std::{
    ops::{AddAssign, SubAssign},
    *,
};

#[derive(Serialize)]
pub struct DepositedTokenInfo {
    balance: Cis2TokenAmount,
}

impl DepositedTokenInfo {
    pub fn new(balance: Cis2TokenAmount) -> Self {
        Self {
            balance,
        }
    }

    pub fn add_balance(&mut self, amount: Cis2TokenAmount) { self.balance.add_assign(amount); }

    pub fn decrease_balance(&mut self, amount: &Cis2TokenAmount) -> Cis2TokenAmount {
        self.balance.sub_assign(*amount);
        self.balance
    }
}

#[derive(Serialize)]
pub struct TokenOwnerUId {
    pub token_id: TokenUId,
    pub owner:    AccountAddress,
}

impl TokenOwnerUId {
    pub fn new(token_id: TokenUId, owner: AccountAddress) -> Self {
        Self {
            token_id,
            owner,
        }
    }
}

#[derive(Serial, DeserialWithState, Deletable)]
#[concordium(state_parameter = "S")]
pub struct ListedTokenInfo<S> {
    supply:         Cis2TokenAmount,
    exchange_rates: StateSet<ExchangeRate, S>,
}

impl ListedTokenInfo<StateApi> {
    pub fn new(supply: Cis2TokenAmount, state_builder: &mut StateBuilder) -> Self {
        Self {
            supply,
            exchange_rates: state_builder.new_set(),
        }
    }

    pub fn add_exchange_rate(&mut self, rate: ExchangeRate) { self.exchange_rates.insert(rate); }

    pub fn decrease_supply(&mut self, amount: &Cis2TokenAmount) -> Cis2TokenAmount {
        self.supply.sub_assign(*amount);
        self.supply
    }
}

#[derive(Serialize, SchemaType)]
pub struct ListedToken {
    pub token_id:       TokenUId,
    pub owner:          AccountAddress,
    pub exchange_rates: Vec<ExchangeRate>,
    pub supply:         Cis2TokenAmount,
}

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
/// Represents the state of the market contract.
pub struct State<S = StateApi> {
    /// Tokens which can be received by the market contract and kept in custody
    /// of the contract / deposited with the contract.
    deposited_tokens:     StateMap<TokenOwnerUId, DepositedTokenInfo, S>,
    /// Tokens which are listed for sale.
    listed_tokens:        StateMap<TokenOwnerUId, ListedTokenInfo<S>, S>,
    commission:           Rate,
    /// Tokens which can be used to pay for the commission & listed tokens.
    payment_tokens:       StateSet<TokenUId, S>,
    /// Contracts whose tokens can be exchanged for payment tokens.
    sell_token_contracts: StateSet<ContractAddress, S>,
}

impl State<StateApi> {
    pub fn new(
        commission: Rate,
        payment_tokens: Vec<TokenUId>,
        sell_token_contracts: Vec<ContractAddress>,
        state_builder: &mut StateBuilder,
    ) -> Self {
        let mut state = Self {
            deposited_tokens: state_builder.new_map(),
            listed_tokens: state_builder.new_map(),
            commission,
            payment_tokens: state_builder.new_set(),
            sell_token_contracts: state_builder.new_set(),
        };

        for token in payment_tokens {
            state.payment_tokens.insert(token);
        }

        for contract in sell_token_contracts {
            state.sell_token_contracts.insert(contract);
        }

        state
    }

    pub fn commission(&self) -> Rate { self.commission.to_owned() }

    pub fn sell_token_contracts(&self) -> Vec<ContractAddress> {
        self.sell_token_contracts.iter().map(|r| r.to_owned()).collect()
    }

    pub fn add_sell_token_contract(&mut self, contract: ContractAddress) {
        self.sell_token_contracts.insert(contract);
    }

    pub fn payment_tokens(&self) -> Vec<TokenUId> {
        self.payment_tokens.iter().map(|r| r.to_owned()).collect()
    }

    pub fn add_payment_token(&mut self, token: TokenUId) { self.payment_tokens.insert(token); }

    pub fn can_be_paid_by(&self, token_uid: &TokenUId) -> bool {
        self.payment_tokens.contains(token_uid)
    }

    pub fn can_list(&self, token_uid: &TokenUId) -> bool {
        self.sell_token_contracts.contains(&token_uid.contract)
    }

    pub fn add_or_increase_deposits(
        &mut self,
        token_uid: TokenUId,
        amount: Cis2TokenAmount,
        owner: AccountAddress,
    ) {
        self.deposited_tokens
            .entry(TokenOwnerUId::new(token_uid, owner))
            .and_modify(|r| r.add_balance(amount))
            .or_insert_with(|| DepositedTokenInfo::new(amount));
    }

    pub fn decrease_deposit(&mut self, id: &TokenOwnerUId, amount: &Cis2TokenAmount) {
        let remaining = {
            let entry = self.deposited_tokens.get_mut(id);
            match entry {
                Some(mut entry) => entry.decrease_balance(amount),
                None => return,
            }
        };

        if remaining.0.eq(&0u64) {
            self.deposited_tokens.remove(id)
        }
    }

    pub fn deposited_amount(
        &self,
        token_uid: &TokenUId,
        address: &AccountAddress,
    ) -> Cis2TokenAmount {
        self.deposited_tokens
            .get(&TokenOwnerUId::new(token_uid.to_owned(), *address))
            .map(|r| r.balance.to_owned())
            .unwrap_or(0.into())
    }

    pub fn unlisted_amount(
        &self,
        token_uid: &TokenUId,
        address: &AccountAddress,
    ) -> Cis2TokenAmount {
        self.deposited_amount(token_uid, address)
            .sub(self.listed_amount(&TokenOwnerUId::new(token_uid.to_owned(), *address)))
    }

    pub fn add_or_replace_listed(
        &mut self,
        token_uid: TokenUId,
        owner: AccountAddress,
        supply: Cis2TokenAmount,
        exchange_rates: Vec<ExchangeRate>,
        state_builder: &mut StateBuilder,
    ) {
        let mut listed_token_info = ListedTokenInfo::new(supply, state_builder);
        for rate in exchange_rates {
            listed_token_info.add_exchange_rate(rate);
        }
        self.listed_tokens.insert(
            TokenOwnerUId {
                token_id: token_uid,
                owner,
            },
            listed_token_info,
        );
    }

    pub fn remove_listed(&mut self, token_uid: TokenUId, owner: AccountAddress) {
        self.listed_tokens.remove(&TokenOwnerUId {
            token_id: token_uid,
            owner,
        });
    }

    pub fn listed_amount(&self, id: &TokenOwnerUId) -> Cis2TokenAmount {
        self.listed_tokens.get(id).map(|t| t.supply).unwrap_or(0.into())
    }

    /// Decreases the supply of the listed token by the given amount.
    /// Returns true if the token was removed from the listed tokens.
    pub fn decrease_listed_amount(&mut self, id: &TokenOwnerUId, amount: &Cis2TokenAmount) -> bool {
        let remaining = {
            let entry = self.listed_tokens.get_mut(id);
            match entry {
                Some(mut entry) => entry.decrease_supply(amount),
                None => return false,
            }
        };

        if remaining.0.eq(&0u64) {
            self.listed_tokens.remove(id);
            true
        } else {
            false
        }
    }

    pub fn is_listed(&self, token_uid: &TokenUId, owner: &AccountAddress) -> bool {
        self.listed_tokens
            .get(&TokenOwnerUId::new(token_uid.to_owned(), owner.to_owned()))
            .is_some()
    }

    pub fn get_listed(&self, token_uid: &TokenUId, owner: &AccountAddress) -> Option<ListedToken> {
        self.listed_tokens
            .get(&TokenOwnerUId {
                token_id: token_uid.to_owned(),
                owner:    owner.to_owned(),
            })
            .map(|r| ListedToken {
                token_id:       token_uid.to_owned(),
                owner:          owner.to_owned(),
                exchange_rates: r.exchange_rates.iter().map(|r| r.to_owned()).collect(),
                supply:         r.supply.to_owned(),
            })
    }
}
