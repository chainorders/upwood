/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { ForestProject } from "./ForestProject";
import type { ForestProjectTokenContract } from "./ForestProjectTokenContract";
import type { Market } from "./Market";
import type { SecurityMintFund } from "./SecurityMintFund";
import type { TokenMetadata } from "./TokenMetadata";

export type ForestProjectAggApiModel = {
	forest_project: ForestProject;
	supply: string;
	user_balance: string;
	property_contract?: ForestProjectTokenContract;
	property_market?: Market;
	property_market_currency_metadata?: TokenMetadata;
	property_fund?: SecurityMintFund;
	property_fund_currency_metadata?: TokenMetadata;
	bond_contract?: ForestProjectTokenContract;
	bond_fund?: SecurityMintFund;
	bond_fund_currency_metadata?: TokenMetadata;
	contract_signed: boolean;
	user_notified: boolean;
};
