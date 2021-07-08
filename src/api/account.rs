use crate::api::{Client, Config, IG};
use crate::models::{
	ActivityHistory,
	ActivityHistoryQuery,
	Accounts,
	Preferences,
	OkResponse,
	TransactionHistory,
	TransactionHistoryQuery
};

pub struct Account {
	client: Client
}

impl Account {
	/// GET /accounts
	/// Returns a list of accounts belonging to the logged-in client.
	pub fn get_accounts(&self) -> Result<Accounts, reqwest::Error> {
		let endpoint: String = "/accounts".into();
		let data: Accounts = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// GET /accounts/preferences
	/// Returns account preferences.
	pub fn get_preferences(&self) -> Result<Preferences, reqwest::Error> {
		let endpoint: String = "/accounts/preferences".into();
		let data: Preferences = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// PUT /accounts/preferences
	/// Updates account preferences.
	pub fn update_preferences(&self, preferences: &Preferences) -> Result<OkResponse, reqwest::Error> {
		let endpoint: String = "/accounts/preferences".into();
		let data: OkResponse = self.client.put_signed(&endpoint, 1, Some(preferences))?;
		Ok(data)
	}

	/// GET /history/activity
	/// Returns the account activity history.
	pub fn get_activity_history(&self, query: &ActivityHistoryQuery) -> Result<ActivityHistory, reqwest::Error> {
		let endpoint: String = "/history/activity".into();
		let data: ActivityHistory = self.client.get_signed(&endpoint, 3, Some(query))?;
		Ok(data)
	}

	/// GET /history/transactions
	/// Returns the transaction history.
	/// By default returns the minute prices within the last 10 minutes.
	pub fn get_transaction_history(&self, query: &TransactionHistoryQuery) -> Result<TransactionHistory, reqwest::Error> {
		let endpoint: String = "/history/transactions".into();
		let data: TransactionHistory = self.client.get_signed(&endpoint, 2, Some(query))?;
		Ok(data)
	}
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
