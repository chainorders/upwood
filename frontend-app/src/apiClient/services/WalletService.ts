/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ClaimRequest } from '../models/ClaimRequest';
import type { PagedResponse_ForestProjectFundsAffiliateRewardRecord_ } from '../models/PagedResponse_ForestProjectFundsAffiliateRewardRecord_';
import type { PagedResponse_ForestProjectFundsInvestmentRecord_ } from '../models/PagedResponse_ForestProjectFundsInvestmentRecord_';
import type { PagedResponse_UserTransaction_ } from '../models/PagedResponse_UserTransaction_';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class WalletService {

    /**
     * @param page
     * @returns PagedResponse_ForestProjectFundsInvestmentRecord_
     * @throws ApiError
     */
    public static getUserInvestmentsList(
        page: number,
    ): CancelablePromise<PagedResponse_ForestProjectFundsInvestmentRecord_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/user/investments/list/{page}',
            path: {
                'page': page,
            },
        });
    }

    /**
     * @param page
     * @returns PagedResponse_ForestProjectFundsAffiliateRewardRecord_
     * @throws ApiError
     */
    public static getUserAffiliateRewardsList(
        page: number,
    ): CancelablePromise<PagedResponse_ForestProjectFundsAffiliateRewardRecord_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/user/affiliate/rewards/list/{page}',
            path: {
                'page': page,
            },
        });
    }

    /**
     * @param investmentRecordId
     * @returns ClaimRequest
     * @throws ApiError
     */
    public static getUserAffiliateRewardsClaim(
        investmentRecordId: string,
    ): CancelablePromise<ClaimRequest> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/user/affiliate/rewards/claim/{investment_record_id}',
            path: {
                'investment_record_id': investmentRecordId,
            },
        });
    }

    /**
     * @param page
     * @returns PagedResponse_UserTransaction_
     * @throws ApiError
     */
    public static getUserTransactionsList(
        page: number,
    ): CancelablePromise<PagedResponse_UserTransaction_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/user/transactions/list/{page}',
            path: {
                'page': page,
            },
        });
    }

}
