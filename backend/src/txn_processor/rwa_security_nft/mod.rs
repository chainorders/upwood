//! # RWA Security NFT
//! The RWA Security NFT module provides the functionality to manage RWA
//! security NFTs. The `RwaSecurityNftProcessor` struct provides methods to
//! process transactions related to RWA security NFTs. The API endpoints are
//! defined using the `poem_openapi` and `poem` crates, and the responses are
//! serialized as JSON using the `Json` type. The `RwaSecurityNftApi` struct
//! provides methods to retrieve paged lists of tokens and holders for a
//! specific RWA security NFT contract. It interacts with the
//! `IRwaSecurityNftDb` trait to fetch data from the database

pub mod api;
use super::cis2_api;
