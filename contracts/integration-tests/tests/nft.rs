pub mod utils;

use concordium_smart_contract_testing::*;
use concordium_std::AccountAddress;
use integration_tests::{
    security_nft::{ISecurityNftModule, SecurityNftModule},
    test_contract_client::{ITestContract, ITestModule},
};
use utils::{chain::create_accounts, common::init_identity_contracts, consts::SECURITY_NFT_MODULE};

const ADMIN: AccountAddress = AccountAddress([0; 32]);
const ACCOUNT_BALANCE: u64 = 1000;

#[test]
fn init() {
    let mut chain = Chain::new();
    let accounts = create_accounts(
        &mut chain,
        vec![ADMIN],
        Amount::from_ccd(ACCOUNT_BALANCE),
    );
    let admin = accounts.first().expect("Creating accounts");
    let (identity_registry, compliance_contract) =
        init_identity_contracts(&mut chain, admin, vec!["IN".to_owned(), "US".to_owned()]);

    let nft_module = SecurityNftModule {
        module_path: SECURITY_NFT_MODULE.to_owned(),
    };
    nft_module.deploy(&mut chain, admin).expect("Deploying security nft module");
    let _nft_contract = nft_module
        .rwa_security_nft()
        .init(&mut chain, admin, &concordium_rwa_security_nft::init::InitParam {
            identity_registry: identity_registry.contract_address(),
            compliance:        compliance_contract.contract_address(),
            sponsors:          vec![],
        })
        .expect("Initializing security nft module");
}
