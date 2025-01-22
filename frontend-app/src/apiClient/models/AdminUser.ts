/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

/**
 * This is the user being returned by the Users Admin Api.
 * This dosent have the field is_admin.
 */
export type AdminUser = {
    /**
     * The email address of the user
     * This information is provided by the user during the signup process
     */
    email: string;
    /**
     * The concordium account address of the user
     * This information is updated by the user by providing concordium identity proofs
     */
    account_address?: string;
    /**
     * The amount of money that the user wants to invest
     * This information is supposed to be updated by the user
     */
    desired_investment_amount?: number;
    /**
     * The Cognito user id
     * This information is parsed from the identity token
     */
    cognito_user_id: string;
    /**
     * Has the user completed the KYC process?
     * This information is parsed from the Identity Registry contract
     * If the user's account_address is not set, then the user has not completed the KYC process
     */
    kyc_verified: boolean;
};

