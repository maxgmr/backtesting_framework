use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// A portfolio that tracks cash, positions, and equity over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    /// The initial cash allocated upon creation.
    pub initial_cash: f64,
    /// The current amount of cash.
    pub cash: f64,
    /// The list of [`Position`]s.
    pub positions: HashMap<String, Position>,
    /// A history of all the [`Trade`]s made.
    pub trades: Vec<Trade>,
    /// A history of equity.
    pub equity_history: Vec<EquityPoint>,
}

/// Represents a single trade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    ticker: String,
    action: TradeAction,
    quantity: f64,
    price: f64,
    time: OffsetDateTime,
}

/// Defines whether the [`Trade`] is a buy or a sell.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradeAction {
    #[allow(missing_docs)]
    Buy,
    #[allow(missing_docs)]
    Sell,
}

/// Represents a position in a single tracker.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Position {
    /// The quantity of the position.
    pub quantity: f64,
    /// The average cost paid for each constituent of this position.
    pub avg_cost: f64,
}

/// Value of equity at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityPoint {
    /// The time at which this value was recorded.
    pub time: OffsetDateTime,
    /// The value.
    pub equity: f64,
}
