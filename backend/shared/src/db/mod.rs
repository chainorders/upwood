pub mod cis2_security;
pub mod identity_registry;
pub mod nft_multi_rewarded;
pub mod offchain_rewards;
pub mod security_mint_fund;
pub mod security_p2p_trading;
pub mod security_sft_rewards;
pub mod security_sft_single;
pub mod txn_listener;

use concordium_rust_sdk::cis2;
use num_traits::ToPrimitive;
use rust_decimal::Decimal;

pub fn token_amount_to_sql(amount: &cis2::TokenAmount) -> Decimal {
    Decimal::from_str_radix(amount.0.to_str_radix(10).as_str(), 10)
        .expect("Failed to convert to Decimal")
}

pub fn to_u64(value: Decimal) -> u64 { value.to_u64().expect("Failed to convert to u64") }

#[cfg(test)]
mod tests {
    use concordium_rust_sdk::cis2;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;
    use rust_decimal::Decimal;

    use super::token_amount_to_sql;

    #[test]
    fn token_amount_conversions() {
        let amount = 1000u64;
        let token_amount = cis2::TokenAmount(BigUint::from_u64(amount).unwrap());
        let token_amount = token_amount_to_sql(&token_amount);
        assert_eq!(token_amount, Decimal::from_u64(amount).unwrap());
    }
}
