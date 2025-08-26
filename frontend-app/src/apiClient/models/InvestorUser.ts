/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SecurityTokenContractType } from "./SecurityTokenContractType";

export type InvestorUser = {
	contract_address: string;
	investment_token_id: string;
	investment_token_contract_address: string;
	investor: string;
	currency_token_id: string;
	currency_token_contract_address: string;
	currency_amount: string;
	currency_amount_total: string;
	token_amount: string;
	token_amount_total: string;
	create_time: string;
	update_time: string;
	cognito_user_id?: string;
	email?: string;
	forest_project_id?: string;
	forest_project_name?: string;
	forest_project_contract_type?: SecurityTokenContractType;
};
