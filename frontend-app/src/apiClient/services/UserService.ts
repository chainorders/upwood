/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Company } from "../models/Company";
import type { CompanyInvitation } from "../models/CompanyInvitation";
import type { Notification } from "../models/Notification";
import type { PagedResponse_CompanyInvitation } from "../models/PagedResponse_CompanyInvitation";
import type { PagedResponse_UserKYCModel } from "../models/PagedResponse_UserKYCModel";
import type { PagedResponse_UserRegistrationRequest } from "../models/PagedResponse_UserRegistrationRequest";
import type { SystemContractsConfigApiModel } from "../models/SystemContractsConfigApiModel";
import type { UserCompanyCreateUpdateReq } from "../models/UserCompanyCreateUpdateReq";
import type { UserCompanyInvitationCreateReq } from "../models/UserCompanyInvitationCreateReq";
import type { UserCreatePostReq } from "../models/UserCreatePostReq";
import type { UserCreatePostReqAdmin } from "../models/UserCreatePostReqAdmin";
import type { UserKYCModel } from "../models/UserKYCModel";
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
	 * @returns PagedResponse_UserRegistrationRequest
	 * @throws ApiError
	 */
	public static getAdminRegistrationRequestList(
		page?: number,
		pageSize?: number,
	): CancelablePromise<PagedResponse_UserRegistrationRequest> {
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
	 * @returns PagedResponse_UserKYCModel
	 * @throws ApiError
	 */
	public static getAdminUserList(page?: number, pageSize?: number): CancelablePromise<PagedResponse_UserKYCModel> {
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

	/**
	 * @returns Company
	 * @throws ApiError
	 */
	public static getCompany(): CancelablePromise<Company> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/company",
		});
	}

	/**
	 * @param requestBody
	 * @returns Company
	 * @throws ApiError
	 */
	public static postCompany(requestBody: UserCompanyCreateUpdateReq): CancelablePromise<Company> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/company",
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}

	/**
	 * @param requestBody
	 * @returns Company
	 * @throws ApiError
	 */
	public static putCompany(requestBody: UserCompanyCreateUpdateReq): CancelablePromise<Company> {
		return __request(OpenAPI, {
			method: "PUT",
			url: "/company",
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}

	/**
	 * @param requestBody
	 * @returns CompanyInvitation
	 * @throws ApiError
	 */
	public static postCompanyInvitation(
		requestBody: UserCompanyInvitationCreateReq,
	): CancelablePromise<CompanyInvitation> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/company/invitation",
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}

	/**
	 * @param invitationId
	 * @param accepted
	 * @returns any
	 * @throws ApiError
	 */
	public static putCompanyInvitation(invitationId: string, accepted: boolean): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "PUT",
			url: "/company/invitation",
			query: {
				invitation_id: invitationId,
				accepted: accepted,
			},
		});
	}

	/**
	 * @param invitationId
	 * @returns any
	 * @throws ApiError
	 */
	public static deleteCompanyInvitation(invitationId: string): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "DELETE",
			url: "/company/invitation",
			query: {
				invitation_id: invitationId,
			},
		});
	}

	/**
	 * @returns PagedResponse_CompanyInvitation
	 * @throws ApiError
	 */
	public static getCompanyInvitationList(): CancelablePromise<PagedResponse_CompanyInvitation> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/company/invitation/list",
		});
	}

	/**
	 * @returns PagedResponse_UserKYCModel
	 * @throws ApiError
	 */
	public static getCompanyMembersList(): CancelablePromise<PagedResponse_UserKYCModel> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/company/members/list",
		});
	}

	/**
	 * @param userId
	 * @returns any
	 * @throws ApiError
	 */
	public static deleteCompanyMembers(userId: string): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "DELETE",
			url: "/company/members",
			query: {
				user_id: userId,
			},
		});
	}
}
