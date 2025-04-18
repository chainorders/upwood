/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SecurityTokenContractType } from "./SecurityTokenContractType";

export type ForestProjectContract = {
	contract_address: string;
	module_ref: string;
	contract_name: string;
	owner: string;
	identity_registry?: string;
	compliance_contract?: string;
	forest_project_id: string;
	forest_project_name: string;
	contract_type: SecurityTokenContractType;
	fund_token_id?: string;
	symbol: string;
	decimals: number;
	metadata_url: string;
	metadata_hash?: string;
	created_at: string;
	updated_at: string;
};
