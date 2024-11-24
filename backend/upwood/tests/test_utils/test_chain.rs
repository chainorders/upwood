use chrono::{DateTime, Utc};
use concordium_rust_sdk::base::smart_contracts::WasmModule;
use concordium_smart_contract_testing::{
    AccountAddress, Amount, ContractAddress, ContractInitError, ContractInvokeError,
    ContractInvokeSuccess, Duration, Energy, InitContractPayload, ModuleDeployError,
    ModuleDeploySuccess, Signer, UpdateContractPayload,
};
use events_listener::listener::{ParsedBlock, ParsedTxn};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use sha2::Sha256;
use shared::db::txn_listener::ListenerBlock;

use super::conversions::{to_contract_call_init, to_contract_call_update};

pub struct Chain {
    pub chain:                concordium_smart_contract_testing::Chain,
    pub pending_transactions: Vec<ParsedTxn>,
    pub total_txn_count:      u32,
    pub block_count:          u32,
}

impl Chain {
    const MAX_ENERGY: Energy = Energy { energy: 20_000 };

    pub fn new(now: DateTime<Utc>) -> Self {
        let chain = concordium_smart_contract_testing::Chain::new_with_time(
            now.timestamp_millis().to_u64().unwrap().into(),
        );

        Self {
            chain,
            pending_transactions: Vec::new(),
            total_txn_count: 0,
            block_count: 0,
        }
    }

    pub fn create_account(
        &mut self,
        address: concordium_smart_contract_testing::AccountAddress,
        balance: Amount,
    ) -> Account {
        let account = concordium_smart_contract_testing::Account::new(address, balance);
        let existing = self.chain.create_account(account.clone());
        assert!(existing.is_none());
        Account(account)
    }

    pub fn deploy_module(
        &mut self,
        sender: AccountAddress,
        module: WasmModule,
    ) -> Result<ModuleDeploySuccess, ModuleDeployError> {
        self.chain
            .module_deploy_v1(Signer::with_one_key(), sender, module)
    }

    pub fn init(
        &mut self,
        sender: AccountAddress,
        payload: InitContractPayload,
    ) -> Result<ContractAddress, ContractInitError> {
        let res = self.chain.contract_init(
            Signer::with_one_key(),
            sender,
            Self::MAX_ENERGY,
            payload.clone(),
        )?;
        self.insert_transaction(ParsedTxn {
            hash:           self.generate_random_hash(self.total_txn_count),
            index:          self.pending_transactions.len() as u64,
            sender:         sender.to_string(),
            contract_calls: vec![to_contract_call_init(
                &res,
                payload.mod_ref,
                payload.init_name,
            )],
        });
        Ok(res.contract_address)
    }

    pub fn update(
        &mut self,
        sender: AccountAddress,
        payload: UpdateContractPayload,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        self.update_with_energy(sender, payload, Self::MAX_ENERGY)
    }

    pub fn update_with_energy(
        &mut self,
        sender: AccountAddress,
        payload: UpdateContractPayload,
        energy: Energy,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        let res = self.chain.contract_update(
            Signer::with_one_key(),
            sender,
            sender.into(),
            energy,
            payload.clone(),
        )?;
        self.insert_transaction(ParsedTxn {
            hash:           self.generate_random_hash(self.total_txn_count),
            index:          self.pending_transactions.len() as u64,
            sender:         sender.to_string(),
            contract_calls: to_contract_call_update(&res),
        });
        Ok(res)
    }

    fn generate_random_hash(&self, seed: u32) -> Vec<u8> {
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(seed.to_be_bytes());
        let hash = hasher.finalize();
        hash.into_iter().collect()
    }

    pub fn insert_transaction(&mut self, txn: ParsedTxn) {
        self.pending_transactions.push(txn);
        self.total_txn_count += 1;
    }

    pub fn produce_block(&mut self) -> ParsedBlock {
        let block = ParsedBlock {
            block:        ListenerBlock {
                block_height:    Decimal::from_u32(self.block_count).unwrap(),
                block_hash:      self.generate_random_hash(self.block_count),
                block_slot_time: DateTime::from_timestamp_millis(
                    self.chain.block_time().millis.to_i64().unwrap(),
                )
                .unwrap()
                .naive_utc(),
            },
            transactions: self.pending_transactions.clone(),
        };
        self.pending_transactions = Vec::new();
        self.block_count += 1;
        self.tick_block_time(Duration::from_seconds(2));
        block
    }

    pub fn block_time_naive_utc(&self) -> chrono::NaiveDateTime {
        DateTime::from_timestamp_millis(self.chain.block_time().millis.to_i64().unwrap())
            .unwrap()
            .naive_utc()
    }

    pub fn tick_block_time(&mut self, duration: Duration) {
        self.chain
            .tick_block_time(duration)
            .expect("tick block time");
    }
}

#[derive(Clone)]
pub struct Account(pub concordium_smart_contract_testing::Account);

impl Account {
    pub fn address(&self) -> AccountAddress { self.0.address }

    pub fn address_str(&self) -> String { self.0.address.to_string() }

    pub fn transact<F, T>(&self, f: F) -> T
    where F: FnOnce(AccountAddress) -> T {
        f(self.0.address)
    }
}
