/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Market } from "./Market";

/**
 * Pages Response. This is a generic response that can be used to return a list
 * of items with pagination.
 */
export type PagedResponse_Market = {
	page_count: number;
	page: number;
	data: Array<Market>;
};
