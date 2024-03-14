//! This module contains the implementation of the RWA Identity Registry.
//! The `RwaIdentityRegistry` struct provides methods to interact with the RWA Identity Registry contract.
//! It interacts with the `IRwaIdentityRegistryDb` trait to fetch data from the database.
//! The API endpoints are defined using the `poem_openapi` and `poem` crates, and the responses are serialized as JSON using the `Json` type.

pub mod db;
pub mod processor;
