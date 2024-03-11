use super::test_contract_client::*;
use concordium_rwa_market::{
    deposit::DepositParams,
    event::Event,
    exchange::{Amounts, ExchangeParams},
    init::InitParams,
    list::GetListedParam,
    types::Cis2TokenAmount,
};
use concordium_smart_contract_testing::*;

pub const CONTRACT_NAME: &str = "init_rwa_market";

pub trait IMarketModule: ITestModule {
    fn rwa_market(&self) -> GenericInit<InitParams, Event> {
        GenericInit::<InitParams, Event>::new(self.module_ref(), CONTRACT_NAME)
    }
}

pub trait IMarketContract: ITestContract {
    fn deposit(&self) -> GenericReceive<DepositParams, (), Event> {
        GenericReceive::<DepositParams, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "deposit",
            self.max_energy(),
        )
    }

    fn balance_of_listed(
        &self,
    ) -> GenericReceive<GetListedParam, concordium_cis2::TokenAmountU64, Event> {
        GenericReceive::<GetListedParam, Cis2TokenAmount, Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "balanceOfListed",
            self.max_energy(),
        )
    }

    fn calculate_amounts(&self) -> GenericReceive<ExchangeParams, Amounts, Event> {
        GenericReceive::<ExchangeParams, Amounts, Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "calculateAmounts",
            self.max_energy(),
        )
    }
}

pub struct MarketContract(pub ContractAddress);

impl ITestContract for MarketContract {
    fn contract_name() -> OwnedContractName {
        OwnedContractName::new_unchecked(CONTRACT_NAME.to_owned())
    }

    fn contract_address(&self) -> ContractAddress { self.0 }
}

impl IMarketContract for MarketContract {}

pub struct MarketModule {
    pub module_path: String,
}

impl ITestModule for MarketModule {
    fn module_path(&self) -> String { self.module_path.to_owned() }
}

impl IMarketModule for MarketModule {}
