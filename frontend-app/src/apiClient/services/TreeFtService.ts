/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { SftSingleTokenDetails } from "../models/SftSingleTokenDetails";

import type { CancelablePromise } from "../core/CancelablePromise";
import { OpenAPI } from "../core/OpenAPI";
import { request as __request } from "../core/request";

export class TreeFtService {
	/**
	 * Retrieves the details of the token associated with the TreeFT contract.
	 * This function is an administrative endpoint that requires the caller to be an admin.
	 * It retrieves the token details from the database using the provided `DbPool` and `TreeFTContractAddress`.
	 *
	 * # Arguments
	 * - `db_pool`: A reference to the database connection pool.
	 * - `claims`: The bearer authorization claims of the caller.
	 * - `carbon_credit_contract`: A reference to the TreeFT contract address.
	 *
	 * # Returns
	 * A `JsonResult` containing the `TokenDetails` of the token associated with the TreeFT contract.
	 * @returns SftSingleTokenDetails
	 * @throws ApiError
	 */
	public static getAdminTreeFtsContract(): CancelablePromise<SftSingleTokenDetails> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/tree_fts/contract",
		});
	}
}
