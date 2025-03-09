/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { UserTokenHolder } from "./UserTokenHolder";

/**
 * Pages Response. This is a generic response that can be used to return a list
 * of items with pagination.
 */
export type PagedResponse_UserTokenHolder = {
	page_count: number;
	page: number;
	data: Array<UserTokenHolder>;
};
