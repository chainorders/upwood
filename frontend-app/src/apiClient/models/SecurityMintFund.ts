/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SecurityMintFundState } from "./SecurityMintFundState";

export type SecurityMintFund = {
	contract_address: string;
	investment_token_id: string;
	investment_token_contract_address: string;
	token_id: string;
	token_contract_address: string;
	currency_token_id: string;
	currency_token_contract_address: string;
	currency_amount: string;
	token_amount: string;
	receiver_address?: string;
	rate_numerator: string;
	rate_denominator: string;
	fund_state: SecurityMintFundState;
	create_time: string;
	update_time: string;
};
