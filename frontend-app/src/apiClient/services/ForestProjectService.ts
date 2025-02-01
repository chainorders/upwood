/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ForestProject } from '../models/ForestProject';
import type { ForestProjectAggApiModel } from '../models/ForestProjectAggApiModel';
import type { ForestProjectMedia } from '../models/ForestProjectMedia';
import type { ForestProjectPrice } from '../models/ForestProjectPrice';
import type { ForestProjectState } from '../models/ForestProjectState';
import type { ForestProjectTokenContract } from '../models/ForestProjectTokenContract';
import type { ForestProjectTokenUserYield } from '../models/ForestProjectTokenUserYield';
import type { ForestProjectTokenYieldListApiModel } from '../models/ForestProjectTokenYieldListApiModel';
import type { PagedResponse_ForestProject_ } from '../models/PagedResponse_ForestProject_';
import type { PagedResponse_ForestProjectAggApiModel_ } from '../models/PagedResponse_ForestProjectAggApiModel_';
import type { PagedResponse_ForestProjectFundInvestor_ } from '../models/PagedResponse_ForestProjectFundInvestor_';
import type { PagedResponse_ForestProjectMedia_ } from '../models/PagedResponse_ForestProjectMedia_';
import type { PagedResponse_ForestProjectPrice_ } from '../models/PagedResponse_ForestProjectPrice_';
import type { PagedResponse_ForestProjectTokenContractAggApiModel_ } from '../models/PagedResponse_ForestProjectTokenContractAggApiModel_';
import type { PagedResponse_TokenMetadata_ } from '../models/PagedResponse_TokenMetadata_';
import type { SecurityTokenContractType } from '../models/SecurityTokenContractType';
import type { TokenMetadata } from '../models/TokenMetadata';
import type { UserYieldsAggregate } from '../models/UserYieldsAggregate';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class ForestProjectService {

    /**
     * @param state
     * @param page
     * @returns PagedResponse_ForestProjectAggApiModel_
     * @throws ApiError
     */
    public static getForestProjectsList(
        state: ForestProjectState,
        page: number,
    ): CancelablePromise<PagedResponse_ForestProjectAggApiModel_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/forest_projects/list/{state}/{page}',
            path: {
                'state': state,
                'page': page,
            },
        });
    }

    /**
     * @param projectId
     * @returns ForestProjectAggApiModel
     * @throws ApiError
     */
    public static getForestProjects(
        projectId: string,
    ): CancelablePromise<ForestProjectAggApiModel> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/forest_projects/{project_id}',
            path: {
                'project_id': projectId,
            },
        });
    }

    /**
     * @returns PagedResponse_ForestProjectAggApiModel_
     * @throws ApiError
     */
    public static getForestProjectsListOwned(): CancelablePromise<PagedResponse_ForestProjectAggApiModel_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/forest_projects/list/owned',
        });
    }

    /**
     * @returns PagedResponse_ForestProjectTokenContractAggApiModel_
     * @throws ApiError
     */
    public static getForestProjectsContractListOwned(): CancelablePromise<PagedResponse_ForestProjectTokenContractAggApiModel_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/forest_projects/contract/list/owned',
        });
    }

    /**
     * @param projectId
     * @param page
     * @returns PagedResponse_ForestProjectMedia_
     * @throws ApiError
     */
    public static getForestProjectsMediaList(
        projectId: string,
        page: number,
    ): CancelablePromise<PagedResponse_ForestProjectMedia_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/forest_projects/{project_id}/media/list/{page}',
            path: {
                'project_id': projectId,
                'page': page,
            },
        });
    }

    /**
     * @param projectId
     * @param mediaId
     * @returns ForestProjectMedia
     * @throws ApiError
     */
    public static getForestProjectsMedia(
        projectId: string,
        mediaId: string,
    ): CancelablePromise<ForestProjectMedia> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/forest_projects/{project_id}/media/{media_id}',
            path: {
                'project_id': projectId,
                'media_id': mediaId,
            },
        });
    }

    /**
     * @param projectId
     * @returns ForestProjectTokenContract
     * @throws ApiError
     */
    public static getForestProjectsContractList(
        projectId: string,
    ): CancelablePromise<Array<ForestProjectTokenContract>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/forest_projects/{project_id}/contract/list',
            path: {
                'project_id': projectId,
            },
        });
    }

    /**
     * @returns UserYieldsAggregate
     * @throws ApiError
     */
    public static getForestProjectsYieldsTotal(): CancelablePromise<Array<UserYieldsAggregate>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/forest_projects/yields/total',
        });
    }

    /**
     * @returns ForestProjectTokenUserYield
     * @throws ApiError
     */
    public static getForestProjectsYieldsClaimable(): CancelablePromise<Array<ForestProjectTokenUserYield>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/forest_projects/yields/claimable',
        });
    }

    /**
     * Finds a forest project by its ID.
     * Only admins can access this endpoint.
     * # Arguments
     * - `claims`: The claims of the authenticated user.
     * - `db_pool`: The database connection pool.
     * - `project_id`: The ID of the forest project to find.
     *
     * # Returns
     * The forest project with the given ID, or an error if the project is not found.
     * @param projectId
     * @returns ForestProject
     * @throws ApiError
     */
    public static getAdminForestProjects(
        projectId: string,
    ): CancelablePromise<ForestProject> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/forest_projects/{project_id}',
            path: {
                'project_id': projectId,
            },
        });
    }

    /**
     * @param page
     * @param state
     * @returns PagedResponse_ForestProject_
     * @throws ApiError
     */
    public static getAdminForestProjectsList(
        page: number,
        state?: ForestProjectState,
    ): CancelablePromise<PagedResponse_ForestProject_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/forest_projects/list/{page}',
            path: {
                'page': page,
            },
            query: {
                'state': state,
            },
        });
    }

    /**
     * @param requestBody
     * @returns ForestProject
     * @throws ApiError
     */
    public static postAdminForestProjects(
        requestBody: ForestProject,
    ): CancelablePromise<ForestProject> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/forest_projects',
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * @param requestBody
     * @returns ForestProject
     * @throws ApiError
     */
    public static putAdminForestProjects(
        requestBody: ForestProject,
    ): CancelablePromise<ForestProject> {
        return __request(OpenAPI, {
            method: 'PUT',
            url: '/admin/forest_projects',
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * @param projectId
     * @param requestBody
     * @returns ForestProjectMedia
     * @throws ApiError
     */
    public static postAdminForestProjectsMedia(
        projectId: string,
        requestBody: ForestProjectMedia,
    ): CancelablePromise<ForestProjectMedia> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/forest_projects/{project_id}/media',
            path: {
                'project_id': projectId,
            },
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * @param projectId
     * @param mediaId
     * @returns ForestProjectMedia
     * @throws ApiError
     */
    public static deleteAdminForestProjectsMedia(
        projectId: string,
        mediaId: string,
    ): CancelablePromise<ForestProjectMedia> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/admin/forest_projects/{project_id}/media/{media_id}',
            path: {
                'project_id': projectId,
                'media_id': mediaId,
            },
        });
    }

    /**
     * @param projectId
     * @returns ForestProjectPrice
     * @throws ApiError
     */
    public static getAdminForestProjectsPriceLatest(
        projectId: string,
    ): CancelablePromise<ForestProjectPrice> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/forest_projects/{project_id}/price/latest',
            path: {
                'project_id': projectId,
            },
        });
    }

    /**
     * @param projectId
     * @param priceAt
     * @returns ForestProjectPrice
     * @throws ApiError
     */
    public static getAdminForestProjectsPrice(
        projectId: string,
        priceAt: string,
    ): CancelablePromise<ForestProjectPrice> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/forest_projects/{project_id}/price/{price_at}',
            path: {
                'project_id': projectId,
                'price_at': priceAt,
            },
        });
    }

    /**
     * @param projectId
     * @param priceAt
     * @returns any
     * @throws ApiError
     */
    public static deleteAdminForestProjectsPrice(
        projectId: string,
        priceAt: string,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/admin/forest_projects/{project_id}/price/{price_at}',
            path: {
                'project_id': projectId,
                'price_at': priceAt,
            },
        });
    }

    /**
     * @param projectId
     * @param page
     * @returns PagedResponse_ForestProjectPrice_
     * @throws ApiError
     */
    public static getAdminForestProjectsPriceList(
        projectId: string,
        page: number,
    ): CancelablePromise<PagedResponse_ForestProjectPrice_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/forest_projects/{project_id}/price/list/{page}',
            path: {
                'project_id': projectId,
                'page': page,
            },
        });
    }

    /**
     * @param projectId
     * @param requestBody
     * @returns ForestProjectPrice
     * @throws ApiError
     */
    public static postAdminForestProjectsPrice(
        projectId: string,
        requestBody: ForestProjectPrice,
    ): CancelablePromise<ForestProjectPrice> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/forest_projects/{project_id}/price',
            path: {
                'project_id': projectId,
            },
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * @param projectId
     * @param page
     * @returns PagedResponse_ForestProjectFundInvestor_
     * @throws ApiError
     */
    public static getAdminForestProjectsFundInvestorList(
        projectId: string,
        page: number,
    ): CancelablePromise<PagedResponse_ForestProjectFundInvestor_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/forest_projects/{project_id}/fund/investor/list/{page}',
            path: {
                'project_id': projectId,
                'page': page,
            },
        });
    }

    /**
     * @param contractAddress
     * @returns ForestProjectTokenContract
     * @throws ApiError
     */
    public static getAdminForestProjectsContract(
        contractAddress: string,
    ): CancelablePromise<ForestProjectTokenContract> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/forest_projects/contract/{contract_address}',
            path: {
                'contract_address': contractAddress,
            },
        });
    }

    /**
     * @param projectId
     * @param contractType
     * @returns ForestProjectTokenContract
     * @throws ApiError
     */
    public static getAdminForestProjectsContractByType(
        projectId: string,
        contractType: SecurityTokenContractType,
    ): CancelablePromise<ForestProjectTokenContract> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/forest_projects/{project_id}/contract_by_type/{contract_type}',
            path: {
                'project_id': projectId,
                'contract_type': contractType,
            },
        });
    }

    /**
     * @param requestBody
     * @returns ForestProjectTokenContract
     * @throws ApiError
     */
    public static postAdminForestProjectsContract(
        requestBody: ForestProjectTokenContract,
    ): CancelablePromise<ForestProjectTokenContract> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/forest_projects/contract',
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * @param requestBody
     * @returns ForestProjectTokenContract
     * @throws ApiError
     */
    public static putAdminForestProjectsContract(
        requestBody: ForestProjectTokenContract,
    ): CancelablePromise<ForestProjectTokenContract> {
        return __request(OpenAPI, {
            method: 'PUT',
            url: '/admin/forest_projects/contract',
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * @param projectId
     * @param contractType
     * @returns any
     * @throws ApiError
     */
    public static deleteAdminForestProjectsContract(
        projectId: string,
        contractType: SecurityTokenContractType,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/admin/forest_projects/{project_id}/contract/{contract_type}',
            path: {
                'project_id': projectId,
                'contract_type': contractType,
            },
        });
    }

    /**
     * @param contractAddress
     * @param tokenId
     * @returns ForestProjectTokenYieldListApiModel
     * @throws ApiError
     */
    public static getAdminForestProjectsContractTokenYeildList(
        contractAddress: string,
        tokenId: string,
    ): CancelablePromise<Array<ForestProjectTokenYieldListApiModel>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/forest_projects/{project_id}/contract/{contract_address}/token/{token_id}/yeild/list',
            path: {
                'contract_address': contractAddress,
                'token_id': tokenId,
            },
        });
    }

    /**
     * @param requestBody
     * @returns TokenMetadata
     * @throws ApiError
     */
    public static postAdminTokenMetadata(
        requestBody: TokenMetadata,
    ): CancelablePromise<TokenMetadata> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/token_metadata',
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * @param requestBody
     * @returns TokenMetadata
     * @throws ApiError
     */
    public static putAdminTokenMetadata(
        requestBody: TokenMetadata,
    ): CancelablePromise<TokenMetadata> {
        return __request(OpenAPI, {
            method: 'PUT',
            url: '/admin/token_metadata',
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * @param contractAddress
     * @param tokenId
     * @returns TokenMetadata
     * @throws ApiError
     */
    public static getAdminTokenMetadata(
        contractAddress: string,
        tokenId: string,
    ): CancelablePromise<TokenMetadata> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/token_metadata/{contract_address}/{token_id}',
            path: {
                'contract_address': contractAddress,
                'token_id': tokenId,
            },
        });
    }

    /**
     * @param contractAddress
     * @param tokenId
     * @returns any
     * @throws ApiError
     */
    public static deleteAdminTokenMetadata(
        contractAddress: string,
        tokenId: string,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/admin/token_metadata/{contract_address}/{token_id}',
            path: {
                'contract_address': contractAddress,
                'token_id': tokenId,
            },
        });
    }

    /**
     * @param page
     * @returns PagedResponse_TokenMetadata_
     * @throws ApiError
     */
    public static getAdminTokenMetadataList(
        page: number,
    ): CancelablePromise<PagedResponse_TokenMetadata_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/token_metadata/list/{page}',
            path: {
                'page': page,
            },
        });
    }

}
