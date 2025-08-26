/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { MarketType } from "./MarketType";

export type Market = {
	contract_address: string;
	token_id?: string;
	token_contract_address: string;
	currency_token_id: string;
	currency_token_contract_address: string;
	liquidity_provider: string;
	/**
	 * Rate at which the liquidity provider is buying tokens. This is the sell rate for users of the contract.
	 */
	buy_rate_numerator?: string;
	buy_rate_denominator?: string;
	/**
	 * Rate at which the liquidity provider is selling tokens. This is the buy rate for the users of the contract.
	 */
	sell_rate_numerator?: string;
	sell_rate_denominator?: string;
	create_time: string;
	update_time: string;
	/**
	 * In case of mint market this is the start of the market. Also the start of the calculation of the token id.
	 */
	token_id_calculation_start?: string;
	/**
	 * In case of mint market this is the Milliseconds value after which the Market will move to the next token id.
	 */
	token_id_calculation_diff_millis?: string;
	/**
	 * In case of mint market this is the base token id. The token id will be calculated as base_token_id + (current_time - token_id_calculation_start) / token_id_calculation_diff_millis
	 */
	token_id_calculation_base_token_id?: string;
	market_type: MarketType;
	/**
	 * Maximum amount of tokens which the market can give out / sell.
	 * This value will decrease when someone buys and increase when someone sells
	 */
	max_token_amount: string;
	/**
	 * Maximum amount of currency units which this market will give out.
	 * This value will decrease when someone sell and increases when someone buys
	 */
	max_currency_amount?: string;
	/**
	 * Total amount  of tokens which the market as bought / users have sold.
	 */
	token_in_amount: string;
	/**
	 * Total amount of currency units which the market has given out / users have sold tokens.
	 */
	currency_out_amount: string;
	/**
	 * Total amount of tokens which the market has given out / users have bought tokens.
	 */
	token_out_amount: string;
	/**
	 * Total amount of currency units which the market has received / users have bought tokens.
	 */
	currency_in_amount: string;
};
