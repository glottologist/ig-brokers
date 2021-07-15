
use crate::client::Client;
use crate::config::Config;
use crate::models::*;
use reqwest::Error;

pub struct IG {
	client: Client
}

impl IG {
	/// Creates a new API instance defaulting to the production configuration.
	pub fn new(account_id: String, api_key: String, username: String, password: String) -> IG {
		IG::new_with_config(account_id, api_key, username, password, Config::live())
	}

	/// Creates a new API instance with a config.
	pub fn new_with_config(account_id: String, api_key: String, username: String, password: String, config: Config) -> IG {
		let client = Client::new(account_id, api_key, username, password, config);
		IG { client }
	}

	/// GET /accounts
	/// Returns a list of accounts belonging to the logged-in client.
	pub fn get_accounts(&self) -> Result<Accounts, Error> {
		let endpoint: String = "/accounts".into();
		let data: Accounts = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// GET /accounts/preferences
	/// Returns account preferences.
	pub fn get_preferences(&self) -> Result<Preferences, Error> {
		let endpoint: String = "/accounts/preferences".into();
		let data: Preferences = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// PUT /accounts/preferences
	/// Updates account preferences.
	pub fn update_preferences(&self, preferences: &Preferences) -> Result<OkResponse, Error> {
		let endpoint: String = "/accounts/preferences".into();
		let data: OkResponse = self.client.put_signed(&endpoint, 1, Some(preferences))?;
		Ok(data)
	}

	/// GET /history/activity
	/// Returns the account activity history.
	pub fn get_activity_history(&self, query: &ActivityHistoryQuery) -> Result<ActivityHistory, Error> {
		let endpoint: String = "/history/activity".into();
		let data: ActivityHistory = self.client.get_signed(&endpoint, 3, Some(query))?;
		Ok(data)
	}

	/// GET /history/transactions
	/// Returns the transaction history.
	/// By default returns the minute prices within the last 10 minutes.
	pub fn get_transaction_history(&self, query: &TransactionHistoryQuery) -> Result<TransactionHistory, Error> {
		let endpoint: String = "/history/transactions".into();
		let data: TransactionHistory = self.client.get_signed(&endpoint, 2, Some(query))?;
		Ok(data)
	}

	/// GET /clientsentiment
	/// Returns the client sentiment for the given instrument's market.
	pub fn get_client_sentiments(&self, query: &SentimentQuery) -> Result<Sentiments, Error> {		
		let endpoint = "/clientsentiment".to_string();
		let data: Sentiments = self.client.get_signed(&endpoint, 1, Some(query))?;
		Ok(data)
	}

	/// GET /clientsentiment/{marketId}
	/// Returns the client sentiment for the given instrument's market.
	pub fn get_client_sentiment(&self, market_id: &String) -> Result<Sentiment, Error> {
		let endpoint = format!("/clientsentiment/{}", market_id);
		let data: Sentiment = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// GET /clientsentiment/related/{marketId}
	/// Returns a list of related (what others have traded) client sentiment for the given instrument's market.
	pub fn get_related_client_sentiment(&self, market_id: &String) -> Result<Sentiments, Error> {
		let endpoint = format!("/clientsentiment/related/{}", market_id);
		let data: Sentiments = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// GET /confirms/{dealReference}
	/// Returns a deal confirmation for the given deal reference.
	pub fn get_deal_confirmation(&self, deal_reference: &String) -> Result<DealConfirmation, Error> {
		let endpoint = format!("/confirms/{}", deal_reference);
		let data: DealConfirmation = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

// 	/// GET /positions
// 	/// Returns all open positions for the active account.
// 	pub fn get_positions(&self) -> Positions {
// 		let endpoint = String::from("/positions");
// 		self.client.get_signed(&endpoint)
// 	}

// 	/// GET /positions/{dealId}
// 	/// Returns an open position for the active account by deal identifier.
// 	pub fn get_positions_with_deal(&self, deal_id: &String) -> Position {
// 		let endpoint = format!("/positions/{}", deal_id);
// 		self.client.get_signed(&endpoint)
// 	}

	/// DELETE /positions/otc
	/// Closes one or more OTC positions.
	pub fn close_position(&self, req: &ClosePosition) -> Result<DealRef, Error> {
		let endpoint: String = "/positions/otc".into();
		let data: DealRef = self.client.delete_signed(&endpoint, 1, Some(req))?;
		Ok(data)
	}

	/// POST /positions/otc
	/// Creates an OTC position.
	pub fn create_position(&self, req: &CreatePosition) -> Result<DealRef, Error> {
		let endpoint: String = "/positions/otc".into();
		let data: DealRef = self.client.post_signed(&endpoint, 2, Some(req))?;
		Ok(data)
	}

// 	/// PUT /positions/otc/{dealId}
// 	/// Updates an OTC position.
// 	pub fn update_position(&self, deal_id: &String, req: &UpdatePosition) {
// 		let endpoint = format!("/positions/otc/{}", deal_id);
// 		self.client.put_signed(&endpoint, &Some(req))
// 	}

// 	/// GET /positions/sprintmarkets
// 	/// A list of sprint market positions.
// 	pub fn get_sprint_market_positions(&self) -> SprintMarketPositions {
// 		let endpoint = String::from("/positions/sprintmarkets");
// 		self.client.get_signed(&endpoint)
// 	}

// 	/// POST /positions/sprintmarkets
// 	/// Creates a sprint market position.
// 	pub fn create_sprint_market_position(&self, req: &CreateSprintMarketPosition) -> DealRef {
// 		let endpoint = String::from("/positions/sprintmarkets");
// 		self.client.post_signed(&endpoint, &req)
// 	}

// 	/// GET /workingorders
// 	/// Returns all open working orders for the active account.
// 	pub fn get_working_orders(&self) -> WorkingOrders {
// 		let endpoint = String::from("/workingorders");
// 		self.client.get_signed(&endpoint)
// 	}

// 	/// POST /workingorders/otc
// 	/// Creates an OTC working order.
// 	pub fn create_working_order(&self, req: &CreateWorkingOrder) -> DealRef {
// 		let endpoint = String::from("/workingorders/otc");
// 		self.client.post_signed(&endpoint, req)
// 	}

// 	/// DELETE /workingorders/otc/{dealId}
// 	/// Deletes an OTC working order.
// 	pub fn delete_working_order(&self, deal_id: &String) -> DealRef {
// 		let endpoint = format!("/workingorders/otc/{}", deal_id);
// 		self.client.delete_signed(&endpoint, &None::<()>)
// 	}

// 	/// PUT /workingorders/otc/{dealId}
// 	/// Updates an OTC working order.
// 	pub fn update_working_order(&self, deal_id: &String, req: &UpdateWorkingOrder) -> DealRef {
// 		let endpoint = format!("/workingorders/otc/{}", deal_id);
// 		self.client.put_signed(&endpoint, &Some(req))
// 	}

	/// GET /operations/application
	/// Returns a list of client owned applications.
	pub fn get_applications(&self) -> Result<Vec<Application>, Error> {
		let endpoint = "/operations/application".to_string();
		let data: Vec<Application> = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// PUT /operations/application
	/// Alters the details of a given user application.
	pub fn update_application(&self, req: &UpdateApplication) -> Result<Application, Error> {
		let endpoint = "/operations/application".to_string();
		let data: Application = self.client.put_signed(&endpoint, 1, Some(req))?;
		Ok(data)
	}

	/// GET /session
	/// Returns the user's session details and optionally tokens.
	pub fn get_session(&self) -> Result<Session, Error> {
		let endpoint = "/session".to_string();
		let data: Session = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// GET /marketnavigation
	/// Returns all top-level nodes (market categories) in the market navigation hierarchy.
	pub fn get_market_categories(&self) -> Result<MarketCategory, Error> {
		let endpoint: String = "/marketnavigation".into();
		let data: MarketCategory = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// GET /marketnavigation/{nodeId}
	/// Returns all sub-nodes of the given node in the market navigation hierarchy.
	pub fn get_market_category(&self, node_id: &String) -> Result<MarketCategory, Error> {
		let endpoint = format!("/marketnavigation/{}", node_id);
		let data: MarketCategory = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// GET /markets
	/// Returns the details of the given markets.
	pub fn get_markets(&self, query: &MarketsQuery) -> Result<Markets, Error> {
		let endpoint: String = "/markets".into();
		let data: Markets = self.client.get_signed(&endpoint, 2, Some(query))?;
		Ok(data)
	}

	/// GET /markets/{epic}
	/// Returns the details of the given market.
	pub fn get_market(&self, epic: &String) -> Result<Market, Error> {
		let endpoint = format!("/markets/{}", epic);
		let data: Market = self.client.get_signed(&endpoint, 3, None::<()>)?;
		Ok(data)
	}

	/// GET /markets?searchTerm={searchTerm}
	/// Returns all markets matching the search term.
	pub fn search_markets(&self, search_term: &String) -> Result<MarketSearch, Error> {
		let endpoint = format!("/markets?searchTerm={}", search_term);
		let data: MarketSearch = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// GET /prices/{epic}
	/// Returns historical prices for a particular instrument.
	/// By default returns the minute prices within the last 10 minutes.
	pub fn get_prices(&self, epic: &String, query: &PricesQuery) -> Result<Prices, Error> {
		let endpoint = format!("/prices/{}", epic);
		let data: Prices = self.client.get_signed(&endpoint, 3, Some(query))?;
		Ok(data)
	}

	/// GET /watchlists
	/// Returns all watchlists belonging to the active account
	pub fn get_watchlists(&self) -> Result<Watchlists, Error> {
		let endpoint = "/watchlists".to_string();
		let data: Watchlists = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// POST /watchlists
	/// Creates a watchlist.
	pub fn create_watchlist(&self, req: &CreateWatchlist) -> Result<CreateWatchlistResult, Error> {
		let endpoint = "/watchlists".to_string();
		let data: CreateWatchlistResult = self.client.post_signed(&endpoint, 1, Some(req))?;
		Ok(data)
	}

	/// DELETE /watchlists/{watchlistId}
	/// Deletes a watchlist.
	pub fn delete_watchlist(&self, watchlist_id: &String) -> Result<OkResponse, Error> {
		let endpoint = format!("/watchlists/{}", watchlist_id);
		let data: OkResponse = self.client.delete_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// GET /watchlists/{watchlistId}
	/// Returns a watchlist.
	pub fn get_watchlist(&self, watchlist_id: &String) -> Result<MarketSearch, Error> {
		let endpoint = format!("/watchlists/{}", watchlist_id);
		let data: MarketSearch = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// PUT /watchlists/{watchlistId}
	/// Add a market to watchlist.
	pub fn add_market_to_watchlist(&self, watchlist_id: &String, req: &AddToWatchlist) -> Result<OkResponse, Error> {
		let endpoint = format!("/watchlists/{}", watchlist_id);
		let data: OkResponse = self.client.put_signed(&endpoint, 1, Some(req))?;
		Ok(data)
	}

	/// DELETE /watchlists/{watchlistId}/{epic}
	/// Remove a market from a watchlist.
	pub fn remove_market_from_watchlist(&self, watchlist_id: &String, epic: &String) -> Result<OkResponse, Error> {
		let endpoint = format!("/watchlists/{}/{}", watchlist_id, epic);
		let data: OkResponse = self.client.delete_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}
}
