use concordium_cis2::{IsTokenAmount, IsTokenId};
use concordium_protocols::rate::Rate;
use concordium_rust_sdk::base::contracts_common::{Cursor, Deserial, Read, Serial};
use concordium_rust_sdk::cis2;
use concordium_rust_sdk::types::ContractAddress;
use num_traits::FromPrimitive;
use rust_decimal::Decimal;

pub trait Cis2TokenIdToDecimal {
    fn to_decimal(&self) -> Decimal;
}

impl Cis2TokenIdToDecimal for cis2::TokenId {
    fn to_decimal(&self) -> Decimal {
        let mut bytes = vec![];
        self.serial(&mut bytes)
            .expect("Failed to serialize token id");
        let mut cursor: Cursor<_> = Cursor::new(bytes);
        let size = cursor.read_u8().expect("Failed to read token id size");
        let token_id = match size {
            0 => 0,
            1 => cursor.read_u8().expect("Failed to read cis2 token id u8") as u64,
            2 => cursor.read_u16().expect("Failed to read cis2 token id u16") as u64,
            4 => cursor.read_u32().expect("Failed to read cis2 token id") as u64,
            8 => cursor.read_u64().expect("Failed to read cis2 token id"),
            _ => panic!("Invalid cis2 token id size: {}", size),
        };
        Decimal::from_u64(token_id).expect("Failed to convert to Decimal cis2")
    }
}

pub trait TokenIdToDecimal {
    fn to_decimal(&self) -> Decimal;
}

impl<T: IsTokenId> TokenIdToDecimal for T {
    fn to_decimal(&self) -> Decimal {
        let mut bytes = vec![];
        self.serial(&mut bytes)
            .expect("Failed to serialize token id");
        let mut cursor: Cursor<_> = Cursor::new(bytes);
        let size = cursor.read_u8().expect("Failed to read token id size");
        let token_id = match size {
            0 => 0,
            1 => cursor.read_u8().expect("Failed to read token id u8") as u64,
            2 => cursor.read_u16().expect("Failed to read token id u16") as u64,
            4 => cursor.read_u32().expect("Failed to read token id") as u64,
            8 => cursor.read_u64().expect("Failed to read token id"),
            _ => panic!("Invalid token id size: {}", size),
        };
        Decimal::from_u64(token_id).expect("Failed to convert to Decimal")
    }
}

pub trait TokenAmountToDecimal {
    fn to_decimal(&self) -> Decimal;
}

impl<A: IsTokenAmount> TokenAmountToDecimal for A {
    fn to_decimal(&self) -> Decimal {
        let token_amount = to_cis2_token_amount(self);
        Decimal::from_str_radix(token_amount.0.to_str_radix(10).as_str(), 10)
            .expect("Failed to convert token amount to Decimal")
    }
}

fn to_cis2_token_amount<A>(amount: &A) -> cis2::TokenAmount
where A: IsTokenAmount {
    let mut bytes = vec![];
    amount
        .serial(&mut bytes)
        .expect("Failed to serialize token amount");
    let mut cursor: Cursor<_> = Cursor::new(bytes);

    cis2::TokenAmount::deserial(&mut cursor).expect("Failed to deserialize token amount")
}

pub trait ContractAddressToDecimal {
    fn to_decimal(&self) -> Decimal;
}
impl ContractAddressToDecimal for ContractAddress {
    fn to_decimal(&self) -> Decimal {
        Decimal::from_u64(self.index).expect("Failed to convert contract address to Decimal")
    }
}

pub trait RateToDecimal {
    fn to_decimal(&self) -> Decimal;
}

impl RateToDecimal for Rate {
    #[inline]
    fn to_decimal(&self) -> Decimal {
        Decimal::from_u64(self.numerator).expect("Failed to convert nunmerator to Decimal")
            / Decimal::from_u64(self.denominator).expect("Failed to convert denominator to Decimal")
    }
}

#[inline]
pub fn rate_to_decimal(numerator: u64, denominator: u64) -> Decimal {
    Decimal::from_u64(numerator).expect("Failed to convert nunmerator to Decimal")
        / Decimal::from_u64(denominator).expect("Failed to convert denominator to Decimal")
}
