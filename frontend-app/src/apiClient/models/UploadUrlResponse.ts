/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

/**
 * Represents the response from an IPFS file upload request, containing the
 * presigned URL for uploading the file and the generated file name.
 */
export type UploadUrlResponse = {
	presigned_url: string;
	file_name: string;
};
