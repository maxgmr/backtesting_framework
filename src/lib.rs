//! # backtesting-framework
//!
//! A simple framework for testing quantitative trading strategies.
#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::todo
)]

use serde::{Deserialize, Serialize};

mod asset;
mod portfolio;
mod price_data;

pub use asset::*;
pub use portfolio::*;
pub use price_data::*;

/// A Unix timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(i64);

/// An interval of time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interval {
    /// The start time of the interval.
    pub start: Timestamp,
    /// The end time of the interval.
    pub end: Timestamp,
}
