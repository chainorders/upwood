/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ApiUser } from "../models/ApiUser";
import type { LoginReq } from "../models/LoginReq";
import type { LoginRes } from "../models/LoginRes";
import type { PagedResponse_ApiUser_ } from "../models/PagedResponse_ApiUser_";
import type { PagedResponse_UserRegistrationRequest_ } from "../models/PagedResponse_UserRegistrationRequest_";
import type { SystemContractsConfig } from "../models/SystemContractsConfig";
import type { UserCreatePostReq } from "../models/UserCreatePostReq";
import type { UserCreatePostReqAdmin } from "../models/UserCreatePostReqAdmin";
import type { UserRegisterGetRes } from "../models/UserRegisterGetRes";
import type { UserRegistrationRequest } from "../models/UserRegistrationRequest";
import type { UserRegistrationRequestApi } from "../models/UserRegistrationRequestApi";

import type { CancelablePromise } from "../core/CancelablePromise";
import { OpenAPI } from "../core/OpenAPI";
import { request as __request } from "../core/request";

export class UserService {
	/**
	 * @param requestBody
	 * @returns any
	 * @throws ApiError
	 */
	public static postUserRegistrationRequest(requestBody: UserRegistrationRequestApi): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/user/registration-request",
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}

	/**
	 * @param page
	 * @returns PagedResponse_UserRegistrationRequest_
	 * @throws ApiError
	 */
	public static getAdminRegistrationRequestList(
		page: number,
	): CancelablePromise<PagedResponse_UserRegistrationRequest_> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/registration-request/list/{page}",
			path: {
				page: page,
			},
		});
	}

	/**
	 * @param id
	 * @returns UserRegistrationRequest
	 * @throws ApiError
	 */
	public static getAdminRegistrationRequest(id: string): CancelablePromise<UserRegistrationRequest> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/registration-request/{id}",
			path: {
				id: id,
			},
		});
	}

	/**
	 * @param id
	 * @param isAccepted
	 * @returns any
	 * @throws ApiError
	 */
	public static putAdminRegistrationRequestAccept(id: string, isAccepted: boolean): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "PUT",
			url: "/admin/registration-request/{id}/accept/{is_accepted}",
			path: {
				id: id,
				is_accepted: isAccepted,
			},
		});
	}

	/**
	 * @param registrationRequestId
	 * @returns UserRegisterGetRes
	 * @throws ApiError
	 */
	public static getUserRegister(registrationRequestId: string): CancelablePromise<UserRegisterGetRes> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/user/register/{registration_request_id}",
			path: {
				registration_request_id: registrationRequestId,
			},
		});
	}

	/**
	 * @param registrationRequestId
	 * @param requestBody
	 * @returns ApiUser
	 * @throws ApiError
	 */
	public static postUserRegister(
		registrationRequestId: string,
		requestBody: UserCreatePostReq,
	): CancelablePromise<ApiUser> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/user/register/{registration_request_id}",
			path: {
				registration_request_id: registrationRequestId,
			},
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}

	/**
	 * @param registrationRequestId
	 * @param requestBody
	 * @returns ApiUser
	 * @throws ApiError
	 */
	public static postAdminUserRegister(
		registrationRequestId: string,
		requestBody: UserCreatePostReqAdmin,
	): CancelablePromise<ApiUser> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/admin/user/register/{registration_request_id}",
			path: {
				registration_request_id: registrationRequestId,
			},
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}

	/**
	 * @param requestBody
	 * @returns LoginRes
	 * @throws ApiError
	 */
	public static postUserLogin(requestBody: LoginReq): CancelablePromise<LoginRes> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/user/login",
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}

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
	public static getUser(): CancelablePromise<ApiUser> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/user",
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
	 * @returns ApiUser
	 * @throws ApiError
	 */
	public static getAdminUser(cognitoUserId: string): CancelablePromise<ApiUser> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/user/{cognito_user_id}",
			path: {
				cognito_user_id: cognitoUserId,
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
	 * @returns ApiUser
	 * @throws ApiError
	 */
	public static getAdminUserAccountAddress(accountAddress: string): CancelablePromise<ApiUser> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/user/account_address/{account_address}",
			path: {
				account_address: accountAddress,
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
	 * @returns PagedResponse_ApiUser_
	 * @throws ApiError
	 */
	public static getAdminUserList(page: number): CancelablePromise<PagedResponse_ApiUser_> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/user/list/{page}",
			path: {
				page: page,
			},
		});
	}

	/**
	 * @returns SystemContractsConfig
	 * @throws ApiError
	 */
	public static getSystemConfig(): CancelablePromise<SystemContractsConfig> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/system_config",
		});
	}
}
