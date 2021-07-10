use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accounts {
	pub accounts: Vec<Account>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
	pub account_alias: Option<String>,
	pub account_id: String,
	pub account_name: String,
	pub account_type: AccountType,
	pub balance: Balance,
	pub can_transfer_from: bool,
	pub can_transfer_to: bool,
	pub currency: String,
	pub preferred: bool,
	pub status: AccountStatus
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
	Cfd,
	Physical,
	Spreadbet
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
	pub available: f64,
	pub balance: f64,
	pub deposit: f64,
	pub profit_loss: f64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
	Disabled,
	Enabled,
	SuspendedFromDealing
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
	pub trailing_stops_enabled: bool
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityHistoryQuery {
	pub from: Option<NaiveDateTime>,
	pub to: Option<NaiveDateTime>,
	pub detailed: Option<bool>,
	pub deal_id: Option<String>,
	pub filter: Option<String>,
	pub page_size: Option<u32>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityHistory {
	pub activities: Vec<Activity>,
	pub metadata: ActivityMetadata
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
	pub channel: Channel,
	pub date: String,
	pub deal_id: String,
	pub description: String,
	pub details: Option<ActivityDetails>,
	pub epic: String,
	pub period: String,
	pub status: ActivityStatus,
	pub r#type: ActivityType
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Channel {
	Dealer,
	Mobile,
	PublicFixApi,
	PublicWebApi,
	System,
	Web
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDetails {
	pub actions: Vec<ActivityAction>,
	pub currency: String,
	pub deal_reference: String,
	pub direction: Direction,
	pub good_till_date: String,
	pub guaranteed_stop: bool,
	pub level: f64,
	pub limit_distance: f64,
	pub limit_level: f64,
	pub market_name: String,
	pub size: f64,
	pub stop_distance: f64,
	pub stop_level: f64,
	pub trailing_step: f64,
	pub trailing_stop_distance: f64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityAction {
	pub action_type: ActivityActionType,
	pub affected_deal_id: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityActionType {
	LimitOrderAmended,
	LimitOrderDeleted,
	LimitOrderFilled,
	LimitOrderOpened,
	LimitOrderRolled,
	PositionClosed,
	PositionDeleted,
	PositionOpened,
	PositionPartiallyClosed,
	PositionRolled,
	StopLimitAmended,
	StopOrderAmended,
	StopOrderDeleted,
	StopOrderFilled,
	StopOrderOpened,
	StopOrderRolled,
	Unknown,
	WorkingOrderDeleted
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
	Buy,
	Sell
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityStatus {
	Accepted,
	Rejected,
	Unknown
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
	EditStopAndLimit,
	Position,
	System,
	WorkingOrder
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMetadata {
	pub paging: ActivityPaging
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPaging {
	pub next: Option<String>,
	pub size: u32
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHistoryQuery {
	pub r#type: Option<TransactionType>,
	pub from: Option<NaiveDateTime>,
	pub to: Option<NaiveDateTime>,
	pub max_span_seconds: Option<u64>,
	pub page_size: Option<u32>,
	pub page_number: Option<u32>
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
	All,
	AllDeal,
	Deposit,
	Withdrawal
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHistory {
	pub metadata: TransactionMetadata,
	pub transactions: Vec<Transaction>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMetadata {
	pub page_data: TransactionPaging,
	pub size: u32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionPaging {
	pub page_number: u32,
	pub page_size: u32,
	pub total_pages: u32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
	pub cash_transaction: bool,
	pub close_level: String,
	pub currency: String,
	pub date: String,
	pub date_utc: String,
	pub instrument_name: String,
	pub open_date_utc: String,
	pub open_level: String,
	pub period: String,
	pub profit_and_loss: String,
	pub reference: String,
	pub size: String,
	pub transaction_type: String
}
