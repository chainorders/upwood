/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { P2PTradeContract } from "./P2PTradeContract";
import type { SecurityMintFundContract } from "./SecurityMintFundContract";

export type SystemContractsConfigApiModel = {
	identity_registry_contract_index: string;
	compliance_contract_index: string;
	carbon_credit_contract_index: string;
	carbon_credit_token_id: string;
	euro_e_contract_index: string;
	euro_e_token_id: string;
	tree_ft_contract_index: string;
	tree_nft_contract_index: string;
	offchain_rewards_contract_index: string;
	mint_funds_contract_index: string;
	trading_contract_index: string;
	yielder_contract_index: string;
	mint_funds_contract: SecurityMintFundContract;
	trading_contract: P2PTradeContract;
};
