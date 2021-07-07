// use std::fmt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRes {
	pub accounts: Vec<AccountData>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
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

// pub struct Preferences {
// 	pub trailing_stops_enabled: bool
// }

// pub struct AccountHistory {
// 	pub activities: Vec<AccountActivity>,
// 	pub metadata: Metadata
// }

// pub struct AccountHistory2 {
// 	pub activities: Vec<AccountActivity2>
// }

// pub struct AccountActivity {
// 	pub channel: Channel,
// 	pub date: String,
// 	pub deal_id: String,
// 	pub description: String,
// 	pub details: ActivityDetails,
// 	pub epic: String,
// 	pub period: String,
// 	pub status: ActivityStatus,
// 	pub r#type: ActivityType
// }

// pub struct AccountActivity2 {
// 	pub action_status: String,
// 	pub activity: String,
// 	pub activity_history_id: String,
// 	pub channel: String,
// 	pub currency: String,
// 	pub date: String,
// 	pub deal_id: String,
// 	pub epic: String,
// 	pub level: String,
// 	pub limit: String,
// 	pub market_name: String,
// 	pub period: String,
// 	pub result: String,
// 	pub size: String,
// 	pub stop: String,
// 	pub stop_type: String,
// 	pub time: String
// }

// pub enum Channel {
// 	Dealer,
// 	Mobile,
// 	PublicFixApi,
// 	PublicWebApi,
// 	System,
// 	Web
// }

// pub struct ActivityDetails {
// 	pub actions: Vec<ActivityAction>,
// 	pub currency: String,
// 	pub deal_reference: String,
// 	pub direction: Direction,
// 	pub good_till_date: String,
// 	pub guaranteed_stop: bool,
// 	pub level: f64,
// 	pub limit_distance: f64,
// 	pub limit_level: f64,
// 	pub market_name: String,
// 	pub size: f64,
// 	pub stop_distance: f64,
// 	pub stop_level: f64,
// 	pub trailing_step: f64,
// 	pub trailing_stop_distance: f64
// }

// pub struct ActivityAction {
// 	pub action_type: ActivityActionType,
// 	pub affected_deal_id: String
// }

// pub enum ActivityActionType {
// 	LimitOrderAmended,
// 	LimitOrderDeleted,
// 	LimitOrderFilled,
// 	LimitOrderOpened,
// 	LimitOrderRolled,
// 	PositionClosed,
// 	PositionDeleted,
// 	PositionOpened,
// 	PositionPartiallyClosed,
// 	PositionRolled,
// 	StopLimitAmended,
// 	StopOrderAmended,
// 	StopOrderDeleted,
// 	StopOrderFilled,
// 	StopOrderOpened,
// 	StopOrderRolled,
// 	Unknown,
// 	WorkingOrderDeleted
// }

pub enum Direction {
	Buy,
	Sell
}

// pub enum ActivityStatus {
// 	Accepted,
// 	Rejected,
// 	Unknown
// }

// pub enum ActivityType {
// 	EditStopAndLimit,
// 	Position,
// 	System,
// 	WorkingOrder
// }

// pub struct Metadata {
// 	pub pading: Paging
// }

// pub struct Paging {
// 	pub next: String,
// 	pub size: u32
// }

// pub struct TransactionHistoryParams {
// 	pub r#type: TransactionType,
// 	pub from: String,
// 	pub to: String,
// 	pub max_span_seconds: u64,
// 	pub page_size: u32,
// 	pub page_number: u32
// }

// pub enum TransactionType {
// 	All,
// 	AllDeal,
// 	Deposit,
// 	Withdrawal
// }

// impl fmt::Display for TransactionType {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		match self {
// 			TransactionType::All => write!(f, "ALL"),
// 			TransactionType::AllDeal => write!(f, "ALL_DEAL"),
// 			TransactionType::Deposit => write!(f, "DEPOSIT"),
// 			TransactionType::Withdrawal => write!(f, "WITHDRAWAL")
// 		}
// 	}
// }

// pub struct TransactionHistory {
// 	pub metadata: TransactionHistoryMetadata,
// 	pub transaction: Vec<Transaction>
// }

// pub struct TransactionHistoryMetadata {
// 	pub page_data: PagingMetadata,
// 	pub size: u32
// }

// pub struct PagingMetadata {
// 	pub page_number: u32,
// 	pub page_size: u32,
// 	pub total_pages: u32
// }

// pub struct Transaction {
// 	pub cash_transaction: bool,
// 	pub close_level: String,
// 	pub currency: String,
// 	pub date: String,
// 	pub date_utc: String,
// 	pub instrument_name: String,
// 	pub open_date_utc: String,
// 	pub open_level: String,
// 	pub period: String,
// 	pub profit_and_loss: String,
// 	pub reference: String,
// 	pub size: String,
// 	pub transaction_type: String
// }

// pub struct TransactionHistory2 {
// 	pub transactions: Vec<Transaction2>
// }

// pub struct Transaction2 {
// 	cash_transaction: bool,
// 	close_level: String,
// 	currency: String,
// 	date: String,
// 	instrument_name: String,
// 	open_level: String,
// 	period: String,
// 	profit_and_loss: String,
// 	reference: String,
// 	size: String,
// 	transaction_type: String
// }
