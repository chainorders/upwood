/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SignedMetadata } from "./SignedMetadata";

export type MintData = {
	signed_metadata: SignedMetadata;
	signer: string;
	/**
	 * Json serialized `AccountSignatures`
	 */
	signature: any;
};
