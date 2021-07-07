use serde::Deserialize;

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

// pub struct Application {
// 	pub allow_equities: bool,
// 	pub allow_quote_orders: bool,
// 	pub allowance_account_historical_data: f64,
// 	pub allowance_account_overall: f64,
// 	pub allowance_account_trading: f64,
// 	pub allowance_application_overall: f64,
// 	pub api_key: String,
// 	pub concurrent_subscriptions_limit: f64,
// 	pub created_date: String,
// 	pub name: String,
// 	pub status: ApplicationStatus
// }

// pub enum ApplicationStatus {
// 	Disabled,
// 	Enabled,
// 	Revoked
// }

// pub struct UpdateApplication {
// 	allowance_account_overall: f64,
// 	allowance_account_trading: f64,
// 	api_key: String,
// 	status: ApplicationStatus
// }
