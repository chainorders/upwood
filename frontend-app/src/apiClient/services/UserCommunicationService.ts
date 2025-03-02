/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Guide } from "../models/Guide";
import type { MaintenanceMessage } from "../models/MaintenanceMessage";
import type { NewsArticle } from "../models/NewsArticle";
import type { PagedResponse_Guide } from "../models/PagedResponse_Guide";
import type { PagedResponse_MaintenanceMessage } from "../models/PagedResponse_MaintenanceMessage";
import type { PagedResponse_NewsArticle } from "../models/PagedResponse_NewsArticle";
import type { PagedResponse_PlatformUpdate } from "../models/PagedResponse_PlatformUpdate";
import type { PagedResponse_SupportQuestion } from "../models/PagedResponse_SupportQuestion";
import type { PlatformUpdate } from "../models/PlatformUpdate";
import type { SupportQuestion } from "../models/SupportQuestion";

import type { CancelablePromise } from "../core/CancelablePromise";
import { OpenAPI } from "../core/OpenAPI";
import { request as __request } from "../core/request";

export class UserCommunicationService {
	/**
	 * @param page
	 * @param pageSize
	 * @returns PagedResponse_NewsArticle
	 * @throws ApiError
	 */
	public static getNewsArticlesList(page: number, pageSize?: number): CancelablePromise<PagedResponse_NewsArticle> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/news_articles/list",
			query: {
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param id
	 * @returns NewsArticle
	 * @throws ApiError
	 */
	public static getAdminNewsArticles(id: string): CancelablePromise<NewsArticle> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/news_articles/{id}",
			path: {
				id: id,
			},
		});
	}

	/**
	 * @param id
	 * @returns any
	 * @throws ApiError
	 */
	public static deleteAdminNewsArticles(id: string): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "DELETE",
			url: "/admin/news_articles/{id}",
			path: {
				id: id,
			},
		});
	}

	/**
	 * @param requestBody
	 * @returns any
	 * @throws ApiError
	 */
	public static postAdminNewsArticles(requestBody: NewsArticle): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/admin/news_articles",
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @returns PagedResponse_PlatformUpdate
	 * @throws ApiError
	 */
	public static getPlatformUpdatesList(
		page: number,
		pageSize?: number,
	): CancelablePromise<PagedResponse_PlatformUpdate> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/platform_updates/list",
			query: {
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param id
	 * @returns PlatformUpdate
	 * @throws ApiError
	 */
	public static getAdminPlatformUpdates(id: string): CancelablePromise<PlatformUpdate> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/platform_updates/{id}",
			path: {
				id: id,
			},
		});
	}

	/**
	 * @param id
	 * @returns any
	 * @throws ApiError
	 */
	public static deleteAdminPlatformUpdates(id: string): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "DELETE",
			url: "/admin/platform_updates/{id}",
			path: {
				id: id,
			},
		});
	}

	/**
	 * @returns PlatformUpdate
	 * @throws ApiError
	 */
	public static getPlatformUpdatesLatest(): CancelablePromise<PlatformUpdate> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/platform_updates/latest",
		});
	}

	/**
	 * @param requestBody
	 * @returns any
	 * @throws ApiError
	 */
	public static postAdminPlatformUpdates(requestBody: PlatformUpdate): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/admin/platform_updates",
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @returns PagedResponse_MaintenanceMessage
	 * @throws ApiError
	 */
	public static getMaintenanceMessagesList(
		page: number,
		pageSize?: number,
	): CancelablePromise<PagedResponse_MaintenanceMessage> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/maintenance_messages/list",
			query: {
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param id
	 * @returns MaintenanceMessage
	 * @throws ApiError
	 */
	public static getAdminMaintenanceMessages(id: string): CancelablePromise<MaintenanceMessage> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/maintenance_messages/{id}",
			path: {
				id: id,
			},
		});
	}

	/**
	 * @param id
	 * @returns any
	 * @throws ApiError
	 */
	public static deleteAdminMaintenanceMessages(id: string): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "DELETE",
			url: "/admin/maintenance_messages/{id}",
			path: {
				id: id,
			},
		});
	}

	/**
	 * @returns MaintenanceMessage
	 * @throws ApiError
	 */
	public static getMaintenanceMessagesLatest(): CancelablePromise<MaintenanceMessage> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/maintenance_messages/latest",
		});
	}

	/**
	 * @param requestBody
	 * @returns any
	 * @throws ApiError
	 */
	public static postAdminMaintenanceMessages(requestBody: MaintenanceMessage): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/admin/maintenance_messages",
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @returns PagedResponse_Guide
	 * @throws ApiError
	 */
	public static getGuidesList(page: number, pageSize?: number): CancelablePromise<PagedResponse_Guide> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/guides/list",
			query: {
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param id
	 * @returns Guide
	 * @throws ApiError
	 */
	public static getAdminGuides(id: string): CancelablePromise<Guide> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/guides/{id}",
			path: {
				id: id,
			},
		});
	}

	/**
	 * @param id
	 * @returns any
	 * @throws ApiError
	 */
	public static deleteAdminGuides(id: string): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "DELETE",
			url: "/admin/guides/{id}",
			path: {
				id: id,
			},
		});
	}

	/**
	 * @param requestBody
	 * @returns any
	 * @throws ApiError
	 */
	public static postAdminGuides(requestBody: Guide): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/admin/guides",
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @returns PagedResponse_SupportQuestion
	 * @throws ApiError
	 */
	public static getAdminSupportQuestionsList(
		page: number,
		pageSize?: number,
	): CancelablePromise<PagedResponse_SupportQuestion> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/support_questions/list",
			query: {
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param id
	 * @returns SupportQuestion
	 * @throws ApiError
	 */
	public static getAdminSupportQuestions(id: string): CancelablePromise<SupportQuestion> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/support_questions/{id}",
			path: {
				id: id,
			},
		});
	}

	/**
	 * @param id
	 * @returns any
	 * @throws ApiError
	 */
	public static deleteAdminSupportQuestions(id: string): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "DELETE",
			url: "/admin/support_questions/{id}",
			path: {
				id: id,
			},
		});
	}

	/**
	 * @param requestBody
	 * @returns any
	 * @throws ApiError
	 */
	public static postSupportQuestions(requestBody: string): CancelablePromise<any> {
		return __request(OpenAPI, {
			method: "POST",
			url: "/support_questions",
			body: requestBody,
			mediaType: "application/json; charset=utf-8",
		});
	}
}
