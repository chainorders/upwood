use super::{cis2_test_contract::*, test_contract_client::*};
use concordium_cis2::Receiver;
use concordium_rwa_security_nft::{event::Event, types::*};
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

    fn agents(&self) -> GenericReceive<(), Vec<Address>, Event> {
        GenericReceive::<(), Vec<Address>, Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "agents",
            self.max_energy(),
        )
    }

    fn remove_agent(&self) -> GenericReceive<Address, (), Event> {
        GenericReceive::<Address, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "removeAgent",
            self.max_energy(),
        )
    }

    fn is_agent(&self) -> GenericReceive<Address, bool, Event> {
        GenericReceive::<Address, bool, Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "isAgent",
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

    fn forced_transfer(&self) -> GenericReceive<TransferParams, (), Event> {
        GenericReceive::<TransferParams, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "forcedTransfer",
            self.max_energy(),
        )
    }

    fn pause(&self) -> GenericReceive<PauseParams, (), Event> {
        GenericReceive::<PauseParams, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "pause",
            self.max_energy(),
        )
    }

    fn un_pause(&self) -> GenericReceive<PauseParams, (), Event> {
        GenericReceive::<PauseParams, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "unPause",
            self.max_energy(),
        )
    }

    fn is_paused(&self) -> GenericReceive<PauseParams, IsPausedResponse, Event> {
        GenericReceive::<PauseParams, IsPausedResponse, Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "isPaused",
            self.max_energy(),
        )
    }

    fn burn(&self) -> GenericReceive<BurnParams, (), Event> {
        GenericReceive::<BurnParams, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "burn",
            self.max_energy(),
        )
    }

    fn freeze(&self) -> GenericReceive<FreezeParams, (), Event> {
        GenericReceive::<FreezeParams, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "freeze",
            self.max_energy(),
        )
    }

    fn un_freeze(&self) -> GenericReceive<FreezeParams, (), Event> {
        GenericReceive::<FreezeParams, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "unFreeze",
            self.max_energy(),
        )
    }

    fn balance_of_frozen(&self) -> GenericReceive<FrozenParams, FrozenResponse, Event> {
        GenericReceive::<FrozenParams, FrozenResponse, Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "balanceOfFrozen",
            self.max_energy(),
        )
    }

    fn balance_of_un_frozen(&self) -> GenericReceive<FrozenParams, FrozenResponse, Event> {
        GenericReceive::<FrozenParams, FrozenResponse, Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "balanceOfUnFrozen",
            self.max_energy(),
        )
    }

    fn recover(&self) -> GenericReceive<RecoverParam, (), Event> {
        GenericReceive::<RecoverParam, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "recover",
            self.max_energy(),
        )
    }

    fn recovery_address(&self) -> GenericReceive<Address, Option<Address>, Event> {
        GenericReceive::<Address, Option<Address>, Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "recoveryAddress",
            self.max_energy(),
        )
    }
}

pub trait ISecurityNftContractExt: ISecurityNftContract {
    /// Mint a single token and return the token id.
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
