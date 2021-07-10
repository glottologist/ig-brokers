use crate::api::{Client, Config, IG};
use crate::models::{MarketCategory, Markets, MarketsQuery /*MarketDetailsReq, MarketDetailsRes, MarketDetails*/};
// use chrono::NaiveDate;
// use std::collections::HashMap;

pub struct Market {
	client: Client
}

impl Market {
	/// GET /marketnavigation
	/// Returns all top-level nodes (market categories) in the market navigation hierarchy.
	pub fn get_market_categories(&self) -> Result<MarketCategory, reqwest::Error> {
		let endpoint: String = "/marketnavigation".into();
		let data: MarketCategory = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// GET /marketnavigation/{nodeId}
	/// Returns all sub-nodes of the given node in the market navigation hierarchy.
	pub fn get_market_category(&self, node_id: &String) -> Result<MarketCategory, reqwest::Error> {
		let endpoint = format!("/marketnavigation/{}", node_id);
		let data: MarketCategory = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// GET /markets
	/// Returns the details of the given markets.
	pub fn get_markets(&self, query: &MarketsQuery) -> Result<Markets, reqwest::Error> {
		let endpoint: String = "/markets".into();
		let data: Markets = self.client.get_signed(&endpoint, 2, Some(query))?;
		Ok(data)
	}

	/// GET /markets/{epic}
	/// Returns the details of the given market.
	pub fn get_market(&self, epic: &String) -> Result<Market, reqwest::Error> {
		let endpoint: String = format!("/markets/{}", epic);
		let data: Market = self.client.get_signed(&endpoint, 3, None::<()>)?;
		Ok(data)
	}

// 	/// GET /markets?searchTerm={searchTerm}
// 	pub fn find_markets(&self, search_term: &String) {
		
// 	}

// 	/// GET /prices/{epic}
// 	pub fn get_prices(&self, epic: &String) {

// 	}
}

impl IG for Market {
	fn new(account_id: String, api_key: String, username: String, password: String) -> Market {
		Self::new_with_config(account_id, api_key, username, password, Config::default())
	}

	fn new_with_config(account_id: String, api_key: String, username: String, password: String, config: Config) -> Market {
		let client = Client::new(account_id, api_key, username, password, config);
		Market { client }
	}
}
