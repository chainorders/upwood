/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

/**
 * The processor type of a contract.
 * The processor type is used to determine which processor to use when processing events.
 * ### Caution! This is persisted to the database. Hence changing the processor type requires a migration.
 */
export enum ProcessorType {
    IDENTITY_REGISTRY = 'IdentityRegistry',
    SECURITY_SFT_SINGLE = 'SecuritySftSingle',
    SECURITY_SFT_REWARDS = 'SecuritySftRewards',
    NFT_MULTI_REWARDED = 'NftMultiRewarded',
    SECURITY_MINT_FUND = 'SecurityMintFund',
    SECURITY_P2PTRADING = 'SecurityP2PTrading',
    OFFCHAIN_REWARDS = 'OffchainRewards',
    SECURITY_SFT_MULTI = 'SecuritySftMulti',
    SECURITY_SFT_MULTI_YIELDER = 'SecuritySftMultiYielder',
}
