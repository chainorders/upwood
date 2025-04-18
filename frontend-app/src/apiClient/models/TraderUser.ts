/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SecurityTokenContractType } from "./SecurityTokenContractType";

export type TraderUser = {
	contract_address: string;
	token_id: string;
	token_contract_address: string;
	currency_token_id: string;
	currency_token_contract_address: string;
	trader: string;
	token_in_amount: string;
	token_out_amount: string;
	currency_in_amount: string;
	currency_out_amount: string;
	create_time: string;
	update_time: string;
	cognito_user_id?: string;
	email?: string;
	forest_project_id?: string;
	forest_project_name?: string;
	forest_project_contract_type?: SecurityTokenContractType;
};
