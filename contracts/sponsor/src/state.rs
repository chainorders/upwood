use concordium_std::*;

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub struct State<S = StateApi> {
    nonces: StateMap<AccountAddress, u64, S>,
}

const DEFAULT_NONCE: u64 = 0;

impl State<StateApi> {
    pub fn new(state_builder: &mut StateBuilder) -> Self {
        State {
            nonces: state_builder.new_map(),
        }
    }

    pub fn get_nonce(&self, account: AccountAddress) -> u64 {
        self.nonces.get(&account).map(|nonce| *nonce).unwrap_or(DEFAULT_NONCE)
    }

    pub fn increment_nonce(&mut self, account: AccountAddress) {
        self.nonces.entry(account).or_insert(DEFAULT_NONCE).modify(|nonce| *nonce += 1)
    }
}
