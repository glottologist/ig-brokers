use crate::models::{InstrumentType, MarketStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketCategory {
	pub markets: Option<Vec<MarketData3>>,
	pub nodes: Option<Vec<MarketNode>>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketData3 {
	pub bid: f64,
	pub delay_time: f64,
	pub epic: String,
	pub expiry: String,
	pub high: f64,
	pub instrument_name: String,
	pub instrument_type: InstrumentType,
	pub lot_size: f64,
	pub low: f64,
	pub market_status: MarketStatus,
	pub net_change: f64,
	pub offer: f64,
	pub otc_tradeable: bool,
	pub percentage_change: f64,
	pub scaling_factor: f64,
	pub streaming_prices_available: bool,
	pub update_time: String,
	pub update_time_utc: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketNode {
	pub id: String,
	pub name: String
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketsQuery {
	pub epics: Vec<String>,
	pub filter: Option<MarketDetailsFilterType>
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketDetailsFilterType {
	All,
	SnapshotOnly
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Markets {
	pub market_details: Vec<Market>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
	pub dealing_rules: DealingRules,
	pub instrument: InstrumentDetails,
	pub snapshot: MarketSnapshot
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealingRules {
	pub market_order_preference: MarketOrderPreference,
	pub max_stop_or_limit_distance: DealingRule,
	pub min_controlled_risk_stop_distance: DealingRule,
	pub min_deal_size: DealingRule,
	pub min_normal_stop_or_limit_distance: DealingRule,
	pub min_step_distance: DealingRule,
	pub trailing_stops_preference: TrailingStopsPreference
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketOrderPreference {
	AvailableDefaultOff,
	AvailableDefaultOn,
	NotAvailable
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DealingRule {
	pub unit: RuleUnit,
	pub value: f64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuleUnit {
	Percentage,
	Points
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrailingStopsPreference {
	Available,
	NotAvailable
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentDetails {
	pub chart_code: String,
	pub contract_size: String,
	pub controlled_risk_allowed: bool,
	pub country: String,
	pub currencies: Vec<Currency>,
	pub epic: String,
	pub expiry: String,
	pub expiry_details: Expiry,
	pub force_open_allowed: bool,
	pub limited_risk_premium: DealingRule,
	pub lot_size: f64,
	pub margin_deposit_bands: Vec<DepositBand>,
	pub margin_factor: f64,
	pub margin_factor_unit: RuleUnit,
	pub market_id: String,
	pub name: String,
	pub news_code: String,
	pub ope_pip_means: String,
	pub opening_hours: OpeningHours,
	pub slippage_factor: SlippageFactor,
	pub special_info: Vec<String>,
	pub sprint_markets_maximum_expiry_time: f64,
	pub sprint_markets_minimum_expiry_time: f64,
	pub stop_limits_allowed: bool,
	pub streaming_prices_available: bool,
	pub r#type: InstrumentType,
	pub unit: InstrumentUnit,
	pub value_of_one_pip: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
	pub base_exchange_rate: f64,
	pub code: String,
	pub exchange_rate: f64,
	pub is_default: bool,
	pub symbol: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expiry {
	pub last_dealing_date: String,
	pub settlement_info: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositBand {
	pub currency: String,
	pub margin: f64,
	pub max: f64,
	pub min: f64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpeningHours {
	pub market_times: Vec<MarketTime>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketTime {
	pub close_time: String,
	pub open_time: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlippageFactor {
	pub unit: String,
	pub value: f64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentUnit {
	Amount,
	Contracts,
	Shares
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketSnapshot {
	pub bid: f64,
	pub binary_odds: f64,
	pub controlled_risk_extra_spread: f64,
	pub decimal_places_factor: f64,
	pub delay_time: f64,
	pub high: f64,
	pub low: f64,
	pub market_status: MarketStatus,
	pub net_change: f64,
	pub offer: f64,
	pub percentage_change: f64,
	pub scaling_factor: f64,
	pub update_time: String
}
