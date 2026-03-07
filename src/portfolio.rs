use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// The different types of errors which can be encountered when working with a [`Portfolio`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum PortfolioError {
    #[error("attempted to buy {buy_amt} with only {cash} in cash")]
    InsufficientCashError { buy_amt: f64, cash: f64 },
    #[error("attempted to access nonexistent position in {0}")]
    NonexistentPositionError(String),
    #[error("attempted to sell {sell_amt} shares while only owning {own_amt}")]
    InsufficientSharesError { sell_amt: f64, own_amt: f64 },
}

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
impl Portfolio {
    /// Create a new portfolio with an initial allocation of cash.
    #[must_use]
    pub fn new(initial_cash: f64) -> Self {
        Self {
            initial_cash,
            cash: initial_cash,
            positions: HashMap::new(),
            trades: Vec::new(),
            equity_history: Vec::new(),
        }
    }

    /// Execute a buy order.
    ///
    /// # Errors
    ///
    /// This function returns a [`PortfolioError::InsufficientCashError`] if the total cost of the
    /// buy order exceeds the amount of cash in the portfolio.
    pub fn buy(
        &mut self,
        ticker: &str,
        quantity: f64,
        price: f64,
        time: OffsetDateTime,
    ) -> Result<(), PortfolioError> {
        let cost = quantity * price;
        if cost > self.cash {
            return Err(PortfolioError::InsufficientCashError {
                buy_amt: cost,
                cash: self.cash,
            });
        }

        self.cash -= cost;

        let position = self.positions.entry(ticker.to_string()).or_default();
        let total_cost = (position.quantity * position.avg_cost) + cost;
        position.quantity += quantity;
        position.avg_cost = total_cost / position.quantity;

        self.trades.push(Trade {
            ticker: ticker.to_string(),
            action: TradeAction::Buy,
            quantity,
            price,
            time,
        });

        Ok(())
    }

    /// Execute a sell order.
    ///
    /// # Errors
    ///
    /// This function returns a [`PortfolioError::NonexistentPositionError`] if this portfolio has
    /// no positions in the given ticker.
    ///
    /// This function returns a [`PortfolioError::InsufficientSharesError`] if there is an attempt
    /// to sell more shares than the portfolio owns.
    pub fn sell(
        &mut self,
        ticker: &str,
        quantity: f64,
        price: f64,
        time: OffsetDateTime,
    ) -> Result<(), PortfolioError> {
        let position = self
            .positions
            .get_mut(ticker)
            .ok_or(PortfolioError::NonexistentPositionError(ticker.to_string()))?;

        if position.quantity < quantity {
            return Err(PortfolioError::InsufficientSharesError {
                sell_amt: quantity,
                own_amt: position.quantity,
            });
        }

        position.quantity -= quantity;
        self.cash += quantity * price;

        self.trades.push(Trade {
            ticker: ticker.to_string(),
            action: TradeAction::Sell,
            quantity,
            price,
            time,
        });

        Ok(())
    }

    /// Get the current position quantity for the given ticker
    #[must_use]
    pub fn get_position_quantity(&self, ticker: &str) -> f64 {
        self.positions.get(ticker).map_or(0f64, |p| p.quantity)
    }
}

/// Represents a single trade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// The symbol for the traded asset.
    pub ticker: String,
    /// Buy/sell.
    pub action: TradeAction,
    /// The quantity of the traded asset.
    pub quantity: f64,
    /// The price of the trade.
    pub price: f64,
    /// The time the trade occurred.
    pub time: OffsetDateTime,
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const FLOAT_CMP_TOLERANCE: f64 = 0.00001;
    fn f64s_are_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < FLOAT_CMP_TOLERANCE
    }

    #[test]
    fn buy_and_sell() {
        let mut p = Portfolio::new(10_000f64);

        p.buy("ABC.XYZ", 10f64, 100f64, OffsetDateTime::now_utc())
            .unwrap();
        assert!(f64s_are_eq(p.cash, 9_000f64));
        let pos = p.positions.get("ABC.XYZ").unwrap();
        assert!(f64s_are_eq(pos.avg_cost, 100f64));
        assert!(f64s_are_eq(pos.quantity, 10f64));
        let trade = p.trades.first().unwrap();
        assert_eq!(trade.ticker.as_str(), "ABC.XYZ");
        assert_eq!(trade.action, TradeAction::Buy);
        assert!(f64s_are_eq(trade.quantity, 10f64));
        assert!(f64s_are_eq(trade.price, 100f64));

        p.buy("ABC.XYZ", 10f64, 200f64, OffsetDateTime::now_utc())
            .unwrap();
        assert!(f64s_are_eq(p.cash, 7_000f64));
        let pos = p.positions.get("ABC.XYZ").unwrap();
        assert!(f64s_are_eq(pos.avg_cost, 150f64));
        assert!(f64s_are_eq(pos.quantity, 20f64));
        let trade = p.trades.get(1).unwrap();
        assert_eq!(trade.ticker.as_str(), "ABC.XYZ");
        assert_eq!(trade.action, TradeAction::Buy);
        assert!(f64s_are_eq(trade.quantity, 10f64));
        assert!(f64s_are_eq(trade.price, 200f64));

        p.sell("ABC.XYZ", 10f64, 400f64, OffsetDateTime::now_utc())
            .unwrap();
        assert!(f64s_are_eq(p.cash, 11_000f64));
        let pos = p.positions.get("ABC.XYZ").unwrap();
        assert!(f64s_are_eq(pos.avg_cost, 150f64));
        assert!(f64s_are_eq(pos.quantity, 10f64));
        let trade = p.trades.get(2).unwrap();
        assert_eq!(trade.ticker.as_str(), "ABC.XYZ");
        assert_eq!(trade.action, TradeAction::Sell);
        assert!(f64s_are_eq(trade.quantity, 10f64));
        assert!(f64s_are_eq(trade.price, 400f64));
    }

    #[test]
    fn buy_insufficient_cash() {
        let mut p = Portfolio::new(100f64);
        let e = p
            .buy("ABC.XYZ", 5f64, 21f64, OffsetDateTime::now_utc())
            .unwrap_err();
        if let PortfolioError::InsufficientCashError { buy_amt, cash } = e {
            assert!(f64s_are_eq(buy_amt, 5f64 * 21f64));
            assert!(f64s_are_eq(cash, 100f64));
        } else {
            panic!("{e}");
        }
    }

    #[test]
    fn sell_nonexistent_position() {
        let mut p = Portfolio::new(100f64);
        let e = p
            .sell("DNE", 5f64, 5f64, OffsetDateTime::now_utc())
            .unwrap_err();
        if let PortfolioError::NonexistentPositionError(s) = e {
            assert_eq!(s.as_str(), "DNE");
        } else {
            panic!("{e}");
        }
    }

    #[test]
    fn sell_insufficient_shares() {
        let mut p = Portfolio::new(2000f64);
        p.buy("ABC.XYZ", 2f64, 100f64, OffsetDateTime::now_utc())
            .unwrap();
        let e = p
            .sell("ABC.XYZ", 3f64, 100f64, OffsetDateTime::now_utc())
            .unwrap_err();
        if let PortfolioError::InsufficientSharesError { sell_amt, own_amt } = e {
            assert!(f64s_are_eq(sell_amt, 3f64));
            assert!(f64s_are_eq(own_amt, 2f64));
        } else {
            panic!("{e}");
        }
    }
}
