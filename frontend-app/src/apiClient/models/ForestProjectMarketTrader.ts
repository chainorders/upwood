/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SecurityTokenContractType } from "./SecurityTokenContractType";
import type { Trader } from "./Trader";

export type ForestProjectMarketTrader = {
	trader: Trader;
	market_type: SecurityTokenContractType;
	forest_project_id: string;
	forest_project_name: string;
	cognito_user_id: string;
	email: string;
};
