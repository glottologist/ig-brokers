use crate::api::{Client, Config, IG};
use crate::models::{AccountRes};
use chrono::NaiveDate;
use std::collections::HashMap;

pub struct Account {
	client: Client
}

impl Account {
	/// GET /accounts
	/// Returns a list of accounts belonging to the logged-in client.
	pub fn get_accounts(&self) -> Result<AccountRes, reqwest::Error> {
		let endpoint: String = "/accounts".into();
		let data = self.client.get_signed::<AccountRes>(&endpoint)?;
		Ok(data)
	}

	// /// GET /accounts/preferences
	// /// Returns account preferences.
	// pub fn get_preferences(&self) -> Preferences {
	// 	let endpoint = String::from("/accounts/preferences");
	// 	self.client.get_signed(&endpoint)
	// }

	// /// PUT /accounts/preferences
	// /// Updates account preferences.
	// pub fn update_preferences(&self, preferences: &Preferences) -> ResponseStatus {
	// 	let endpoint = String::from("/accounts/preferences");
	// 	self.client.put_signed(&endpoint, &Some(preferences))
	// }

	// /// GET /history/activity
	// /// Returns the account activity history.
	// pub fn get_account_history(&self) -> AccountHistory {
	// 	let endpoint = String::from("/history/activity");
	// 	self.client.get_signed(&endpoint)
	// }

	// /// GET /history/activity/{fromDate}/{toDate}
	// /// Returns the account activity history for the given date range.
	// pub fn get_account_history_with_dates(&self, from: &NaiveDate, to: &NaiveDate) -> AccountHistory2 {
	// 	let endpoint = format!("/history/activity/{}/{}", from, to);
	// 	self.client.get_signed(&endpoint)
	// }

	// /// GET /history/activity/{lastPeriod}
	// /// Returns the account activity history for the last specified period.
	// pub fn get_account_history_with_period(&self, period: &String) -> AccountHistory2 {
	// 	let endpoint = format!("/history/activity/{}", period);
	// 	self.client.get_signed(&endpoint)
	// }

	// /// GET /history/transactions
	// /// Returns the transaction history.
	// /// By default returns the minute prices within the last 10 minutes.
	// pub fn get_transaction_history(&self, req: &TransactionHistoryParams) -> TransactionHistory {
	// 	let params: HashMap<String, String> = HashMap::new();
	// 	params.insert(String::from("type"), req.r#type.to_string());
	// 	params.insert(String::from("from"), req.from);
	// 	params.insert(String::from("to"), req.to);
	// 	params.insert(String::from("maxSpanSeconds"), req.max_span_seconds.to_string());
	// 	params.insert(String::from("pageSize"), req.page_size.to_string());
	// 	params.insert(String::from("page_number"), req.page_number.to_string());

	// 	let endpoint = String::from("/history/transactions");
	// 	self.client.get_with_params_signed(&endpoint, &params)
	// }

	// /// GET /history/transactions/{transactionType}/{fromDate}/{toDate}
	// /// Returns the transaction history for the specified transaction type and given date range.
	// pub fn get_transaction_history_with_dates(&self, ttype: &String, from: &NaiveDate, to: &NaiveDate) -> TransactionHistory2 {
	// 	let endpoint = format!("/history/transactions/{}/{}/{}", ttype, from.to_string(), to.to_string());
	// 	self.client.get_signed(&endpoint)
	// }

	// /// GET /history/transactions/{transactionType}/{lastPeriod}
	// /// Returns the transaction history for the specified transaction type and period.
	// pub fn get_transaction_history_with_period(&self, ttype: &String, period: &String) -> TransactionHistory2 {
	// 	let endpoint = format!("/history/transactions/{}/{}", ttype, period);
	// 	self.client.get_signed(&endpoint)
	// }
}

impl IG for Account {
	fn new(account_id: String, api_key: String, username: String, password: String) -> Account {
		Self::new_with_config(account_id, api_key, username, password, Config::default())
	}

	fn new_with_config(account_id: String, api_key: String, username: String, password: String, config: Config) -> Account {
		let client = Client::new(account_id, api_key, username, password, config);
		Account { client }
	}
}
