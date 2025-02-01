/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { UploadUrlResponse } from '../models/UploadUrlResponse';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class FilesService {

    /**
     * Create a presigned URL to upload a file to S3
     * Requires admin privileges
     *
     * # Arguments
     * * `BearerAuthorization(claims): BearerAuthorization` - The bearer token claims
     * * `Data(files_bucket): Data<&s3::FilesBucket>` - The S3 bucket for files
     *
     * # Returns
     * * `Json<UploadUrlResponse>` - The presigned URL to upload the file & file name
     * @returns UploadUrlResponse
     * @throws ApiError
     */
    public static postAdminFilesS3UploadUrl(): CancelablePromise<UploadUrlResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/files/s3/upload_url',
        });
    }

    /**
     * Delete a file from S3
     * Requires admin privileges
     *
     * # Arguments
     * * `BearerAuthorization(claims): BearerAuthorization` - The bearer token claims
     * * `Data(files_bucket): Data<&s3::FilesBucket>` - The S3 bucket for files
     * * `Path(file_name): Path<Uuid>` - The file name to delete
     *
     * # Returns
     * * `NoResResult` - The result of the operation
     * @param fileName
     * @returns any
     * @throws ApiError
     */
    public static deleteAdminFilesS3(
        fileName: string,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/admin/files/s3/{file_name}',
            path: {
                'file_name': fileName,
            },
        });
    }

    /**
     * Create a presigned URL to upload a file to IPFS
     * Requires admin privileges
     *
     * # Arguments
     * * `BearerAuthorization(claims): BearerAuthorization` - The bearer token claims
     * * `Data(files_bucket): Data<&ipfs::filebase::FilesBucket>` - The IPFS bucket for files
     *
     * # Returns
     * * `Json<UploadUrlResponse>` - The presigned URL to upload the file & file name
     * @returns UploadUrlResponse
     * @throws ApiError
     */
    public static postAdminFilesIpfsUploadUrl(): CancelablePromise<UploadUrlResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/admin/files/ipfs/upload_url',
        });
    }

    /**
     * Delete a file from IPFS
     * Requires admin privileges
     *
     * # Arguments
     * * `BearerAuthorization(claims): BearerAuthorization` - The bearer token claims
     * * `Data(files_bucket): Data<&ipfs::filebase::FilesBucket>` - The IPFS bucket for files
     * * `Path(file_name): Path<Uuid>` - The file name to delete
     *
     * # Returns
     * * `NoResResult` - The result of the operation
     * @param fileName
     * @returns any
     * @throws ApiError
     */
    public static deleteAdminFilesIpfs(
        fileName: string,
    ): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/admin/files/ipfs/{file_name}',
            path: {
                'file_name': fileName,
            },
        });
    }

}
