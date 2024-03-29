use concordium_cis2::{TokenAmountU64, TokenIdUnit};
use concordium_std::{ContractAddress, OwnedContractName};
use euroe_stablecoin::{MintParams, RoleTypes};
use super::{
    cis2_test_contract::{ICis2Contract, ICis2ContractUnitTokenExt},
    test_contract_client::*
};

pub const CONTRACT_NAME: &str = "init_euroe_stablecoin";

pub trait IEuroeModule: ITestModule {
    fn euroe(&self) -> GenericInit<(), concordium_cis2::Cis2Event<TokenIdUnit, TokenAmountU64>> {
        GenericInit::<(), concordium_cis2::Cis2Event<TokenIdUnit, TokenAmountU64>>::new(
            self.module_ref(),
            CONTRACT_NAME,
        )
    }
}

pub trait IEuroeContract:
    ICis2Contract<TokenIdUnit, TokenAmountU64, concordium_cis2::Cis2Event<TokenIdUnit, TokenAmountU64>>
{
    fn grant_role(&self) -> GenericReceive<RoleTypes, (), concordium_cis2::Cis2Event<TokenIdUnit, TokenAmountU64>> {
        GenericReceive::<RoleTypes, (),  concordium_cis2::Cis2Event<TokenIdUnit, TokenAmountU64>>::new(
            self.contract_address(), 
            Self::contract_name(), 
            "grantRole", self.max_energy())
    }

    fn mint(
        &self,
    ) -> GenericReceive<MintParams, (), concordium_cis2::Cis2Event<TokenIdUnit, TokenAmountU64>>
    {
        GenericReceive::<MintParams, (), concordium_cis2::Cis2Event<TokenIdUnit, TokenAmountU64>>::new(self.contract_address(), Self::contract_name(), "mint", self.max_energy())
    }
}

pub struct EuroeModule {
    pub module_path: String,
}

impl ITestModule for EuroeModule {
    fn module_path(&self) -> String { self.module_path.to_owned() }
}

impl IEuroeModule for EuroeModule {}

pub struct EuroeContract(pub ContractAddress);

impl ITestContract for EuroeContract {
    fn contract_name() -> OwnedContractName {
        OwnedContractName::new_unchecked(CONTRACT_NAME.to_owned())
    }

    fn contract_address(&self) -> ContractAddress { self.0 }
}

impl
    ICis2Contract<
        TokenIdUnit,
        TokenAmountU64,
        concordium_cis2::Cis2Event<TokenIdUnit, TokenAmountU64>,
    > for EuroeContract
{
}

impl
    ICis2ContractUnitTokenExt<
        TokenAmountU64,
        concordium_cis2::Cis2Event<TokenIdUnit, TokenAmountU64>,
    > for EuroeContract
{
}

impl IEuroeContract for EuroeContract {}
