#![allow(clippy::too_many_arguments)]

use super::{test_contract_client::*, utils::to_token_id_vec};
use crate::{
    cis2_test_contract::{ICis2Contract, ICis2ContractExt},
    security_nft::{ISecurityNftContract, SecurityNftContract},
};
use concordium_cis2::{AdditionalData, Cis2Event, Receiver, TransferParams};
use concordium_rwa_market::types::Rate;
use concordium_rwa_security_sft::{
    deposit::DepositParams,
    event::Event,
    init::InitParam,
    mint::{AddParam, AddParams, MintParam},
    types::{NftTokenUId, TokenAmount, TokenId},
};
use concordium_smart_contract_testing::*;

pub const CONTRACT_NAME: &str = "init_rwa_security_sft";

pub trait ISecuritySftModule: ITestModule {
    fn rwa_security_sft(&self) -> GenericInit<InitParam, Event> {
        GenericInit::<InitParam, Event>::new(self.module_ref(), CONTRACT_NAME)
    }
}

pub trait IContract: ITestContract + ICis2Contract<TokenId, TokenAmount, Event> {
    fn add_tokens(&self) -> GenericReceive<AddParams, (), Event> {
        GenericReceive::<AddParams, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "addTokens",
            self.max_energy(),
        )
    }

    fn deposit(&self) -> GenericReceive<DepositParams, (), Event> {
        GenericReceive::<DepositParams, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "deposit",
            self.max_energy(),
        )
    }
}

pub struct SecuritySftModule {
    pub module_path: String,
}
impl ITestModule for SecuritySftModule {
    fn module_path(&self) -> String { self.module_path.to_owned() }
}
impl ISecuritySftModule for SecuritySftModule {}

pub struct SecuritySftContract(pub ContractAddress);
impl ITestContract for SecuritySftContract {
    fn contract_name() -> OwnedContractName {
        OwnedContractName::new_unchecked(CONTRACT_NAME.to_owned())
    }

    fn contract_address(&self) -> ContractAddress { self.0 }
}
impl IContract for SecuritySftContract {}
impl ICis2Contract<TokenId, TokenAmount, Event> for SecuritySftContract {}
impl ICis2ContractExt<TokenId, TokenAmount, Event> for SecuritySftContract {}

pub fn sft_mint(
    chain: &mut Chain,
    agent: &Account,
    nft_contract: &SecurityNftContract,
    sft_contract: &SecuritySftContract,
    owner: &Account,
    nft_metadata: concordium_rwa_security_nft::types::ContractMetadataUrl,
    sft_metadata: concordium_rwa_security_sft::types::ContractMetadataUrl,
    fractions: u64,
) -> concordium_cis2::TokenIdU32 {
    // Mint Nft Token
    let nft_token_id = {
        let nft_mint_res = nft_contract
            .mint()
            .update(chain, agent, &concordium_rwa_security_nft::mint::MintParams {
                owner:  Receiver::Account(owner.address),
                tokens: vec![concordium_rwa_security_nft::mint::MintParam {
                    metadata_url: nft_metadata,
                }],
            })
            .expect("NFT: Mint");
        let nft_token_ids: Vec<_> = nft_contract
            .mint()
            .parse_events(&nft_mint_res)
            .expect("NFT: Mint parsing events")
            .iter()
            .filter_map(|e| {
                if let concordium_rwa_security_nft::event::Event::Cis2(Cis2Event::Mint(e)) = e {
                    Some(e.token_id)
                } else {
                    None
                }
            })
            .collect();

        *nft_token_ids.first().expect("NFT: First Token Id")
    };
    // Add Nft token to Sft contract
    let sft_added_token_id = {
        let sft_add_res = sft_contract
            .add_tokens()
            .update(chain, agent, &AddParams {
                tokens: vec![AddParam {
                    deposit_token_id: NftTokenUId {
                        contract: nft_contract.contract_address(),
                        id:       to_token_id_vec(nft_token_id),
                    },
                    metadata_url:     sft_metadata,
                    fractions_rate:   Rate::new(fractions, 1).expect("Rate"),
                }],
            })
            .expect("SFT: Add Tokens");
        let sft_token_ids: Vec<_> = sft_contract
            .add_tokens()
            .parse_events(&sft_add_res)
            .expect("SFT: Add Tokens parsing events")
            .iter()
            .filter_map(|e| {
                if let concordium_rwa_security_sft::event::Event::Cis2(Cis2Event::TokenMetadata(
                    e,
                )) = e
                {
                    Some(e.token_id)
                } else {
                    None
                }
            })
            .collect();
        *sft_token_ids.first().expect("SFT: First Token Id")
    };
    // Transfer Nft token to Sft contract and mint Sft token
    let sft_minted_token_id = {
        let nft_transfer_res = nft_contract
            .transfer()
            .update(
                chain,
                owner,
                &TransferParams(vec![concordium_cis2::Transfer {
                    amount:   concordium_cis2::TokenAmountU8(1),
                    from:     Address::Account(owner.address),
                    to:       concordium_cis2::Receiver::Contract(
                        sft_contract.contract_address(),
                        OwnedEntrypointName::new_unchecked("deposit".to_string()),
                    ),
                    token_id: nft_token_id,
                    data:     AdditionalData::from(to_bytes(&MintParam {
                        deposited_token_id:    NftTokenUId {
                            contract: nft_contract.contract_address(),
                            id:       to_token_id_vec(nft_token_id),
                        },
                        deposited_amount:      concordium_cis2::TokenAmountU8(1),
                        deposited_token_owner: owner.address,
                        owner:                 Receiver::Account(owner.address),
                    })),
                }]),
            )
            .expect("NFT: Transfer & SFT Mint");
        let sft_minted_token_ids = sft_contract
            .deposit()
            .parse_events(&nft_transfer_res)
            .expect("SFT: Deposit parsing events")
            .iter()
            .filter_map(|e| {
                if let Event::Cis2(Cis2Event::Mint(e)) = e {
                    Some(e.token_id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        *sft_minted_token_ids.first().expect("SFT: First minted token Id")
    };
    // Assert that the minted token id is the same as the added token id
    assert_eq!(sft_minted_token_id, sft_added_token_id);

    sft_added_token_id
}
