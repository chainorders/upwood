//! This module contains the implementation of the RWA Market Transaction
//! Processor. The `RwaMarketProcessor` struct provides methods to process
//! transactions related to the RWA Market. It interacts with the `IRwaMarketDb`
//! trait to fetch data from the database and the `IEventEmitter` trait to emit
//! events. The API endpoints are defined using the `poem_openapi` and `poem`
//! crates, and the responses are serialized as JSON using the `Json` type.

pub mod api;
pub mod db;
pub mod processor;
