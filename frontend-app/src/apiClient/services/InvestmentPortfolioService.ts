/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { InvestmentPortfolioUserAggregate } from "../models/InvestmentPortfolioUserAggregate";
import type { PortfolioValue } from "../models/PortfolioValue";

import type { CancelablePromise } from "../core/CancelablePromise";
import { OpenAPI } from "../core/OpenAPI";
import { request as __request } from "../core/request";

export class InvestmentPortfolioService {
	/**
	 * @param at
	 * @returns string
	 * @throws ApiError
	 */
	public static getPortfolioValue(at: string): CancelablePromise<string> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/portfolio/value",
			query: {
				at: at,
			},
		});
	}

	/**
	 * @param now
	 * @returns InvestmentPortfolioUserAggregate
	 * @throws ApiError
	 */
	public static getPortfolioAggregate(now?: string): CancelablePromise<InvestmentPortfolioUserAggregate> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/portfolio/aggregate",
			query: {
				now: now,
			},
		});
	}

	/**
	 * @param months
	 * @param now
	 * @returns PortfolioValue
	 * @throws ApiError
	 */
	public static getPortfolioValueLastNMonths(months: number, now?: string): CancelablePromise<Array<PortfolioValue>> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/portfolio/value_last_n_months/{months}",
			path: {
				months: months,
			},
			query: {
				now: now,
			},
		});
	}
}
