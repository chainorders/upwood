/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { MaintenanceMessage } from "./MaintenanceMessage";

/**
 * Pages Response. This is a generic response that can be used to return a list
 * of items with pagination.
 */
export type PagedResponse_MaintenanceMessage = {
	page_count: number;
	page: number;
	data: Array<MaintenanceMessage>;
};
