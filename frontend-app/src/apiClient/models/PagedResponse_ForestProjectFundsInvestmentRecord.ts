/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { ForestProjectFundsInvestmentRecord } from "./ForestProjectFundsInvestmentRecord";

/**
 * Pages Response. This is a generic response that can be used to return a list
 * of items with pagination.
 */
export type PagedResponse_ForestProjectFundsInvestmentRecord = {
	page_count: number;
	page: number;
	data: Array<ForestProjectFundsInvestmentRecord>;
};
