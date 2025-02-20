/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SecurityTokenContractType } from "./SecurityTokenContractType";

export type ForestProjectFundsAffiliateRewardRecord = {
	investment_record_id: string;
	fund_contract_address: string;
	investment_token_contract_address: string;
	investment_token_id: string;
	fund_type: SecurityTokenContractType;
	forest_project_id: string;
	is_default?: boolean;
	investor_cognito_user_id: string;
	investor_account_address: string;
	currency_token_id: string;
	currency_token_contract_address: string;
	currency_amount: string;
	token_amount: string;
	investment_token_symbol: string;
	investment_token_decimals: number;
	currency_token_symbol: string;
	currency_token_decimals: number;
	claim_id?: string;
	claims_contract_address?: string;
	reward_amount: string;
	remaining_reward_amount: string;
	affiliate_cognito_user_id: string;
	affiliate_commission: string;
};
