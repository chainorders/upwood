use std::{cmp, hash};

use poem_openapi::Enum;

#[derive(
    diesel_derive_enum::DbEnum,
    Debug,
    PartialEq,
    Enum,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    cmp::Eq,
    hash::Hash,
)]
#[ExistingTypePath = "crate::schema::sql_types::ForestProjectSecurityTokenContractType"]
#[DbValueStyle = "snake_case"]
pub enum SecurityTokenContractType {
    Property,
    Bond,
    PropertyPreSale,
    BondPreSale,
}

impl std::fmt::Display for SecurityTokenContractType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityTokenContractType::Property => write!(f, "Property"),
            SecurityTokenContractType::Bond => write!(f, "Bond"),
            SecurityTokenContractType::PropertyPreSale => write!(f, "PropertyPreSale"),
            SecurityTokenContractType::BondPreSale => write!(f, "BondPreSale"),
        }
    }
}
