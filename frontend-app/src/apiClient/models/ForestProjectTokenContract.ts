/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SecurityTokenContractType } from "./SecurityTokenContractType";

export type ForestProjectTokenContract = {
	contract_address: string;
	forest_project_id: string;
	contract_type: SecurityTokenContractType;
	fund_token_id?: string;
	symbol: string;
	decimals: number;
	metadata_url: string;
	metadata_hash?: string;
	created_at: string;
	updated_at: string;
};
