use crate::api::{Client, Config, IG};
use crate::models::{Account as Acc, History, History2, Preferences, ResponseStatus};
use chrono::NaiveDate;

pub struct Account {
	client: Client
}

impl Account {
	/// GET /accounts
	/// Returns a list of accounts belonging to the logged-in client.
	pub fn get_accounts(&self) -> Vec<Acc> {
		let endpoint = String::from("/accounts");
		self.client.get_signed(&endpoint)
	}

	/// GET /accounts/preferences
	/// Returns account preferences.
	pub fn get_preferences(&self) -> Preferences {
		let endpoint = String::from("/accounts/preferences");
		self.client.get_signed(&endpoint)
	}

	/// PUT /accounts/preferences
	/// Updates account preferences.
	pub fn update_preferences(&self, preferences: &Preferences) -> ResponseStatus {
		let endpoint = String::from("/accounts/preferences");
		self.client.put_signed(&endpoint, preferences)
	}

	/// GET /history/activity
	/// Returns the account history.
	pub fn get_account_history(&self) -> History {
		let endpoint = String::from("/history/activity");
		self.client.get_signed(&endpoint)
	}

	/// GET /history/activity/{fromDate}/{toDate}
	pub fn get_account_history_with_dates(&self, from: &NaiveDate, to: &NaiveDate) -> History2 {
		let endpoint = format!("/history/activity/{}/{}", from, to);
		self.client.get_signed(&endpoint)
	}

	/// GET /history/activity/{lastPeriod}
	pub fn get_account_history_with_period(&self, period: &String) {
		let endpoint = format!("/history/activity/{}", period);
		self.client.get_signed(&endpoint)
	}

	/// GET /history/transactions
	pub fn get_transaction_history(&self) {

	}

	/// GET /history/transactions/{transactionType}/{fromDate}/{toDate}
	pub fn get_transaction_history_with_dates(&self, ttype: &String, from: &NaiveDate, to: &NaiveDate) {

	}

	/// GET /history/transactions/{transactionType}/{lastPeriod}
	pub fn get_transaction_history_with_period(&self, ttype: &String, period: &String) {

	}
}

impl IG for Account {
	fn new(api_key: Option<String>, api_secret: Option<String>) -> Account {
		Self::new_with_config(api_key, api_secret, Config::default())
	}

	fn new_with_config(api_key: Option<String>, api_secret: Option<String>, config: Config) -> Account {
		let client = Client::new(api_key, api_secret, config);
		Account { client }
	}
}
