/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { SftSingleTokenDetails } from '../models/SftSingleTokenDetails';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class CarbonCreditsService {

    /**
     * Retrieves the details of the carbon credit contract.
     * This function is an admin-only endpoint that retrieves the details of the carbon credit contract.
     *
     * # Arguments
     * * `db_pool` - A reference to the database connection pool.
     * * `claims` - The bearer authorization claims of the authenticated user.
     * * `carbon_credit_contract` - A reference to the carbon credit contract address.
     *
     * # Returns
     * A JSON result containing the token details of the carbon credit contract.
     * @returns SftSingleTokenDetails
     * @throws ApiError
     */
    public static getAdminCarbonCreditsContract(): CancelablePromise<SftSingleTokenDetails> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/carbon_credits/contract',
        });
    }

}
