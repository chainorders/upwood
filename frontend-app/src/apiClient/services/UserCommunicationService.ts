/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Guide } from "../models/Guide";
import type { MaintenanceMessage } from "../models/MaintenanceMessage";
import type { NewsArticle } from "../models/NewsArticle";
import type { PagedResponse_Guide_ } from "../models/PagedResponse_Guide_";
import type { PagedResponse_MaintenanceMessage_ } from "../models/PagedResponse_MaintenanceMessage_";
import type { PagedResponse_NewsArticle_ } from "../models/PagedResponse_NewsArticle_";
import type { PagedResponse_PlatformUpdate_ } from "../models/PagedResponse_PlatformUpdate_";
import type { PagedResponse_SupportQuestion_ } from "../models/PagedResponse_SupportQuestion_";
import type { PlatformUpdate } from "../models/PlatformUpdate";
import type { SupportQuestion } from "../models/SupportQuestion";

import type { CancelablePromise } from "../core/CancelablePromise";
import { OpenAPI } from "../core/OpenAPI";
import { request as __request } from "../core/request";

export class UserCommunicationService {
	/**
	 * @param page
	 * @returns PagedResponse_NewsArticle_
	 * @throws ApiError
	 */
	public static getNewsArticlesList(page: number): CancelablePromise<PagedResponse_NewsArticle_> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/news_articles/list/{page}",
			path: {
				page: page,
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
	 * @returns PagedResponse_PlatformUpdate_
	 * @throws ApiError
	 */
	public static getPlatformUpdatesList(page: number): CancelablePromise<PagedResponse_PlatformUpdate_> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/platform_updates/list/{page}",
			path: {
				page: page,
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
	 * @returns PagedResponse_MaintenanceMessage_
	 * @throws ApiError
	 */
	public static getMaintenanceMessagesList(page: number): CancelablePromise<PagedResponse_MaintenanceMessage_> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/maintenance_messages/list/{page}",
			path: {
				page: page,
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
	 * @returns PagedResponse_Guide_
	 * @throws ApiError
	 */
	public static getGuidesList(page: number): CancelablePromise<PagedResponse_Guide_> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/guides/list/{page}",
			path: {
				page: page,
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
	 * @returns PagedResponse_SupportQuestion_
	 * @throws ApiError
	 */
	public static getAdminSupportQuestionsList(page: number): CancelablePromise<PagedResponse_SupportQuestion_> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/support_questions/list/{page}",
			path: {
				page: page,
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
