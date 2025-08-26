/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SecurityTokenContractType } from "./SecurityTokenContractType";

export type TokenHolderUser = {
	cis2_address: string;
	token_id: string;
	holder_address: string;
	frozen_balance: string;
	un_frozen_balance: string;
	create_time: string;
	update_time: string;
	cognito_user_id?: string;
	email?: string;
	forest_project_id?: string;
	forest_project_name?: string;
	forest_project_contract_type?: SecurityTokenContractType;
};
