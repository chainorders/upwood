/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { ForestProject } from "./ForestProject";
import type { ForestProjectTokenContract } from "./ForestProjectTokenContract";
import type { Market } from "./Market";
import type { SecurityMintFund } from "./SecurityMintFund";

export type ForestProjectAggApiModel = {
	forest_project: ForestProject;
	supply: string;
	user_balance: string;
	property_contract?: ForestProjectTokenContract;
	property_market?: Market;
	property_fund?: SecurityMintFund;
	bond_contract?: ForestProjectTokenContract;
	bond_market?: Market;
	bond_fund?: SecurityMintFund;
	contract_signed: boolean;
	user_notified: boolean;
};
