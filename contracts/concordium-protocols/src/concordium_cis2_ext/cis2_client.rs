use concordium_cis2::{
    BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, IsTokenAmount, IsTokenId,
    TokenMetadataQueryParams, TokenMetadataQueryResponse, Transfer, TransferParams,
};
use concordium_std::{
    ContractAddress, DeserialWithState, EntrypointName, ExternStateApi, Host, MetadataUrl, Serial,
};

use crate::contract_client::{invoke_contract, invoke_contract_read_only, ContractClientError};
pub type Cis2ClientError = ContractClientError<()>;

pub trait Cis2Client {
    fn invoke_token_metadata<T: IsTokenId>(
        &self,
        contract: &ContractAddress,
        params: &TokenMetadataQueryParams<T>,
    ) -> Result<TokenMetadataQueryResponse, Cis2ClientError>;

    fn invoke_token_metadata_single<T: IsTokenId>(
        &self,
        contract: &ContractAddress,
        token_id: T,
    ) -> Result<MetadataUrl, Cis2ClientError>;

    fn invoke_transfer<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        params: &TransferParams<T, A>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_transfer_single<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        param: Transfer<T, A>,
    ) -> Result<(), Cis2ClientError>;

    fn invoke_balance_of<T: IsTokenId, A: IsTokenAmount>(
        &self,
        contract: &ContractAddress,
        params: &BalanceOfQueryParams<T>,
    ) -> Result<BalanceOfQueryResponse<A>, Cis2ClientError>;

    fn invoke_balance_of_single<T: IsTokenId, A: IsTokenAmount + Copy>(
        &self,
        contract: &ContractAddress,
        params: BalanceOfQuery<T>,
    ) -> Result<A, Cis2ClientError>;
}

impl<S> Cis2Client for Host<S>
where
    S: Serial + DeserialWithState<ExternStateApi>,
{
    fn invoke_token_metadata<T: IsTokenId>(
        &self,
        contract: &ContractAddress,
        params: &TokenMetadataQueryParams<T>,
    ) -> Result<TokenMetadataQueryResponse, Cis2ClientError> {
        invoke_contract_read_only(
            self,
            contract,
            EntrypointName::new_unchecked("tokenMetadata"),
            params,
        )
    }

    fn invoke_token_metadata_single<T: IsTokenId>(
        &self,
        contract: &ContractAddress,
        token_id: T,
    ) -> Result<MetadataUrl, Cis2ClientError> {
        let params = TokenMetadataQueryParams {
            queries: vec![token_id],
        };
        let res = self.invoke_token_metadata(contract, &params)?;
        let res = res.0.first().ok_or(ContractClientError::InvalidResponse)?;
        Ok(res.clone())
    }

    fn invoke_transfer<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        params: &TransferParams<T, A>,
    ) -> Result<(), Cis2ClientError> {
        invoke_contract(
            self,
            contract,
            EntrypointName::new_unchecked("transfer"),
            params,
        )
    }

    fn invoke_transfer_single<T: IsTokenId, A: IsTokenAmount>(
        &mut self,
        contract: &ContractAddress,
        param: Transfer<T, A>,
    ) -> Result<(), Cis2ClientError> {
        let params = TransferParams(vec![param]);
        self.invoke_transfer(contract, &params)
    }

    fn invoke_balance_of<T: IsTokenId, A: IsTokenAmount>(
        &self,
        contract: &ContractAddress,
        params: &BalanceOfQueryParams<T>,
    ) -> Result<BalanceOfQueryResponse<A>, Cis2ClientError> {
        invoke_contract_read_only(
            self,
            contract,
            EntrypointName::new_unchecked("balanceOf"),
            params,
        )
    }

    fn invoke_balance_of_single<T: IsTokenId, A: IsTokenAmount + Copy>(
        &self,
        contract: &ContractAddress,
        params: BalanceOfQuery<T>,
    ) -> Result<A, Cis2ClientError> {
        let res: BalanceOfQueryResponse<A> = self.invoke_balance_of(contract, &BalanceOfQueryParams {
            queries: vec![params],
        })?;
        let res = res.0.first().ok_or(ContractClientError::InvalidResponse)?;
        Ok(*res)
    }
}
