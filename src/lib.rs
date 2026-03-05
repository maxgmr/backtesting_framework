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
use time::OffsetDateTime;

mod asset;
mod portfolio;
mod price_data;
#[cfg(feature = "yahoo")]
mod yahoo;

pub use asset::*;
pub use portfolio::*;
pub use price_data::*;
#[cfg(feature = "yahoo")]
pub use yahoo::*;

/// An interval of time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interval {
    /// The start time of the interval.
    pub start: OffsetDateTime,
    /// The end time of the interval.
    pub end: OffsetDateTime,
}
