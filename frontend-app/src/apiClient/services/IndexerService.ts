/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Agent } from "../models/Agent";
import type { ListenerBlock } from "../models/ListenerBlock";
import type { Market } from "../models/Market";
import type { PagedResponse_Agent } from "../models/PagedResponse_Agent";
import type { PagedResponse_ExchangeRecord } from "../models/PagedResponse_ExchangeRecord";
import type { PagedResponse_ForestProjectContract } from "../models/PagedResponse_ForestProjectContract";
import type { PagedResponse_InvestmentRecord } from "../models/PagedResponse_InvestmentRecord";
import type { PagedResponse_InvestorUser } from "../models/PagedResponse_InvestorUser";
import type { PagedResponse_Market } from "../models/PagedResponse_Market";
import type { PagedResponse_Token } from "../models/PagedResponse_Token";
import type { PagedResponse_TokenHolderUser } from "../models/PagedResponse_TokenHolderUser";
import type { PagedResponse_TokenHolderUserBalanceUpdate } from "../models/PagedResponse_TokenHolderUserBalanceUpdate";
import type { PagedResponse_TraderUser } from "../models/PagedResponse_TraderUser";
import type { PagedResponse_UserYieldDistribution } from "../models/PagedResponse_UserYieldDistribution";
import type { PagedResponse_Yield } from "../models/PagedResponse_Yield";
import type { SecurityMintFund } from "../models/SecurityMintFund";
import type { SecurityTokenContractType } from "../models/SecurityTokenContractType";
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
			url: "/admin/indexer/contract-exists",
			query: {
				contract_address: contractAddress,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @returns TokenContract
	 * @throws ApiError
	 */
	public static getAdminIndexerTokenContract(contractAddress: string): CancelablePromise<TokenContract> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/token-contract",
			query: {
				contract_address: contractAddress,
			},
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @param projectId
	 * @param contractType
	 * @returns PagedResponse_ForestProjectContract
	 * @throws ApiError
	 */
	public static getAdminIndexerFpTokenContracts(
		page: number,
		pageSize: number,
		projectId?: string,
		contractType?: SecurityTokenContractType,
	): CancelablePromise<PagedResponse_ForestProjectContract> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/fp-token-contracts",
			query: {
				project_id: projectId,
				contract_type: contractType,
				page: page,
				page_size: pageSize,
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
	public static getAdminIndexerAgents(
		contractAddress: string,
		page: number,
		pageSize: number,
	): CancelablePromise<PagedResponse_Agent> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/agents",
			query: {
				contract_address: contractAddress,
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @param contractAddress
	 * @returns PagedResponse_Token
	 * @throws ApiError
	 */
	public static getAdminIndexerTokens(
		page: number,
		pageSize: number,
		contractAddress?: string,
	): CancelablePromise<PagedResponse_Token> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/tokens",
			query: {
				contract_address: contractAddress,
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
	public static getAdminIndexerToken(contractAddress: string, tokenId: string): CancelablePromise<Token> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/token",
			query: {
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
	public static getAdminIndexerMarket(contractAddress: string, tokenId?: string): CancelablePromise<Market> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/market",
			query: {
				contract_address: contractAddress,
				token_id: tokenId,
			},
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @param contractAddress
	 * @param tokenId
	 * @returns PagedResponse_Market
	 * @throws ApiError
	 */
	public static getAdminIndexerMarkets(
		page: number,
		pageSize: number,
		contractAddress?: string,
		tokenId?: string,
	): CancelablePromise<PagedResponse_Market> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/markets",
			query: {
				contract_address: contractAddress,
				token_id: tokenId,
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param contractAddress
	 * @param tokenId
	 * @returns SecurityMintFund
	 * @throws ApiError
	 */
	public static getAdminIndexerFund(contractAddress: string, tokenId: string): CancelablePromise<SecurityMintFund> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/fund",
			query: {
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
	public static getAdminIndexerFunds(
		investmentTokenContractAddress: string,
	): CancelablePromise<Array<SecurityMintFund>> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/funds",
			query: {
				investment_token_contract_address: investmentTokenContractAddress,
			},
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @param investmentTokenContract
	 * @param investmentTokenId
	 * @param investor
	 * @returns PagedResponse_InvestmentRecord
	 * @throws ApiError
	 */
	public static getAdminIndexerInvestmentRecords(
		page: number,
		pageSize: number,
		investmentTokenContract?: string,
		investmentTokenId?: string,
		investor?: string,
	): CancelablePromise<PagedResponse_InvestmentRecord> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/investment-records",
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
	 * @param pageSize
	 * @param tokenContractAddress
	 * @param tokenId
	 * @param buyer
	 * @param seller
	 * @returns PagedResponse_ExchangeRecord
	 * @throws ApiError
	 */
	public static getAdminIndexerExchangeRecords(
		page: number,
		pageSize: number,
		tokenContractAddress?: string,
		tokenId?: string,
		buyer?: string,
		seller?: string,
	): CancelablePromise<PagedResponse_ExchangeRecord> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/exchange-records",
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
	public static getAdminIndexerAgent(
		contractAddress: string,
		agentAddress: string,
		isContract: boolean,
	): CancelablePromise<Agent> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/agent",
			query: {
				contract_address: contractAddress,
				agent_address: agentAddress,
				is_contract: isContract,
			},
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @param tokenContractAddress
	 * @param tokenId
	 * @param yieldedTokenContractAddress
	 * @param yieldedTokenId
	 * @param yieldType
	 * @returns PagedResponse_Yield
	 * @throws ApiError
	 */
	public static getAdminIndexerYields(
		page: number,
		pageSize: number,
		tokenContractAddress?: string,
		tokenId?: string,
		yieldedTokenContractAddress?: string,
		yieldedTokenId?: string,
		yieldType?: YieldType,
	): CancelablePromise<PagedResponse_Yield> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/yields",
			query: {
				token_contract_address: tokenContractAddress,
				token_id: tokenId,
				yielded_token_contract_address: yieldedTokenContractAddress,
				yielded_token_id: yieldedTokenId,
				yield_type: yieldType,
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @param forestProjectId
	 * @param tokenContractAddress
	 * @param toAddress
	 * @param yieldedTokenContractAddress
	 * @param yieldedTokenId
	 * @returns PagedResponse_UserYieldDistribution
	 * @throws ApiError
	 */
	public static getAdminIndexerYieldDistributions(
		page: number,
		pageSize: number,
		forestProjectId?: string,
		tokenContractAddress?: string,
		toAddress?: string,
		yieldedTokenContractAddress?: string,
		yieldedTokenId?: string,
	): CancelablePromise<PagedResponse_UserYieldDistribution> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/yield-distributions",
			query: {
				forest_project_id: forestProjectId,
				token_contract_address: tokenContractAddress,
				to_address: toAddress,
				yielded_token_contract_address: yieldedTokenContractAddress,
				yielded_token_id: yieldedTokenId,
				page: page,
				page_size: pageSize,
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
	public static getAdminIndexerYieldedTokens(
		tokenContractAddress: string,
		page: number,
		pageSize: number,
	): CancelablePromise<PagedResponse_Token> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/yielded-tokens",
			query: {
				token_contract_address: tokenContractAddress,
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @param forestProjectId
	 * @param contractAddress
	 * @param tokenId
	 * @param holderAddress
	 * @returns PagedResponse_TokenHolderUser
	 * @throws ApiError
	 */
	public static getAdminIndexerHolders(
		page: number,
		pageSize: number,
		forestProjectId?: string,
		contractAddress?: string,
		tokenId?: string,
		holderAddress?: string,
	): CancelablePromise<PagedResponse_TokenHolderUser> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/holders",
			query: {
				forest_project_id: forestProjectId,
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
	public static getAdminIndexerHolder(
		contractAddress: string,
		tokenId: string,
		holderAddress: string,
	): CancelablePromise<TokenHolderUser> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/holder",
			query: {
				contract_address: contractAddress,
				token_id: tokenId,
				holder_address: holderAddress,
			},
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @param forestProjectId
	 * @param contractAddress
	 * @param tokenId
	 * @param holderAddress
	 * @param updateType
	 * @returns PagedResponse_TokenHolderUserBalanceUpdate
	 * @throws ApiError
	 */
	public static getAdminIndexerBalanceUpdates(
		page: number,
		pageSize: number,
		forestProjectId?: string,
		contractAddress?: string,
		tokenId?: string,
		holderAddress?: string,
		updateType?: TokenHolderBalanceUpdateType,
	): CancelablePromise<PagedResponse_TokenHolderUserBalanceUpdate> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/balance-updates",
			query: {
				forest_project_id: forestProjectId,
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
	 * @returns Treasury
	 * @throws ApiError
	 */
	public static getAdminIndexerTreasury(): CancelablePromise<Treasury> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/treasury",
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @param forestProjectId
	 * @param investmentContractAddress
	 * @param investmentTokenId
	 * @param investor
	 * @returns PagedResponse_InvestorUser
	 * @throws ApiError
	 */
	public static getAdminIndexerInvestors(
		page: number,
		pageSize: number,
		forestProjectId?: string,
		investmentContractAddress?: string,
		investmentTokenId?: string,
		investor?: string,
	): CancelablePromise<PagedResponse_InvestorUser> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/investors",
			query: {
				forest_project_id: forestProjectId,
				investment_contract_address: investmentContractAddress,
				investment_token_id: investmentTokenId,
				investor: investor,
				page: page,
				page_size: pageSize,
			},
		});
	}

	/**
	 * @param page
	 * @param pageSize
	 * @param forestProjectId
	 * @param tokenContractAddress
	 * @param tokenId
	 * @param trader
	 * @returns PagedResponse_TraderUser
	 * @throws ApiError
	 */
	public static getAdminIndexerTraders(
		page: number,
		pageSize: number,
		forestProjectId?: string,
		tokenContractAddress?: string,
		tokenId?: string,
		trader?: string,
	): CancelablePromise<PagedResponse_TraderUser> {
		return __request(OpenAPI, {
			method: "GET",
			url: "/admin/indexer/traders",
			query: {
				forest_project_id: forestProjectId,
				token_contract_address: tokenContractAddress,
				token_id: tokenId,
				trader: trader,
				page: page,
				page_size: pageSize,
			},
		});
	}
}
