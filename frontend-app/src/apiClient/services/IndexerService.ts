/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Agent } from "../models/Agent";
import type { ListenerBlock } from "../models/ListenerBlock";
import type { Market } from "../models/Market";
import type { PagedResponse_Agent } from "../models/PagedResponse_Agent";
import type { PagedResponse_ExchangeRecord } from "../models/PagedResponse_ExchangeRecord";
import type { PagedResponse_InvestmentRecord } from "../models/PagedResponse_InvestmentRecord";
import type { PagedResponse_Token } from "../models/PagedResponse_Token";
import type { PagedResponse_TokenHolderUser } from "../models/PagedResponse_TokenHolderUser";
import type { PagedResponse_TokenHolderUserBalanceUpdate } from "../models/PagedResponse_TokenHolderUserBalanceUpdate";
import type { PagedResponse_Yield } from "../models/PagedResponse_Yield";
import type { SecurityMintFund } from "../models/SecurityMintFund";
import type { Token } from "../models/Token";
import type { TokenContract } from "../models/TokenContract";
import type { TokenHolderBalanceUpdateType } from "../models/TokenHolderBalanceUpdateType";
import type { TokenHolderUser } from "../models/TokenHolderUser";
import type { Treasury } from "../models/Treasury";
import type { YieldType } from "../models/YieldType";

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
	 * @returns TokenContract
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2(contractAddress: string): CancelablePromise<TokenContract> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}",
			path: {
				contract_address: contractAddress,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @param page
	 * @param pageSize
	 * @returns PagedResponse_Agent
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2AgentList(
		contractAddress: string,
		page: number,
		pageSize?: number,
	): CancelablePromise<PagedResponse_Agent> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}/agent/list",
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
	 * @param page
	 * @param pageSize
	 * @returns PagedResponse_Token
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2TokenList(
		contractAddress: string,
		page?: number,
		pageSize?: number,
	): CancelablePromise<PagedResponse_Token> {
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
	public static getAdminIndexerCis2Market(contractAddress: string, tokenId?: string): CancelablePromise<Market> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}/market",
			path: {
				contract_address: contractAddress,
			},
			query: {
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
	 * @param investmentTokenContractAddress
	 * @returns SecurityMintFund
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2FundList(
		investmentTokenContractAddress: string,
	): CancelablePromise<Array<SecurityMintFund>> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/fund/list",
			query: {
				investment_token_contract_address: investmentTokenContractAddress,
			},
		});
	}

	/**
	 * @param page
	 * @param investmentTokenContract
	 * @param investmentTokenId
	 * @param investor
	 * @param pageSize
	 * @returns PagedResponse_InvestmentRecord
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2FundInvestmentRecordsList(
		page: number,
		investmentTokenContract?: string,
		investmentTokenId?: string,
		investor?: string,
		pageSize?: number,
	): CancelablePromise<PagedResponse_InvestmentRecord> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/fund/investment-records/list",
			query: {
				investment_token_contract: investmentTokenContract,
				investment_token_id: investmentTokenId,
				investor: investor,
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param page
	 * @param tokenContractAddress
	 * @param tokenId
	 * @param buyer
	 * @param seller
	 * @param pageSize
	 * @returns PagedResponse_ExchangeRecord
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2FundMarketRecordsList(
		page: number,
		tokenContractAddress?: string,
		tokenId?: string,
		buyer?: string,
		seller?: string,
		pageSize?: number,
	): CancelablePromise<PagedResponse_ExchangeRecord> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/fund/market-records/list",
			query: {
				token_contract_address: tokenContractAddress,
				token_id: tokenId,
				buyer: buyer,
				seller: seller,
				page: page,
				page_size: pageSize,
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
	 * @param page
	 * @param pageSize
	 * @param tokenContractAddress
	 * @param tokenId
	 * @param yieldTokenContractAddress
	 * @param yieldTokenId
	 * @param yieldType
	 * @returns PagedResponse_Yield
	 * @throws ApiError
	 */
	public static getAdminIndexerYieldList(
		page: number,
		pageSize?: number,
		tokenContractAddress?: string,
		tokenId?: string,
		yieldTokenContractAddress?: string,
		yieldTokenId?: string,
		yieldType?: YieldType,
	): CancelablePromise<PagedResponse_Yield> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/yield/list",
			query: {
				page: page,
				page_size: pageSize,
				token_contract_address: tokenContractAddress,
				token_id: tokenId,
				yield_token_contract_address: yieldTokenContractAddress,
				yield_token_id: yieldTokenId,
				yield_type: yieldType,
			},
		});
	}

	/**
	 * @param tokenContractAddress
	 * @param page
	 * @param pageSize
	 * @returns PagedResponse_Token
	 * @throws ApiError
	 */
	public static getAdminIndexerYieldTokens(
		tokenContractAddress: string,
		page: number,
		pageSize?: number,
	): CancelablePromise<PagedResponse_Token> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/yield/{token_contract_address}/tokens",
			path: {
				token_contract_address: tokenContractAddress,
			},
			query: {
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param page
	 * @param contractAddress
	 * @param tokenId
	 * @param holderAddress
	 * @param pageSize
	 * @returns PagedResponse_TokenHolderUser
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2HolderList(
		page: number,
		contractAddress?: string,
		tokenId?: string,
		holderAddress?: string,
		pageSize?: number,
	): CancelablePromise<PagedResponse_TokenHolderUser> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/holder/list",
			query: {
				contract_address: contractAddress,
				token_id: tokenId,
				holder_address: holderAddress,
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @param tokenId
	 * @param holderAddress
	 * @returns TokenHolderUser
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2TokenHolder(
		contractAddress: string,
		tokenId: string,
		holderAddress: string,
	): CancelablePromise<TokenHolderUser> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/{contract_address}/token/{token_id}/holder/{holder_address}",
			path: {
				contract_address: contractAddress,
				token_id: tokenId,
				holder_address: holderAddress,
			},
		});
	}

	/**
	 * @param page
	 * @param contractAddress
	 * @param tokenId
	 * @param holderAddress
	 * @param updateType
	 * @param pageSize
	 * @returns PagedResponse_TokenHolderUserBalanceUpdate
	 * @throws ApiError
	 */
	public static getAdminIndexerCis2BalanceUpdatesList(
		page: number,
		contractAddress?: string,
		tokenId?: string,
		holderAddress?: string,
		updateType?: TokenHolderBalanceUpdateType,
		pageSize?: number,
	): CancelablePromise<PagedResponse_TokenHolderUserBalanceUpdate> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/cis2/balance-updates/list",
			query: {
				contract_address: contractAddress,
				token_id: tokenId,
				holder_address: holderAddress,
				update_type: updateType,
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
