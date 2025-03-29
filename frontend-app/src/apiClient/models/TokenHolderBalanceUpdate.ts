/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { TokenHolderBalanceUpdateType } from "./TokenHolderBalanceUpdateType";

export type TokenHolderBalanceUpdate = {
	id: string;
	id_serial?: number;
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
};
