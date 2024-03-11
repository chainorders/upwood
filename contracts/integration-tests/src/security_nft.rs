use super::{cis2_test_contract::*, test_contract_client::*};
use concordium_cis2::Receiver;
use concordium_rwa_security_nft::{
    event::Event,
    init::InitParam,
    mint::{MintParam, MintParams},
    types::{ContractMetadataUrl, TokenAmount, TokenId},
};
use concordium_smart_contract_testing::*;

pub const CONTRACT_NAME: &str = "init_rwa_security_nft";

pub trait ISecurityNftModule: ITestModule {
    fn rwa_security_nft(&self) -> GenericInit<InitParam, Event> {
        GenericInit::<InitParam, Event>::new(self.module_ref(), CONTRACT_NAME)
    }
}

pub trait ISecurityNftContract: ITestContract {
    fn add_agent(&self) -> GenericReceive<Address, (), Event> {
        GenericReceive::<Address, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "addAgent",
            self.max_energy(),
        )
    }

    fn mint(&self) -> GenericReceive<MintParams, (), Event> {
        GenericReceive::<MintParams, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "mint",
            self.max_energy(),
        )
    }
}

pub trait ISecurityNftContractExt: ISecurityNftContract {
    fn mint_single_update(
        &self,
        chain: &mut Chain,
        sender: &Account,
        owner: Receiver,
        params: ContractMetadataUrl,
    ) -> Result<concordium_cis2::TokenIdU8, ContractInvokeError> {
        self.mint()
            .update(chain, sender, &MintParams {
                owner,
                tokens: vec![MintParam {
                    metadata_url: params,
                }],
            })
            .map(|r| {
                *self
                    .mint()
                    .parse_events(&r)
                    .expect("security_nft: mint - parse events")
                    .iter()
                    .filter_map(|e| {
                        if let Event::Cis2(concordium_cis2::Cis2Event::Mint(mint_event)) = e {
                            Some(mint_event.token_id)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .first()
                    .expect("security_nft: mint - token id")
            })
    }
}

pub struct SecurityNftModule {
    pub module_path: String,
}

impl ITestModule for SecurityNftModule {
    fn module_path(&self) -> String { self.module_path.to_owned() }
}

impl ISecurityNftModule for SecurityNftModule {}

pub struct SecurityNftContract(pub ContractAddress);

impl ITestContract for SecurityNftContract {
    fn contract_name() -> OwnedContractName {
        OwnedContractName::new_unchecked(CONTRACT_NAME.to_owned())
    }

    fn contract_address(&self) -> ContractAddress { self.0 }
}

impl ISecurityNftContract for SecurityNftContract {}
impl ISecurityNftContractExt for SecurityNftContract {}
impl ICis2Contract<TokenId, TokenAmount, Event> for SecurityNftContract {}
impl ICis2ContractExt<TokenId, TokenAmount, Event> for SecurityNftContract {}
