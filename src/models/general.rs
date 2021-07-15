use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OkResponse {
	pub status: ResponseStatus
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseStatus {
	Success
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
	pub allow_equities: bool,
	pub allow_quote_orders: bool,
	pub allowance_account_historical_data: f64,
	pub allowance_account_overall: f64,
	pub allowance_account_trading: f64,
	pub allowance_application_overall: f64,
	pub api_key: String,
	pub concurrent_subscriptions_limit: f64,
	pub created_date: String,
	pub name: String,
	pub status: ApplicationStatus
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApplicationStatus {
	Disabled,
	Enabled,
	Revoked
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApplication {
	pub allowance_account_overall: f64,
	pub allowance_account_trading: f64,
	pub api_key: String,
	pub status: ApplicationStatus
}
