use concordium_smart_contract_testing::AccountAddress;

pub struct UserTestClient {
    pub id:              String,
    pub email:           String,
    pub password:        String,
    pub id_token:        String,
    pub account_address: String,
}

impl UserTestClient {
    pub async fn call_api<Fut, Fn, R>(&self, call: Fn) -> R
    where
        Fut: std::future::Future<Output=R>,
        Fn: FnOnce(String) -> Fut, {
        call(self.id_token.clone()).await
    }

    pub fn account_address(&self) -> AccountAddress {
        self.account_address
            .parse()
            .expect("Failed to parse account address")
    }

    pub fn transact<F, T>(&self, f: F) -> T
    where F: FnOnce(AccountAddress) -> T {
        f(self.account_address())
    }
}
