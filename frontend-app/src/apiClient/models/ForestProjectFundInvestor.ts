/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Investor } from "./Investor";
import type { SecurityTokenContractType } from "./SecurityTokenContractType";

export type ForestProjectFundInvestor = {
	investor: Investor;
	fund_type: SecurityTokenContractType;
	forest_project_id: string;
	forest_project_name: string;
	cognito_user_id: string;
	email: string;
};
