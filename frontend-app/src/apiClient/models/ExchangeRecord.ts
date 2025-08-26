/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { ExchangeRecordType } from "./ExchangeRecordType";

export type ExchangeRecord = {
	id: string;
	block_height: string;
	txn_index: string;
	contract_address: string;
	token_id: string;
	token_contract_address: string;
	currency_token_id: string;
	currency_token_contract_address: string;
	seller: string;
	buyer: string;
	currency_amount: string;
	token_amount: string;
	rate: string;
	create_time: string;
	exchange_record_type: ExchangeRecordType;
};
