/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { UserKYCModel } from "./UserKYCModel";

/**
 * Pages Response. This is a generic response that can be used to return a list
 * of items with pagination.
 */
export type PagedResponse_UserKYCModel = {
	page_count: number;
	page: number;
	data: Array<UserKYCModel>;
};
