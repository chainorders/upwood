/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SecurityTokenContractType } from "./SecurityTokenContractType";
import type { TokenHolderBalanceUpdateType } from "./TokenHolderBalanceUpdateType";

export type TokenHolderUserBalanceUpdate = {
	id: string;
	block_height: string;
	txn_index: string;
	cis2_address: string;
	token_id: string;
	holder_address: string;
	amount: string;
	frozen_balance: string;
	un_frozen_balance: string;
	txn_sender: string;
	txn_instigator: string;
	update_type: TokenHolderBalanceUpdateType;
	create_time: string;
	cognito_user_id?: string;
	email?: string;
	forest_project_id?: string;
	forest_project_name?: string;
	forest_project_contract_type?: SecurityTokenContractType;
};
