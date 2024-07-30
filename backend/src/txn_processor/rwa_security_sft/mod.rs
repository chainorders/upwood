//! This module contains the implementation of the RWA Security SFT Transaction
//! Processor. The `RwaSecuritySftProcessor` struct provides methods to process
//! transactions related to RWA security SFTs. It interacts with the
//! `IRwaSecuritySftDb` trait to fetch data from the database and the
//! `IEventEmitter` trait to emit events. The API endpoints are defined using
//! the `poem_openapi` and `poem` crates, and the responses are serialized as
//! JSON using the `Json` type. The `RwaSecuritySftApi` struct provides methods
//! to retrieve paged lists of tokens and holders for a specific RWA security
//! SFT contract. The `RwaSecuritySftProcessor` struct provides methods to
//! process transactions related to RWA security SFTs. It interacts with the
//! `IRwaSecuritySftDb` trait to fetch data from the database and the
//! `IEventEmitter` trait to emit events. The API endpoints are defined using
//! the `poem_openapi` and `poem` crates, and the responses are serialized as
//! JSON using the `Json` type.

pub mod api;
use super::cis2_api;
