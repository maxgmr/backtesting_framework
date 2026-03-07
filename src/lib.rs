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
#[cfg(feature = "plotting")]
mod plotting;
mod portfolio;
mod price_data;
#[cfg(feature = "yahoo")]
mod yahoo;

pub use asset::*;
#[cfg(feature = "plotting")]
pub use plotting::*;
pub use portfolio::*;
pub use price_data::*;
#[cfg(feature = "yahoo")]
pub use yahoo::*;

// Re-exports
pub use time;

/// An interval of time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interval {
    /// The start time of the interval.
    pub start: OffsetDateTime,
    /// The end time of the interval.
    pub end: OffsetDateTime,
}
impl Interval {
    /// Create a new interval, returning [`None`] if the end time is earlier than the start time.
    #[must_use]
    pub fn new(start: OffsetDateTime, end: OffsetDateTime) -> Option<Self> {
        if end < start {
            return None;
        }
        Some(Self { start, end })
    }
    /// Return true if and only if this interval overlaps with the other interval.
    #[must_use]
    pub fn overlaps(&self, other: Interval) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    /// Return true if and only if all given intervals share a common point.
    ///
    /// Return [`None`] if given an empty list.
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// # macro_rules! mk_interval {
    /// #     (($h1:expr, $m1:expr, $s1:expr), ($h2:expr, $m2:expr, $s2:expr)) => {
    /// #         Interval::new(
    /// #             OffsetDateTime::new_utc(
    /// #                 Date::from_calendar_date(2000, Month::January, 1).unwrap(),
    /// #                 Time::from_hms($h1, $m1, $s1).unwrap(),
    /// #             ),
    /// #             OffsetDateTime::new_utc(
    /// #                 Date::from_calendar_date(2000, Month::January, 1).unwrap(),
    /// #                 Time::from_hms($h2, $m2, $s2).unwrap(),
    /// #             ),
    /// #         )
    /// #         .unwrap()
    /// #     };
    /// # }
    /// # use time::{Date, Time, Month, OffsetDateTime};
    /// # use backtesting_framework::*;
    /// # let i1 = mk_interval!((14, 00, 00), (14, 30, 00));
    /// # let i2 = mk_interval!((13, 30, 00), (14, 15, 00));
    /// # let i3 = mk_interval!((13, 29, 00), (14, 00, 00));
    /// // i1: 14:00:00-14:30:00
    /// // i2: 13:30:00-14:15:00
    /// // i3: 13:29:00-14:00:00
    /// // This will return true since all three intervals overlap.
    /// assert!(Interval::exists_common_point(&vec![i1, i2, i3]).unwrap());
    ///
    /// # let i1 = mk_interval!((14, 00, 00), (14, 30, 00));
    /// # let i2 = mk_interval!((13, 30, 00), (14, 15, 00));
    /// # let i3 = mk_interval!((14, 15, 01), (14, 20, 00));
    /// // i1: 14:00:00-14:30:00
    /// // i2: 13:30:00-14:15:00
    /// // i3: 14:15:01-14:20:00
    /// // This will return false since i2 and i3 don't overlap.
    /// assert!(!Interval::exists_common_point(&vec![i1, i2, i3]).unwrap());
    ///
    /// // This will return None because there are no intervals.
    /// assert!(Interval::exists_common_point(&vec![]).is_none());
    /// ```
    #[must_use]
    pub fn exists_common_point(intervals: &[Self]) -> Option<bool> {
        let latest_start = intervals.iter().map(|i| i.start).max()?;
        let earliest_end = intervals.iter().map(|i| i.end).min()?;

        Some(latest_start <= earliest_end)
    }
}
impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
