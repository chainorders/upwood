/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AdminUser } from '../models/AdminUser';
import type { ApiUser } from '../models/ApiUser';
import type { CreateChallengeResponse } from '../models/CreateChallengeResponse';
import type { PagedResponse_AdminUser_ } from '../models/PagedResponse_AdminUser_';
import type { SystemContractsConfig } from '../models/SystemContractsConfig';
import type { UpdateAccountAddressReq } from '../models/UpdateAccountAddressReq';
import type { UserRegisterReq } from '../models/UserRegisterReq';
import type { UserRegistrationInvitationSendReq } from '../models/UserRegistrationInvitationSendReq';
import type { UserUpdateAccountAddressRequest } from '../models/UserUpdateAccountAddressRequest';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class UserService {

    /**
     * Retrieves the current user's information based on the provided bearer authorization token.
     * This function fetches the user's information from the database using the Cognito user ID
     * from the bearer authorization token. It also checks if the user's account is KYC verified
     * by looking up the identity registry.
     *
     * # Arguments
     * * `db_pool` - A reference to the database connection pool.
     * * `identity_registry` - A reference to the identity registry.
     * * `claims` - The bearer authorization token claims.
     *
     * # Returns
     * A `JsonResult` containing the user's information.
     * @returns ApiUser
     * @throws ApiError
     */
    public static getUsers(): CancelablePromise<ApiUser> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/users',
        });
    }

    /**
     * Inserts a new user into the database and Cognito user pool.
     * If the user's email is not yet verified, this function will set the email as verified.
     *
     * The function will upsert the user information in the database, including the Cognito user ID, email, and desired investment amount.
     *
     * If the user has an associated account address, the function will check if the identity for that address exists in the identity registry.
     *
     * # Arguments
     * * `user_pool` - A reference to the Cognito user pool.
     * * `db_pool` - A reference to the database connection pool.
     * * `identity_registry` - A reference to the identity registry.
     * * `claims` - The bearer authorization claims for the current user.
     * * `req` - The user registration request containing the desired investment amount.
     *
     * # Returns
     * The newly created or updated user.
     * @param requestBody
     * @returns ApiUser
     * @throws ApiError
     */
    public static postUsers(
        requestBody: UserRegisterReq,
    ): CancelablePromise<ApiUser> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/users',
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * Sends a registration invitation for a new user.
     * This function first checks if the user already exists in the Cognito user pool. If the user exists and their email is already verified, an error is returned.
     * Otherwise, the function either resets the password for the existing user or creates a new user in the Cognito user pool.
     *
     * If the request includes an affiliate account address, the function also inserts the affiliation information into the database.
     *
     * # Arguments
     * * `user_pool` - A reference to the Cognito user pool.
     * * `db_pool` - A reference to the database connection pool.
     * * `req` - The request containing the email and optional affiliate account address.
     *
     * # Returns
     * The user ID of the user for whom the registration invitation was sent.
     * @param requestBody
     * @returns string
     * @throws ApiError
     */
    public static postUsersInvitation(
        requestBody: UserRegistrationInvitationSendReq,
    ): CancelablePromise<string> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/users/invitation',
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * Generates a new challenge for the user to verify their account address.
     * This function first checks if the user has an existing valid challenge. If not, it generates a new challenge and stores it in the database.
     *
     * The function returns the challenge and the serialized identity statement for the user's account address.
     *
     * # Arguments
     * * `id_statement` - A reference to the user's identity statement.
     * * `db_pool` - A reference to the database connection pool.
     * * `config` - A reference to the user challenge configuration.
     * * `claims` - The bearer authorization claims for the current user.
     *
     * # Returns
     * A `CreateChallengeResponse` containing the challenge and the serialized identity statement.
     * @returns CreateChallengeResponse
     * @throws ApiError
     */
    public static postUsersAccountAddressGenerateChallenge(): CancelablePromise<CreateChallengeResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/users/account_address/generate_challenge',
        });
    }

    /**
     * Updates the account address for the current user.
     * # Arguments
     * * `claims` - The bearer authorization claims for the current user.
     * * `concordium_client` - A reference to the Concordium client.
     * * `db_pool` - A reference to the database connection pool.
     * * `user_pool` - A reference to the Cognito user pool.
     * * `network` - A reference to the DID network.
     * * `global_context` - A reference to the Concordium global context.
     * * `config` - A reference to the user challenge configuration.
     * * `request` - The request containing the proof to verify the account address update.
     *
     * # Returns
     * A `NoResResult` indicating the success or failure of the operation.
     * @param requestBody
     * @returns any
     * @throws ApiError
     */
    public static putUsersAccountAddress(
        requestBody: UpdateAccountAddressReq,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'PUT',
            url: '/users/account_address',
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * Get a user by their Cognito user ID.
     * This endpoint is only accessible to admin users.
     *
     * # Arguments
     * - `db_pool`: A reference to the database connection pool.
     * - `identity_registry`: A reference to the identity registry.
     * - `claims`: The authorization claims of the requesting user.
     * - `cognito_user_id`: The Cognito user ID of the user to retrieve.
     *
     * # Returns
     * A JSON response containing the `AdminUser` for the specified Cognito user ID.
     * @param cognitoUserId
     * @returns AdminUser
     * @throws ApiError
     */
    public static getAdminUsers(
        cognitoUserId: string,
    ): CancelablePromise<AdminUser> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/users/{cognito_user_id}',
            path: {
                'cognito_user_id': cognitoUserId,
            },
        });
    }

    /**
     * Delete a user by their Cognito user ID.
     * This endpoint is only accessible to admin users.
     *
     * # Arguments
     * - `user_pool`: A reference to the Cognito user pool.
     * - `db_pool`: A reference to the database connection pool.
     * - `claims`: The authorization claims of the requesting user.
     * - `cognito_user_id`: The Cognito user ID of the user to delete.
     *
     * # Returns
     * A JSON response indicating the success of the deletion.
     * @param cognitoUserId
     * @returns any
     * @throws ApiError
     */
    public static deleteAdminUsers(
        cognitoUserId: string,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/admin/users/{cognito_user_id}',
            path: {
                'cognito_user_id': cognitoUserId,
            },
        });
    }

    /**
     * Get a user by their account address.
     * This endpoint is only accessible to admin users.
     *
     * # Arguments
     * - `db_pool`: A reference to the database connection pool.
     * - `identity_registry`: A reference to the identity registry.
     * - `claims`: The authorization claims of the requesting user.
     * - `account_address`: The account address of the user to retrieve.
     *
     * # Returns
     * A JSON response containing the `AdminUser` for the specified account address.
     * @param accountAddress
     * @returns AdminUser
     * @throws ApiError
     */
    public static getAdminUsersAccountAddress(
        accountAddress: string,
    ): CancelablePromise<AdminUser> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/users/account_address/{account_address}',
            path: {
                'account_address': accountAddress,
            },
        });
    }

    /**
     * Get a list of all the users.
     * This endpoint is only accessible to admin users.
     *
     * # Arguments
     * - `db_pool`: A reference to the database connection pool.
     * - `identity_registry`: A reference to the identity registry.
     * - `claims`: The authorization claims of the requesting user.
     * - `page`: The page number to retrieve.
     *
     * # Returns
     * A JSON response containing a paged list of `AdminUser` objects.
     * @param page
     * @returns PagedResponse_AdminUser_
     * @throws ApiError
     */
    public static getAdminUsersList(
        page: number,
    ): CancelablePromise<PagedResponse_AdminUser_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/users/list/{page}',
            path: {
                'page': page,
            },
        });
    }

    /**
     * Update the Concordium account address for a user.
     * This endpoint is only accessible to admin users.
     *
     * # Arguments
     * - `user_pool`: A reference to the Cognito user pool.
     * - `db_pool`: A reference to the database connection pool.
     * - `claims`: The authorization claims of the requesting user.
     * - `cognito_user_id`: The Cognito user ID of the user to update.
     * - `request`: The request body containing the new account address.
     *
     * # Returns
     * A successful response indicating the account address was updated.
     * @param cognitoUserId
     * @param requestBody
     * @returns any
     * @throws ApiError
     */
    public static putAdminUsersAccountAddress(
        cognitoUserId: string,
        requestBody: UserUpdateAccountAddressRequest,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'PUT',
            url: '/admin/users/{cognito_user_id}/account_address',
            path: {
                'cognito_user_id': cognitoUserId,
            },
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * @returns SystemContractsConfig
     * @throws ApiError
     */
    public static getSystemConfig(): CancelablePromise<SystemContractsConfig> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/system_config',
        });
    }

}
