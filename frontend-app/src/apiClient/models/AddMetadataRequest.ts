/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { MetadataUrl } from "./MetadataUrl";

export type AddMetadataRequest = {
	metadata_url: MetadataUrl;
	/**
	 * The probability of the metadata to be chosen for minting
	 * between 1 and 100
	 */
	probability_percentage: number;
};
