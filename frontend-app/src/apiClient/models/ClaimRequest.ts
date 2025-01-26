/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { ClaimInfo } from "./ClaimInfo";

export type ClaimRequest = {
	claim: ClaimInfo;
	signer: string;
	/**
	 * Json serialized `AccountSignatures`
	 */
	signature: any;
};
