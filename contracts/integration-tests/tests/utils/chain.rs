use concordium_smart_contract_testing::*;

use super::{
    compliance::compliance_deploy_and_init, identity_registry::identity_registry_deploy_and_init,
    security_nft::security_nft_deploy_and_init, security_sft::security_sft_deploy_and_init,
};

pub fn create_accounts(chain: &mut Chain, accounts: Vec<AccountAddress>, amount: Amount) {
    for account in accounts {
        chain.create_account(Account::new_with_balance(
            account,
            AccountBalance::new(amount, Amount::zero(), Amount::zero()).unwrap(),
        ));
    }
}

pub fn init_identity_contracts(
    chain: &mut Chain,
    admin: AccountAddress,
    compliant_nationalities: Vec<String>,
) -> (ContractAddress, ContractAddress, ContractAddress) {
    let ir_contract = identity_registry_deploy_and_init(chain, admin);
    let (compliance_module_contract, compliance_contract) =
        compliance_deploy_and_init(chain, ir_contract, admin, compliant_nationalities);

    (ir_contract, compliance_module_contract, compliance_contract)
}

pub fn init_security_token_contracts(
    chain: &mut Chain,
    admin: AccountAddress,
    identity_registry: ContractAddress,
    compliance: ContractAddress,
    sponsors: Vec<ContractAddress>,
) -> (ContractAddress, ContractAddress) {
    let security_nft_contract =
        security_nft_deploy_and_init(chain, admin, compliance, identity_registry, sponsors.clone());
    let security_sft_contract =
        security_sft_deploy_and_init(chain, admin, compliance, identity_registry, sponsors);

    (security_nft_contract, security_sft_contract)
}
