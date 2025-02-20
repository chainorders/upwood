/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Notification } from "../models/Notification";
import type { PagedResponse_UserKYCModel_ } from "../models/PagedResponse_UserKYCModel_";
import type { PagedResponse_UserRegistrationRequest_ } from "../models/PagedResponse_UserRegistrationRequest_";
import type { SystemContractsConfigApiModel } from "../models/SystemContractsConfigApiModel";
import type { UserCreatePostReq } from "../models/UserCreatePostReq";
import type { UserCreatePostReqAdmin } from "../models/UserCreatePostReqAdmin";
import type { UserKYCModel } from "../models/UserKYCModel";
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
	 * @param pageSize
	 * @returns PagedResponse_UserRegistrationRequest_
	 * @throws ApiError
	 */
	public static getAdminRegistrationRequestList(
		page?: number,
		pageSize?: number,
	): CancelablePromise<PagedResponse_UserRegistrationRequest_> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/registration-request/list",
			query: {
				page: page,
				page_size: pageSize,
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
	 * Accept or reject a user registration request.
	 * If the request is accepted, the user is added to the Cognito user pool.
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
	 * @returns UserRegisterGetRes
	 * @throws ApiError
	 */
	public static getUserRegister(): CancelablePromise<UserRegisterGetRes> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/user/register",
		});
	}

	/**
	 * Registers a user in the Cognito user pool and in the database.
	 * @param requestBody
	 * @returns UserKYCModel
	 * @throws ApiError
	 */
	public static postUserRegister(requestBody: UserCreatePostReq): CancelablePromise<UserKYCModel> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/user/register",
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}

	/**
	 * @param requestBody
	 * @returns UserKYCModel
	 * @throws ApiError
	 */
	public static postAdminUserRegister(requestBody: UserCreatePostReqAdmin): CancelablePromise<UserKYCModel> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/admin/user/register",
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
	 * @returns UserKYCModel
	 * @throws ApiError
	 */
	public static getUser(): CancelablePromise<UserKYCModel> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/user",
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
	 * @param pageSize
	 * @returns PagedResponse_UserKYCModel_
	 * @throws ApiError
	 */
	public static getAdminUserList(page?: number, pageSize?: number): CancelablePromise<PagedResponse_UserKYCModel_> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/user/list",
			query: {
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @returns SystemContractsConfigApiModel
	 * @throws ApiError
	 */
	public static getSystemConfig(): CancelablePromise<SystemContractsConfigApiModel> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/system_config",
		});
	}

	/**
	 * @param requestBody
	 * @returns Notification
	 * @throws ApiError
	 */
	public static postUserNotifications(requestBody: string): CancelablePromise<Notification> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/user/notifications",
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}
}
