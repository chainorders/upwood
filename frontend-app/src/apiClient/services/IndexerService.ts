/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Agent } from "../models/Agent";
import type { ListenerBlock } from "../models/ListenerBlock";
import type { Market } from "../models/Market";
import type { PagedResponse_TokenHolder } from "../models/PagedResponse_TokenHolder";
import type { SecurityMintFund } from "../models/SecurityMintFund";
import type { Token } from "../models/Token";
import type { Treasury } from "../models/Treasury";
import type { Yield } from "../models/Yield";

import type { CancelablePromise } from "../core/CancelablePromise";
import { OpenAPI } from "../core/OpenAPI";
import { request as __request } from "../core/request";

export class IndexerService {
	/**
	 * @returns ListenerBlock
	 * @throws ApiError
	 */
	public static getAdminIndexerBlockLatest(): CancelablePromise<ListenerBlock> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/block/latest",
		});
	}

	/**
	 * @param contractAddress
	 * @returns boolean
	 * @throws ApiError
	 */
	public static getAdminIndexerContractExists(contractAddress: string): CancelablePromise<boolean> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/contract/{contract_address}/exists",
			path: {
				contract_address: contractAddress,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @param page
	 * @param pageSize
	 * @returns Token
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2TokenList(
		contractAddress: string,
		page?: number,
		pageSize?: number,
	): CancelablePromise<Array<Token>> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}/token/list",
			path: {
				contract_address: contractAddress,
			},
			query: {
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @param tokenId
	 * @returns Token
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2Token(contractAddress: string, tokenId: string): CancelablePromise<Token> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}/token/{token_id}",
			path: {
				contract_address: contractAddress,
				token_id: tokenId,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @param tokenId
	 * @returns Market
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2TokenMarket(contractAddress: string, tokenId: string): CancelablePromise<Market> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}/token/{token_id}/market",
			path: {
				contract_address: contractAddress,
				token_id: tokenId,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @returns Market
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2MarketList(contractAddress: string): CancelablePromise<Array<Market>> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}/market/list",
			path: {
				contract_address: contractAddress,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @param tokenId
	 * @returns SecurityMintFund
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2TokenFund(
		contractAddress: string,
		tokenId: string,
	): CancelablePromise<SecurityMintFund> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}/token/{token_id}/fund",
			path: {
				contract_address: contractAddress,
				token_id: tokenId,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @returns SecurityMintFund
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2FundList(contractAddress: string): CancelablePromise<Array<SecurityMintFund>> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}/fund/list",
			path: {
				contract_address: contractAddress,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @param agentAddress
	 * @param isContract Whether the agent_address is a contract or not
	 * @returns Agent
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2Agent(
		contractAddress: string,
		agentAddress: string,
		isContract: boolean,
	): CancelablePromise<Agent> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}/agent/{agent_address}",
			path: {
				contract_address: contractAddress,
				agent_address: agentAddress,
			},
			query: {
				is_contract: isContract,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @param tokenId
	 * @returns Yield
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2TokenYieldsList(
		contractAddress: string,
		tokenId: string,
	): CancelablePromise<Array<Yield>> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}/token/{token_id}/yields/list",
			path: {
				contract_address: contractAddress,
				token_id: tokenId,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @param tokenId
	 * @param page
	 * @param pageSize
	 * @returns PagedResponse_TokenHolder
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2TokenHolderList(
		contractAddress: string,
		tokenId: string,
		page?: number,
		pageSize?: number,
	): CancelablePromise<PagedResponse_TokenHolder> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}/token/{token_id}/holder/list",
			path: {
				contract_address: contractAddress,
				token_id: tokenId,
			},
			query: {
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @returns Treasury
	 * @throws ApiError
	 */
	public static getAdminIndexerYielderTreasury(contractAddress: string): CancelablePromise<Treasury> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/yielder/{contract_address}/treasury",
			path: {
				contract_address: contractAddress,
			},
		});
	}
}
