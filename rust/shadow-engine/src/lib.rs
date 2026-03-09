//! Shadow engine crate root.
//!
//! This crate is intentionally split into strict modules to preserve
//! deterministic behavior and separation of concerns.

pub mod db;
pub mod epoch;
pub mod ingestion;
pub mod merkle;
pub mod publisher;
pub mod state;
pub mod types;
