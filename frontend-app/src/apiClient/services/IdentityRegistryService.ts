/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ListenerContract } from '../models/ListenerContract';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class IdentityRegistryService {

    /**
     * Retrieves the details of the identity registry contract.
     * This function ensures the user is an admin, retrieves the contract details from the database, and returns the contract information as a JSON response.
     *
     * # Arguments
     * * `db_pool` - A reference to the database connection pool.
     * * `claims` - The bearer authorization claims of the authenticated user.
     * * `identity_registry` - A reference to the identity registry contract address.
     *
     * # Returns
     * A JSON result containing the `ListenerContract` details.
     * @returns ListenerContract
     * @throws ApiError
     */
    public static getAdminIdentityRegistryContract(): CancelablePromise<ListenerContract> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/identity_registry/contract',
        });
    }

}
