/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SecurityTokenContractType } from './SecurityTokenContractType';

export type ForestProjectFundInvestor = {
    forest_project_id: string;
    fund_contract_address: string;
    fund_token_id: string;
    fund_token_contract_address: string;
    investment_token_id: string;
    investment_token_contract_address: string;
    fund_type: SecurityTokenContractType;
    investor_account_address: string;
    investment_token_amount: string;
    investment_currency_amount: string;
    investor_cognito_user_id: string;
    investor_email: string;
};

