use crate::Interval;
use serde::{Deserialize, Serialize};

/// Historical price data for a ticker.
#[derive(Debug, Clone)]
pub struct PriceData {
    /// The ticker associated with the data.
    pub ticker: String,
    /// The historical prices of the data.
    pub bars: Vec<PriceBar>,
}
impl PriceData {
    /// Get the closing price at a specific index.
    #[must_use]
    pub fn close_at(&self, index: usize) -> Option<f64> {
        self.bars.get(index).map(|b| b.close)
    }

    /// Get the number of bars.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bars.len()
    }

    /// Check if data is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bars.is_empty()
    }
}

/// A single candlestick of price data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceBar {
    /// The time [`Interval`].
    pub interval: Interval,
    /// The price at the beginning of the interval.
    pub open: f64,
    /// The highest price reached during the interval.
    pub high: f64,
    /// The lowest price reached during the interval.
    pub low: f64,
    /// The final price at the end of the interval.
    pub close: f64,
    /// The final price at the end of the interval, adjusted to account for corporate actions.
    pub adjusted_close: f64,
    /// The total number of units traded throughout the interval.
    pub volume: u64,
}
