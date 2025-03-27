/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ClaimRequest } from "../models/ClaimRequest";
import type { PagedResponse_ForestProjectFundsAffiliateRewardRecord } from "../models/PagedResponse_ForestProjectFundsAffiliateRewardRecord";
import type { PagedResponse_UserTransaction } from "../models/PagedResponse_UserTransaction";

import type { CancelablePromise } from "../core/CancelablePromise";
import { OpenAPI } from "../core/OpenAPI";
import { request as __request } from "../core/request";

export class WalletService {
	/**
	 * @param page
	 * @returns PagedResponse_ForestProjectFundsAffiliateRewardRecord
	 * @throws ApiError
	 */
	public static getUserAffiliateRewardsList(
		page?: number,
	): CancelablePromise<PagedResponse_ForestProjectFundsAffiliateRewardRecord> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/user/affiliate/rewards/list",
			query: {
				page: page,
			},
		});
	}

	/**
	 * @param investmentRecordId
	 * @returns ClaimRequest
	 * @throws ApiError
	 */
	public static getUserAffiliateRewardsClaim(investmentRecordId: string): CancelablePromise<ClaimRequest> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/user/affiliate/rewards/claim/{investment_record_id}",
			path: {
				investment_record_id: investmentRecordId,
			},
		});
	}

	/**
	 * @param page
	 * @returns PagedResponse_UserTransaction
	 * @throws ApiError
	 */
	public static getUserTransactionsList(page?: number): CancelablePromise<PagedResponse_UserTransaction> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/user/transactions/list",
			query: {
				page: page,
			},
		});
	}

	/**
	 * @returns binary
	 * @throws ApiError
	 */
	public static getUserTransactionsListDownload(): CancelablePromise<Blob> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/user/transactions/list/download",
		});
	}

	/**
	 * @returns binary
	 * @throws ApiError
	 */
	public static getUserAffiliateRewardsListDownload(): CancelablePromise<Blob> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/user/affiliate/rewards/list/download",
		});
	}
}
