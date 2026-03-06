use time::OffsetDateTime;
use yahoo_finance_api as yahoo;

use crate::{Asset, Interval, PriceBar, PriceData};

/// Get [`PriceData`] for the given ticker from the start date to the end date (non-inclusive) if
/// available, with the [`YahooInterval`] between quotes.
///
/// # Errors
///
/// This function propagates any [`yahoo::YahooError`]s encountered when using the Yahoo Finance
/// API.
///
/// It will also return a [`yahoo::YahooError::NoResult`] if the request didn't return any usable
/// data.
///
/// It will also return a [`yahoo::YahooError::InvalidDateFormat`] if any timestamps can't be
/// parsed as Unix epochs.
pub async fn yahoo_fetch_price_history(
    ticker: &str,
    start: OffsetDateTime,
    end: OffsetDateTime,
    interval: YahooInterval,
) -> Result<PriceData, yahoo::YahooError> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider
        .get_quote_history_interval(ticker, start, end, interval.into())
        .await?;
    try_response_to_price_data(response, Some(end))
}

/// Get [`PriceData`] for the given ticker from the last `range` amount of time up to now
/// (non-inclusive), with the [`YahooInterval`] between quotes.
///
/// # Errors
///
/// This function propagates any [`yahoo::YahooError`]s encountered when using the Yahoo Finance
/// API.
///
/// It will also return a [`yahoo::YahooError::NoResult`] if the request didn't return any usable
/// data.
///
/// It will also return a [`yahoo::YahooError::InvalidDateFormat`] if any timestamps can't be
/// parsed as Unix epochs.
pub async fn yahoo_fetch_trailing_prices(
    ticker: &str,
    interval: YahooInterval,
    range: YahooRange,
) -> Result<PriceData, yahoo::YahooError> {
    let provider = yahoo::YahooConnector::new()?;
    let response = provider
        .get_quote_range(ticker, interval.into(), range.into())
        .await?;
    try_response_to_price_data(response, None)
}

/// Get [`Asset`] data for the given ticker using the Yahoo Finance API.
///
/// # Errors
///
/// This function propagates any [`yahoo::YahooError`]s encountered when using the Yahoo Finance
/// API.
///
/// It also will return a [`yahoo::YahooError::NoResult`] if the request didn't return anything of
/// use.
///
/// It also will return a [`yahoo::YahooError::InvalidDateFormat`] if any timestamps can't be
/// parsed as Unix epochs.
pub async fn yahoo_fetch_asset(ticker: &str) -> Result<Asset, yahoo::YahooError> {
    let mut provider = yahoo::YahooConnector::new()?;
    let response = provider.get_ticker_info(ticker).await?;
    response.try_into()
}

// If no end time is provided, this function will use chart.result[0].meta.regular_market_time as
// the end time.
fn try_response_to_price_data(
    response: yahoo::YResponse,
    end_time: Option<OffsetDateTime>,
) -> Result<PriceData, yahoo::YahooError> {
    let quotes = response.quotes()?;
    let metadata = response
        .chart
        .result
        .ok_or(yahoo::YahooError::NoResult)?
        .pop()
        .ok_or(yahoo::YahooError::NoResult)?
        .meta;
    let ticker = metadata.symbol;

    let mut bars = Vec::with_capacity(quotes.len());

    // Process all bars except the last
    for window in quotes.windows(2) {
        let quote = &window[0];
        let next_quote = &window[1];

        let start = OffsetDateTime::from_unix_timestamp(quote.timestamp)
            .map_err(|_| yahoo::YahooError::InvalidDateFormat)?;
        let end = OffsetDateTime::from_unix_timestamp(next_quote.timestamp)
            .map_err(|_| yahoo::YahooError::InvalidDateFormat)?;

        bars.push(PriceBar {
            interval: Interval { start, end },
            open: quote.open,
            high: quote.high,
            low: quote.low,
            close: quote.close,
            adjusted_close: quote.adjclose,
            volume: quote.volume,
        });
    }

    if let Some(end_time) = end_time
        && let Some(last_quote) = quotes.last()
    {
        let start = OffsetDateTime::from_unix_timestamp(last_quote.timestamp)
            .map_err(|_| yahoo::YahooError::InvalidDateFormat)?;

        bars.push(PriceBar {
            interval: Interval {
                start,
                end: end_time,
            },
            open: last_quote.open,
            high: last_quote.high,
            low: last_quote.low,
            close: last_quote.close,
            adjusted_close: last_quote.adjclose,
            volume: last_quote.volume,
        });
    }

    Ok(PriceData { ticker, bars })
}

