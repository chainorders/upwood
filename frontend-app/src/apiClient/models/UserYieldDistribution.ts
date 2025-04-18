/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SecurityTokenContractType } from "./SecurityTokenContractType";

export type UserYieldDistribution = {
	id: string;
	contract_address: string;
	token_contract_address: string;
	from_token_version: string;
	to_token_version: string;
	token_amount: string;
	yield_contract_address: string;
	yield_token_id: string;
	yield_amount: string;
	to_address: string;
	create_time: string;
	cognito_user_id?: string;
	email?: string;
	forest_project_id?: string;
	forest_project_name?: string;
	forest_project_contract_type?: SecurityTokenContractType;
};
