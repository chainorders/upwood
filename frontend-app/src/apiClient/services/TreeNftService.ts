/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { NftMultiRewardedDetails } from "../models/NftMultiRewardedDetails";

import type { CancelablePromise } from "../core/CancelablePromise";
import { OpenAPI } from "../core/OpenAPI";
import { request as __request } from "../core/request";

export class TreeNftService {
	/**
	 * Retrieves the nonce for the specified contract index and the authenticated account.
	 * # Arguments
	 * - `claims`: The authenticated account claims.
	 * - `contract_index`: The index of the contract to retrieve the nonce for.
	 * - `db_pool`: The database connection pool.
	 *
	 * # Returns
	 * The nonce for the specified contract index and authenticated account.
	 * @returns number
	 * @throws ApiError
	 */
	public static getTreeNftContractSelfNonce(): CancelablePromise<number> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/tree_nft/contract/self_nonce",
		});
	}

	/**
	 * @returns NftMultiRewardedDetails
	 * @throws ApiError
	 */
	public static getAdminTreeNftContract(): CancelablePromise<NftMultiRewardedDetails> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/tree_nft/contract",
		});
	}
}