impl TryFrom<yahoo::YQuoteSummary> for Asset {
    type Error = yahoo::YahooError;

    #[allow(clippy::too_many_lines)]
    fn try_from(mut value: yahoo::YQuoteSummary) -> Result<Self, Self::Error> {
        // Create asset with all values set to None
        let mut a = Asset::default();

        let quote_summary = value
            .quote_summary
            .take()
            .ok_or(yahoo::YahooError::NoResult)?;
        let mut result = quote_summary.result.ok_or(yahoo::YahooError::NoResult)?;
        let data = result.pop().ok_or(yahoo::YahooError::NoResult)?;

        // Assign values to the asset
        if let Some(p) = data.asset_profile {
            a.address = p.address1;
            a.city = p.city;
            a.state = p.state;
            a.zip = p.zip;
            a.country = p.country;
            a.phone = p.phone;
            a.website = p.website;
            a.industry = p.industry;
            a.sector = p.sector;
            a.full_time_employees = p.full_time_employees.map(|fte| fte as usize);
            a.audit_risk = p.audit_risk.map(|ar| ar as usize);
            a.board_risk = p.board_risk.map(|br| br as usize);
            a.compensation_risk = p.compensation_risk.map(|cr| cr as usize);
            a.share_holder_rights_risk = p.share_holder_rights_risk.map(|shrr| shrr as usize);
            a.overall_risk = p.overall_risk.map(|or| or as usize);
            a.governance_timestamp = convert_odt(p.governance_epoch_date.map(i64::from))?;
            a.compensation_as_of_timestamp =
                convert_odt(p.compensation_as_of_epoch_date.map(i64::from))?;
            a.ir_website = p.ir_website;
        }

        if let Some(q) = data.quote_type {
            a.exchange = q.exchange;
            a.quote_type = q.quote_type;
            a.symbol = q.symbol;
            a.underlying_symbol = q.underlying_symbol;
            a.short_name = q.short_name;
            a.long_name = q.long_name;
            a.first_trade_timestamp = convert_odt(q.first_trade_date_epoch_utc)?;
            a.timezone_full_name = q.timezone_full_name;
            a.timezone_short_name = q.timezone_short_name;
            a.gmt_offset_ms = q.gmt_off_set_milliseconds;
        }

        if let Some(s) = data.summary_detail {
            a.previous_close = s.previous_close;
            a.open = s.open;
            a.day_low = s.day_low;
            a.day_high = s.day_high;
            a.regular_market_previous_close = s.regular_market_previous_close;
            a.regular_market_open = s.regular_market_open;
            a.regular_market_day_low = s.regular_market_day_low;
            a.regular_market_day_high = s.regular_market_day_high;
            a.dividend_rate = s.dividend_rate;
            a.dividend_yield = s.dividend_yield;
            a.ex_dividend_date = convert_odt(s.ex_dividend_date)?;
            a.payout_ratio = s.payout_ratio;
            a.five_year_dividend_yield = s.five_year_avg_dividend_yield;
            a.beta = s.beta;
            a.trailing_pe = s.trailing_pe;
            a.forward_pe = s.forward_pe;
            a.volume = s.volume;
            a.regular_market_volume = s.regular_market_volume;
            a.average_volume = s.average_volume;
            a.average_volume_10days = s.average_volume_10days;
            a.bid = s.bid;
            a.ask = s.ask;
            a.bid_size = s.bid_size;
            a.ask_size = s.ask_size;
            a.market_cap = s.market_cap;
            a.fifty_two_week_low = s.fifty_two_week_low;
            a.fifty_two_week_high = s.fifty_two_week_high;
            a.price_to_sales_ttm = s.price_to_sales_trailing12months;
            a.fifty_day_average = s.fifty_day_average;
            a.two_hundred_day_average = s.two_hundred_day_average;
            a.trailing_annual_dividend_rate = s.trailing_annual_dividend_yield;
            a.trailing_annual_dividend_yield = s.trailing_annual_dividend_yield;
            a.currency = s.currency;
        }

        if let Some(d) = data.default_key_statistics {
            a.enterprise_value = d.enterprise_value;
            a.profit_margins = d.profit_margins;
            a.float_shares = d.float_shares;
            a.shares_outstanding = d.shares_outstanding;
            a.shares_short = d.shares_short;
            a.shares_short_prior_month = d.shares_short_prior_month;
            a.shares_short_prior_month_date =
                convert_odt(d.shares_short_previous_month_date.map(u64::cast_signed))?;
            a.date_short_interest = convert_odt(d.date_short_interest)?;
            a.shares_percent_shares_out = d.shares_percent_shares_out;
            a.held_percent_insiders = d.held_percent_insiders;
            a.held_percent_institutions = d.held_percent_institutions;
            a.short_ratio = d.short_ratio;
            a.short_percent_of_float = d.short_percent_of_float;
            a.implied_shares_outstanding = d.implied_shares_outstanding;
            a.book_value = d.book_value;
            a.price_to_book = d.price_to_book;
            a.last_fiscal_year_end = convert_odt(d.last_fiscal_year_end)?;
            a.next_fiscal_year_end = convert_odt(d.next_fiscal_year_end)?;
            a.most_recent_quarter = convert_odt(d.most_recent_quarter)?;
            a.earnings_quarterly_growth = d.earnings_quarterly_growth;
            a.net_income_to_common = d.net_income_to_common;
            a.trailing_eps = d.trailing_eps;
            a.last_split_factor = d.last_split_factor;
            a.last_split_date = convert_odt(d.last_split_date)?;
            a.enterprise_to_revenue = d.enterprise_to_revenue;
            a.enterprise_to_ebitda = d.enterprise_to_ebitda;
            a.fifty_two_week_change = d.fifty_two_week_change;
            a.sandp_fifty_two_week_change = d.sand_p_fifty_two_week_change;
            a.last_dividend_value = d.last_dividend_value;
            a.last_dividend_date = convert_odt(d.last_dividend_date)?;
        }

        if let Some(f) = data.financial_data {
            a.current_price = f.current_price;
            a.target_high_price = f.target_high_price;
            a.target_low_price = f.target_low_price;
            a.target_mean_price = f.target_mean_price;
            a.target_median_price = f.target_median_price;
            a.recommendation_mean = f.recommendation_mean;
            a.number_of_analyst_opinions = f.number_of_analyst_opinions;
            a.total_cash = f.total_cash;
            a.total_cash_per_share = f.total_cash_per_share;
            a.ebitda = f.ebitda;
            a.total_debt = f.total_debt;
            a.quick_ratio = f.quick_ratio;
            a.current_ratio = f.current_ratio;
            a.total_revenue = f.total_revenue;
            a.debt_to_equity = f.debt_to_equity;
            a.revenue_per_share = f.revenue_per_share;
            a.return_on_assets = f.return_on_assets;
            a.return_on_equity = f.return_on_equity;
            a.gross_profits = f.gross_profits;
            a.free_cash_flow = f.free_cashflow;
            a.operating_cash_flow = f.operating_cashflow;
            a.earnings_growth = f.earnings_growth;
            a.gross_margins = f.gross_margins;
            a.ebitda_margins = f.ebitda_margins;
            a.operating_margins = f.operating_margins;
            a.financial_currency = f.financial_currency;
        }

        Ok(a)
    }
}

