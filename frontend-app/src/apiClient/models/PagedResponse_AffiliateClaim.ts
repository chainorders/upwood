/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { AffiliateClaim } from "./AffiliateClaim";

/**
 * Pages Response. This is a generic response that can be used to return a list
 * of items with pagination.
 */
export type PagedResponse_AffiliateClaim = {
	page_count: number;
	page: number;
	data: Array<AffiliateClaim>;
};
