/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { InvestmentRecordType } from "./InvestmentRecordType";

export type InvestmentRecord = {
	id: string;
	block_height: string;
	txn_index: string;
	contract_address: string;
	investment_token_id: string;
	investment_token_contract_address: string;
	currency_token_id: string;
	currency_token_contract_address: string;
	investor: string;
	currency_amount: string;
	token_amount: string;
	currency_amount_balance: string;
	token_amount_balance: string;
	investment_record_type: InvestmentRecordType;
	create_time: string;
};
