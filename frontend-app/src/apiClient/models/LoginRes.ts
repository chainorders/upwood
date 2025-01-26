/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { ApiUser } from "./ApiUser";
import type { SystemContractsConfig } from "./SystemContractsConfig";

export type LoginRes = {
	id_token: string;
	user: ApiUser;
	contracts: SystemContractsConfig;
};
