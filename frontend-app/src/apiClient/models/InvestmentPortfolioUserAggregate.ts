/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { TokenMetadata } from "./TokenMetadata";

export type InvestmentPortfolioUserAggregate = {
	euro_e_token_metadata?: TokenMetadata;
	/**
	 * The amount of locked euros in the all the mint fund contracts
	 */
	locked_mint_fund_euro_e_amount: string;
	/**
	 * Sum of the amount invested in the mint funds and the amount bought in the P2P trading
	 */
	invested_value: string;
	/**
	 * Sum Of(Balance of each Forest Project Token * the current price of the token)
	 */
	current_portfolio_value: string;
	/**
	 * Current portfolio value - Portfolio value at the beginning of the year - Amount invested in the year + Amount withdrawn in the year
	 */
	yearly_return: string;
	/**
	 * Current portfolio value - Portfolio value at the beginning of the month - Amount invested in the month + Amount withdrawn in the month
	 */
	monthly_return: string;
	/**
	 * (Current portfolio value - Amount withdrawn) / Total amount invested
	 */
	return_on_investment: string;
	/**
	 * The total amount of carbon credit tokens burned
	 */
	carbon_tons_offset: string;
};
