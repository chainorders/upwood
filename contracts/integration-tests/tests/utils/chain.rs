use concordium_smart_contract_testing::*;

pub fn create_accounts(
    chain: &mut Chain,
    accounts: Vec<AccountAddress>,
    amount: Amount,
) -> Vec<Account> {
    accounts
        .iter()
        .map(|account| {
            let account = Account::new(account.to_owned(), amount);
            chain.create_account(account.clone());
            account
        })
        .collect::<Vec<_>>()
}