fn convert_odt(val: Option<i64>) -> Result<Option<OffsetDateTime>, yahoo::YahooError> {
    match val {
        Some(v) => OffsetDateTime::from_unix_timestamp(v)
            .map(Some)
            .map_err(|_| yahoo_finance_api::YahooError::InvalidDateFormat),
        None => Ok(None),
    }
}

/// All the different valid Yahoo Finance API ranges. This is used to specify how far back the
/// asset data should go.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum YahooRange {
    OneDay,
    FiveDays,
    OneMonth,
    ThreeMonths,
    SixMonths,
    OneYear,
    TwoYears,
    FiveYears,
    TenYears,
    Ytd,
    Max,
}
impl From<YahooRange> for &'static str {
    fn from(value: YahooRange) -> &'static str {
        match value {
            YahooRange::OneDay => "1d",
            YahooRange::FiveDays => "5d",
            YahooRange::OneMonth => "1mo",
            YahooRange::ThreeMonths => "3mo",
            YahooRange::SixMonths => "6mo",
            YahooRange::OneYear => "1y",
            YahooRange::TwoYears => "2y",
            YahooRange::FiveYears => "5y",
            YahooRange::TenYears => "10y",
            YahooRange::Ytd => "ytd",
            YahooRange::Max => "max",
        }
    }
}

