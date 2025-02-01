/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { ForestProject } from './ForestProject';
import type { ForestProjectFundApiModel } from './ForestProjectFundApiModel';
import type { ForestProjectMarketApiModel } from './ForestProjectMarketApiModel';

export type ForestProjectAggApiModel = {
    forest_project: ForestProject;
    supply: string;
    property_market?: ForestProjectMarketApiModel;
    property_fund?: ForestProjectFundApiModel;
    bond_fund?: ForestProjectFundApiModel;
    user_balance: string;
};

