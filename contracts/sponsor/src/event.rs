use concordium_std::*;

#[derive(Serial, Deserial, SchemaType)]
pub struct NonceEvent {
    pub account: AccountAddress,
    pub nonce:   u64,
}

#[derive(Serialize, SchemaType)]
pub enum Event {
    Nonce(NonceEvent),
}
