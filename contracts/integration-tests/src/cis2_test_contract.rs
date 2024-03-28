use super::test_contract_client::*;
use concordium_cis2::*;
use concordium_smart_contract_testing::*;

pub trait ICis2Contract<T: IsTokenId, A: IsTokenAmount, TEvent>: ITestContract {
    fn balance_of(
        &self,
    ) -> GenericReceive<BalanceOfQueryParams<T>, BalanceOfQueryResponse<A>, TEvent> {
        GenericReceive::<BalanceOfQueryParams<T>, BalanceOfQueryResponse<A>, TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "balanceOf",
            self.max_energy(),
        )
    }

    fn transfer(&self) -> GenericReceive<TransferParams<T, A>, (), TEvent> {
        GenericReceive::<TransferParams<T, A>, (), TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "transfer",
            self.max_energy(),
        )
    }

    fn update_operator(&self) -> GenericReceive<OperatorUpdate, (), TEvent> {
        GenericReceive::<OperatorUpdate, (), TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "updateOperator",
            self.max_energy(),
        )
    }

    fn operator_of(
        &self,
    ) -> GenericReceive<OperatorOfQueryParams, OperatorOfQueryResponse, TEvent> {
        GenericReceive::<OperatorOfQueryParams, OperatorOfQueryResponse, TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "operatorOf",
            self.max_energy(),
        )
    }

    fn token_metadata(
        &self,
    ) -> GenericReceive<TokenMetadataQueryParams<T>, TokenMetadataQueryResponse, TEvent> {
        GenericReceive::<TokenMetadataQueryParams<T>, TokenMetadataQueryResponse, TEvent>::new(
            self.contract_address(),
            Self::contract_name(),
            "tokenMetadata",
            self.max_energy(),
        )
    }
}

pub trait ICis2ContractExt<T: IsTokenId, A: IsTokenAmount + Copy, TEvent>:
    ICis2Contract<T, A, TEvent> {
    fn balance_of_invoke(
        &self,
        chain: &mut Chain,
        invoker: &Account,
        params: &BalanceOfQueryParams<T>,
    ) -> Result<BalanceOfQueryResponse<A>, ContractInvokeError> {
        self.balance_of().invoke(chain, invoker, params).map(|r| {
            self.balance_of().parse_return_value(&r).expect("cis2: balanceOf - parse return value")
        })
    }

    fn balance_of_single_invoke(
        &self,
        chain: &mut Chain,
        invoker: &Account,
        token_id: T,
        address: Address,
    ) -> Result<A, ContractInvokeError> {
        self.balance_of_invoke(chain, invoker, &BalanceOfQueryParams {
            queries: vec![BalanceOfQuery {
                token_id,
                address,
            }],
        })
        .map(|res| *res.0.first().expect("cis2: balanceOf - single query"))
    }
}

pub trait ICis2ContractUnitTokenExt<A: IsTokenAmount + Copy, TEvent>:
    ICis2Contract<TokenIdUnit, A, TEvent> {
    fn balance_of_invoke(
        &self,
        chain: &mut Chain,
        invoker: &Account,
        params: Vec<Address>,
    ) -> Result<BalanceOfQueryResponse<A>, ContractInvokeError> {
        self.balance_of()
            .invoke(chain, invoker, &BalanceOfQueryParams {
                queries: params
                    .iter()
                    .map(|a| BalanceOfQuery {
                        address:  *a,
                        token_id: TokenIdUnit(),
                    })
                    .collect(),
            })
            .map(|r| {
                self.balance_of()
                    .parse_return_value(&r)
                    .expect("cis2: balanceOf - parse return value")
            })
    }

    fn balance_of_single_invoke(
        &self,
        chain: &mut Chain,
        invoker: &Account,
        params: Address,
    ) -> Result<A, ContractInvokeError> {
        self.balance_of_invoke(chain, invoker, vec![params])
            .map(|res| *res.0.first().expect("cis2: balanceOf - single query"))
    }
}
