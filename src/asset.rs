use serde::{Deserialize, Serialize};

use crate::Timestamp;

/// All the data for a given asset.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Asset {
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub industry: Option<String>,
    pub sector: Option<String>,
    pub full_time_employees: Option<usize>,
    pub audit_risk: Option<usize>,
    pub board_risk: Option<usize>,
    pub compensation_risk: Option<usize>,
    pub share_holder_rights_risk: Option<usize>,
    pub overall_risk: Option<usize>,
    pub governance_timestamp: Option<Timestamp>,
    pub compensation_as_of_timestamp: Option<Timestamp>,
    pub ir_website: Option<String>,
    pub exchange: Option<String>,
    pub quote_type: Option<String>,
    pub symbol: Option<String>,
    pub underlying_symbol: Option<String>,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub first_trade_timestamp: Option<Timestamp>,
    pub timezone_full_name: Option<String>,
    pub timezone_short_name: Option<String>,
    pub gmt_offset_ms: Option<i64>,
    /// The last traded price.
    pub previous_close: Option<f64>,
    /// The price at market open.
    pub open: Option<f64>,
    /// The lowest price today.
    pub day_low: Option<f64>,
    /// The highest price today.
    pub day_high: Option<f64>,
    /// The last traded price (standard hrs).
    pub regular_market_previous_close: Option<f64>,
    /// The price at market open (standard hrs).
    pub regular_market_open: Option<f64>,
    /// The lowest price today (standard hrs).
    pub regular_market_day_low: Option<f64>,
    /// The highest price today (standard hrs).
    pub regular_market_day_high: Option<f64>,
    /// Total annual cash dividend payment per share.
    pub dividend_rate: Option<f64>,
    /// The dividend as a percent of current share price.
    pub dividend_yield: Option<f64>,
    /// The [`Timestamp`] after the stock dividend is paid.
    pub ex_dividend_date: Option<Timestamp>,
    /// The percentage of a company's net income paid as dividends.
    pub payout_ratio: Option<f64>,
    /// Average dividend yield of a company over 5 years.
    pub five_year_dividend_yield: Option<f64>,
    /// Volatility relative to the S&P 500.
    pub beta: Option<f64>,
    /// Current stock price divided by earnings per share over the last 12 months.
    pub trailing_pe: Option<f64>,
    /// Current stock price divided by estimated earnings per share for the next 12 months.
    pub forward_pe: Option<f64>,
    /// Total number of units traded on a given day.
    pub volume: Option<u64>,
    /// Total number of units traded on a given day (standard hrs).
    pub regular_market_volume: Option<u64>,
    /// Average number of units traded over the past 3 months.
    pub average_volume: Option<u64>,
    /// Average number of units traded over the last 10 days.
    pub average_volume_10days: Option<u64>,
    /// The highest price a buyer is willing to pay for this asset.
    pub bid: Option<f64>,
    /// The lowest price a seller is willing to accept for this asset.
    pub ask: Option<f64>,
    /// The total number of shares available at the bid price.
    pub bid_size: Option<i64>,
    /// The total number of shares available at the ask price.
    pub ask_size: Option<i64>,
    /// Total value of this asset's outstanding shares.
    pub market_cap: Option<u64>,
    /// The lowest price a stock has traded over the past year.
    pub fifty_two_week_low: Option<f64>,
    /// The highest price a stock has traded over the past year.
    pub fifty_two_week_high: Option<f64>,
    /// Stock valuation relative to revenue (market cap divided by total sales).
    pub price_to_sales_ttm: Option<f64>,
    /// Average closing price after the last 50 days.
    pub fifty_day_average: Option<f64>,
    /// Average closing price after the last 200 days.
    pub two_hundred_day_average: Option<f64>,
    /// Previous annual cash dividend payout per share.
    pub trailing_annual_dividend_rate: Option<f64>,
    /// Previous annual dividend as a percent of the share price.
    pub trailing_annual_dividend_yield: Option<f64>,
    /// The currency of the asset.
    pub currency: Option<String>,
    /// Measure of asset's total value; sum of market cap, debt, and minority interest while
    /// subtracting cash and equivalents.
    pub enterprise_value: Option<i64>,
    /// Proportion of money kept from revenue after accounting for all expenses.
    pub profit_margins: Option<f64>,
    /// Number of shares available to the general public to trade on the open market.
    pub float_shares: Option<u64>,
    /// Total number of shares currently held by all investors.
    pub shares_outstanding: Option<u64>,
    /// Total number of shorted shares.
    pub shares_short: Option<u64>,
    /// Total number of shorted shares last month.
    pub shares_short_prior_month: Option<u64>,
    /// [`Timestamp`] last month's shorted shares were recorded.
    pub shares_short_prior_month_date: Option<Timestamp>,
    /// [`Timestamp`] the shorted share count was recorded.
    pub date_short_interest: Option<Timestamp>,
    /// Proportion of shares held by short sellers.
    pub shares_percent_shares_out: Option<f64>,
    /// Proportion of shares held by insiders.
    pub held_percent_insiders: Option<f64>,
    /// Proportion of shares held by institutional investors.
    pub held_percent_institutions: Option<f64>,
    /// Number of shares sold short divided by average daily trading volume; the number of days it
    /// would take for short sellers to cover their positions.
    pub short_ratio: Option<f64>,
    /// Proportion of company's tradable shares which have been sold short.
    pub short_percent_of_float: Option<f64>,
    /// Share count that would exist if all convertible securities were converted into common
    /// stock.
    pub implied_shares_outstanding: Option<u64>,
    /// Net assets per share.
    pub book_value: Option<f64>,
    /// Ratio of market cap to book value (net assets).
    pub price_to_book: Option<f64>,
    /// [`Timestamp`] of the last fiscal year end.
    pub last_fiscal_year_end: Option<Timestamp>,
    /// [`Timestamp`] of the next fiscal year end.
    pub next_fiscal_year_end: Option<Timestamp>,
    /// [`Timestamp`] of the most recent quarter.
    pub most_recent_quarter: Option<Timestamp>,
    /// Quarterly percentage change of a company's net profit.
    pub earnings_quarterly_growth: Option<f64>,
    /// Portion of net profit available for common stockholders after all expenses, taxes, and
    /// preferred stock dividends have been paid.
    pub net_income_to_common: Option<i64>,
    /// Profit per share over the last 12 months.
    pub trailing_eps: Option<f64>,
    /// Anticipated profit per share over the next 12 months.
    pub forward_eps: Option<f64>,
    /// The most recent ratio of the last stock split.
    pub last_split_factor: Option<String>,
    /// The [`Timestamp`] of the last stock split.
    pub last_split_date: Option<Timestamp>,
    /// Ratio of enterprise value of annual sales.
    pub enterprise_to_revenue: Option<f64>,
    /// Ratio of enterprise value to EBITDA.
    pub enterprise_to_ebitda: Option<f64>,
    /// The increase or decrease in price over the last year.
    pub fifty_two_week_change: Option<f64>,
    /// The increase or decrease of the S&P 500 price over the last year.
    pub sandp_fifty_two_week_change: Option<f64>,
    /// The most recent cash dividend payment per share.
    pub last_dividend_value: Option<f64>,
    /// The [`Timestamp`] of the last dividend payment date.
    pub last_dividend_date: Option<Timestamp>,
    /// The current price.
    pub current_price: Option<f64>,
    /// Maximum forecasted price in the next 12 months.
    pub target_high_price: Option<f64>,
    /// Minimum forecasted price in the next 12 months.
    pub target_low_price: Option<f64>,
    /// Mean forecasted price in the next 12 months.
    pub target_mean_price: Option<f64>,
    /// Median forecasted price in the next 12 months.
    pub target_median_price: Option<f64>,
    /// Mean recommendation (smaller is better).
    pub recommendation_mean: Option<f64>,
    /// Number of analyst opinions.
    pub number_of_analyst_opinions: Option<u64>,
    /// Total amount of cash owned by asset.
    pub total_cash: Option<u64>,
    /// Cash per share.
    pub total_cash_per_share: Option<f64>,
    /// Earnings before interest, taxes, depreciation, and amortization.
    pub ebitda: Option<i64>,
    /// Total financial obligations.
    pub total_debt: Option<u64>,
    /// Ratio of liquid assets to debt.
    pub quick_ratio: Option<f64>,
    /// Ratio of liquid assets to short-term (<1 year) obligations.
    pub current_ratio: Option<f64>,
    /// Total amount of money brought in before expenses.
    pub total_revenue: Option<i64>,
    /// Ratio of total liabilities to total shareholders' equity.
    pub debt_to_equity: Option<f64>,
    /// Revenue per share.
    pub revenue_per_share: Option<f64>,
    /// Dollars of profit earned for every dollar of assets.
    pub return_on_assets: Option<f64>,
    /// Dollars of profit earned for every dollar of shareholders' equity.
    pub return_on_equity: Option<f64>,
    /// Revenue minus direct costs of goods and services.
    pub gross_profits: Option<i64>,
    /// Cash generated after operating expenses and capital expenditures.
    pub free_cash_flow: Option<i64>,
    /// Cash generated from core, day-to-day business operations (TTM).
    pub operating_cash_flow: Option<i64>,
    /// Increase in net profit over the last 12 months.
    pub earnings_growth: Option<f64>,
    /// Increase in revenue over the last 12 months.
    pub revenue_growth: Option<f64>,
    /// Proportion of revenue remaining after subtracting the direct costs of goods and services.
    pub gross_margins: Option<f64>,
    /// Cash profit per dollar of sales. EBITDA divided by total revenue.
    pub ebitda_margins: Option<f64>,
    /// Profit per dollar of revenue before taxes and interest.
    pub operating_margins: Option<f64>,
    /// The currency of the financial data.
    pub financial_currency: Option<String>,
}
