use std::path::Path;

use chrono::{DateTime, Utc};
use plotters::prelude::*;
use thiserror::Error;

use crate::PriceData;

const RENDER_WIDTH: u32 = 1024;
const RENDER_HEIGHT: u32 = 768;

const CANDLESTICK_WIDTH: u32 = 15;

/// The possible errors which can be encountered when generating a plot.
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum PlottingError {
    #[error("input data for plot is empty")]
    EmptyInputError,
    #[error("date is out of range")]
    DateOutOfRangeError,
    #[error("{0}")]
    DrawingAreaBackendError(String),
    #[error("{0}")]
    DrawingAreaSharingError(String),
    #[error("{0}")]
    DrawingAreaLayoutError(String),
    #[error("failed to convert timestamp to DateTime")]
    TimestampConversionError,
}
impl<DB> From<DrawingAreaErrorKind<DB>> for PlottingError
where
    DB: Send + Sync + std::error::Error,
{
    fn from(value: DrawingAreaErrorKind<DB>) -> Self {
        let s = value.to_string();
        match value {
            DrawingAreaErrorKind::LayoutError => Self::DrawingAreaLayoutError(s),
            DrawingAreaErrorKind::SharingError => Self::DrawingAreaSharingError(s),
            DrawingAreaErrorKind::BackendError(_) => Self::DrawingAreaBackendError(s),
        }
    }
}

/// Generate a graphic displaying the given [`PriceData`] as candlesticks to the given [`Path`].
///
/// # Errors
///
/// This function returns a [`PlottingError`] if any errors are encountered while generating the
/// graphic.
pub fn render_price_candlesticks(
    price_data: &PriceData,
    out_path: &Path,
) -> Result<(), PlottingError> {
    if price_data.is_empty() {
        return Err(PlottingError::EmptyInputError);
    }

    // Get start and end dates
    let x_start = DateTime::from_timestamp_secs(
        price_data
            .bars
            .first()
            .ok_or(PlottingError::EmptyInputError)?
            .interval
            .start
            .unix_timestamp(),
    )
    .ok_or(PlottingError::TimestampConversionError)?;
    let x_end: DateTime<Utc> = DateTime::from_timestamp_secs(
        price_data
            .bars
            .last()
            .ok_or(PlottingError::EmptyInputError)?
            .interval
            .end
            .unix_timestamp(),
    )
    .ok_or(PlottingError::TimestampConversionError)?;
    // Leave some room on either side
    let x_padding = x_end.signed_duration_since(x_start) / 10;
    let padded_x_start = x_start
        .checked_sub_signed(x_padding)
        .ok_or(PlottingError::DateOutOfRangeError)?;
    let padded_x_end = x_end
        .checked_add_signed(x_padding)
        .ok_or(PlottingError::DateOutOfRangeError)?;

    // Get largest and smallest price values
    let y_min = price_data
        .bars
        .iter()
        .map(|b| b.low)
        .fold(f64::INFINITY, f64::min);
    let y_max = price_data
        .bars
        .iter()
        .map(|b| b.high)
        .fold(f64::NEG_INFINITY, f64::max);
    // Leave some room on either side
    let y_padding = (y_max - y_min) / 10f64;
    let padded_y_min = (y_min - y_padding).max(0f64);
    let padded_y_max = y_max + y_padding;

    let root = BitMapBackend::new(out_path, (RENDER_WIDTH, RENDER_HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption(
            format!("{} Price", &price_data.ticker),
            ("serif", 50.0).into_font(),
        )
        .build_cartesian_2d(padded_x_start..padded_x_end, padded_y_min..padded_y_max)?;

    chart.configure_mesh().light_line_style(WHITE).draw()?;

    // Create candlesticks from price data
    let mut candlesticks = Vec::with_capacity(price_data.bars.len());
    for bar in &price_data.bars {
        let x = DateTime::from_timestamp_secs(bar.interval.start.unix_timestamp())
            .ok_or(PlottingError::TimestampConversionError)?;
        candlesticks.push(CandleStick::new(
            x,
            bar.open,
            bar.high,
            bar.low,
            bar.close,
            GREEN,
            RED.filled(),
            CANDLESTICK_WIDTH,
        ));
    }

    chart.draw_series(candlesticks)?;
    root.present()?;
    Ok(())
}