/// All the different valid Yahoo Finance API intervals. This is used to specify the time between
/// price quotes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum YahooInterval {
    OneMinute,
    TwoMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    SixtyMinutes,
    NinetyMinutes,
    OneHour,
    FourHours,
    OneDay,
    FiveDays,
    OneWeek,
    OneMonth,
    ThreeMonths,
}
impl From<YahooInterval> for &'static str {
    fn from(value: YahooInterval) -> Self {
        #[allow(clippy::enum_glob_use)]
        use YahooInterval::*;
        match value {
            OneMinute => "1m",
            TwoMinutes => "2m",
            FiveMinutes => "5m",
            FifteenMinutes => "15m",
            ThirtyMinutes => "30m",
            SixtyMinutes => "60m",
            NinetyMinutes => "90m",
            OneHour => "1h",
            FourHours => "4h",
            OneDay => "1d",
            FiveDays => "5d",
            OneWeek => "1wk",
            OneMonth => "1mo",
            ThreeMonths => "3mo",
        }
    }
}
impl TryFrom<&'static str> for YahooInterval {
    type Error = yahoo::YahooError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1m" => Ok(Self::OneMinute),
            "2m" => Ok(Self::TwoMinutes),
            "5m" => Ok(Self::FiveMinutes),
            "15m" => Ok(Self::FifteenMinutes),
            "30m" => Ok(Self::ThirtyMinutes),
            "60m" => Ok(Self::SixtyMinutes),
            "90m" => Ok(Self::NinetyMinutes),
            "1h" => Ok(Self::OneHour),
            "4h" => Ok(Self::FourHours),
            "1d" => Ok(Self::OneDay),
            "5d" => Ok(Self::FiveDays),
            "1wk" => Ok(Self::OneWeek),
            "1mo" => Ok(Self::OneMonth),
            "3mo" => Ok(Self::ThreeMonths),
            _ => Err(yahoo::YahooError::InvalidDateFormat),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_aapl() {
        let asset = yahoo_fetch_asset("AAPL").await.unwrap();
        assert_eq!(asset.symbol, Some("AAPL".to_string()));
        assert!(asset.market_cap.is_some());
    }

    #[tokio::test]
    async fn get_dne() {
        yahoo_fetch_asset("DOESNTEXIST").await.unwrap_err();
    }

    #[tokio::test]
    async fn get_price_history() {
        // January 1, 2025 at 00:00:00
        let start = OffsetDateTime::from_unix_timestamp(1_735_689_600).unwrap();
        // January 1, 2026 at 00:00:00
        let end = OffsetDateTime::from_unix_timestamp(1_767_225_600).unwrap();
        let price_hist = yahoo_fetch_price_history("ENB.TO", start, end, YahooInterval::OneMonth)
            .await
            .unwrap();
        assert_eq!(price_hist.bars.len(), 12);
    }

    #[tokio::test]
    async fn get_trailing_prices() {
        let price_hist =
            yahoo_fetch_trailing_prices("ENB.TO", YahooInterval::OneMonth, YahooRange::SixMonths)
                .await
                .unwrap();
        assert_eq!(price_hist.bars.len(), 6);
    }
}
