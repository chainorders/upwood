/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { AddMetadataRequest } from '../models/AddMetadataRequest';
import type { MintData } from '../models/MintData';
import type { PagedResponse_TokenHolder_ } from '../models/PagedResponse_TokenHolder_';
import type { TreeNftMetadata } from '../models/TreeNftMetadata';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class TreeNftMetadataService {

    /**
     * Retrieves a random metadata entry and generates a signed metadata object for minting a new NFT.
     * # Arguments
     * - `claims`: The authenticated account claims.
     * - `db_pool`: The database connection pool.
     * - `config`: The TreeNftConfig instance.
     * - `contract_index`: The index of the contract to retrieve the metadata for.
     *
     * # Returns
     * A `MintData` object containing the signed metadata and the signer's address and signature.
     * @returns MintData
     * @throws ApiError
     */
    public static getTreeNftMetadataRandom(): CancelablePromise<MintData> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/tree_nft/metadata/random',
        });
    }

    /**
     * Inserts a new TreeNftMetadata record in the database.
     * This endpoint is only accessible to administrators.
     *
     * # Arguments
     * - `claims`: The authenticated user's claims, used to ensure the user is an admin.
     * - `db_pool`: A reference to the database connection pool.
     * - `req`: The request body containing the metadata to be inserted.
     *
     * # Returns
     * The newly inserted `TreeNftMetadata` record.
     * @param requestBody
     * @returns TreeNftMetadata
     * @throws ApiError
     */
    public static postAdminTreeNftMetadata(
        requestBody: AddMetadataRequest,
    ): CancelablePromise<TreeNftMetadata> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/tree_nft/metadata',
            body: requestBody,
            mediaType: 'application/json; charset=utf-8',
        });
    }

    /**
     * Lists all TreeNftMetadata records in the database, paginated by the given page number.
     * This endpoint is only accessible to administrators.
     *
     * # Arguments
     * - `claims`: The authenticated user's claims, used to ensure the user is an admin.
     * - `db_pool`: A reference to the database connection pool.
     * - `page`: The page number to retrieve, starting from 0.
     *
     * # Returns
     * A vector of `TreeNftMetadata` records for the given page.
     * @param page
     * @returns TreeNftMetadata
     * @throws ApiError
     */
    public static getAdminTreeNftMetadataList(
        page: number,
    ): CancelablePromise<Array<TreeNftMetadata>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/tree_nft/metadata/list/{page}',
            query: {
                'page': page,
            },
        });
    }

    /**
     * Retrieves a TreeNftMetadata record from the database by its ID.
     * This endpoint is only accessible to administrators.
     *
     * # Arguments
     * - `claims`: The authenticated user's claims, used to ensure the user is an admin.
     * - `db_pool`: A reference to the database connection pool.
     * - `id`: The ID of the TreeNftMetadata record to retrieve.
     *
     * # Returns
     * The requested TreeNftMetadata record, or a NotFound error if the record is not found.
     * @param id
     * @returns TreeNftMetadata
     * @throws ApiError
     */
    public static getAdminTreeNftMetadata(
        id: string,
    ): CancelablePromise<TreeNftMetadata> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/tree_nft/metadata/{id}',
            path: {
                'id': id,
            },
        });
    }

    /**
     * Deletes a TreeNftMetadata record from the database by its ID.
     * This endpoint is only accessible to administrators.
     *
     * # Arguments
     * - `claims`: The authenticated user's claims, used to ensure the user is an admin.
     * - `db_pool`: A reference to the database connection pool.
     * - `id`: The ID of the TreeNftMetadata record to delete.
     *
     * # Returns
     * A NoResResult indicating success or failure of the deletion.
     * @param id
     * @returns any
     * @throws ApiError
     */
    public static deleteAdminTreeNftMetadata(
        id: string,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/admin/tree_nft/metadata/{id}',
            path: {
                'id': id,
            },
        });
    }

    /**
     * Lists the owners of the NFT with the given metadata ID for the specified contract.
     * This endpoint is only accessible to admin users.
     *
     * # Parameters
     * - `claims`: The authenticated user's claims.
     * - `db_pool`: The database connection pool.
     * - `contract_index`: The index of the contract to list owners for.
     * - `metadata_id`: The ID of the metadata to list owners for.
     * - `page`: The page number to retrieve (optional).
     *
     * # Returns
     * A paged response containing the list of token holders for the specified metadata.
     * @param metadataId
     * @param page
     * @returns PagedResponse_TokenHolder_
     * @throws ApiError
     */
    public static getAdminTreeNftMetadataOwners(
        metadataId: string,
        page: number,
    ): CancelablePromise<PagedResponse_TokenHolder_> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/admin/tree_nft/metadata/{metadata_id}/owners/{page}',
            path: {
                'metadata_id': metadataId,
            },
            query: {
                'page': page,
            },
        });
    }

}
