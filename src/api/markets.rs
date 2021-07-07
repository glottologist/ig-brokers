// use crate::api::{Client, IG};
// use crate::models::{Markets as Mkts, MarketDetailsReq, MarketDetailsRes, MarketDetails};
// use chrono::NaiveDate;
// use std::collections::HashMap;

// pub struct Markets {
// 	client: Client
// }

// impl Markets {
// 	/// GET /marketnavigation
// 	/// Returns all top-level nodes (market categories) in the market navigation hierarchy.
// 	pub fn get_market_categories(&self) -> Mkts {
// 		let endpoint = String::from("/marketnavigation");
// 		self.client.get_signed(&endpoint)
// 	}

// 	/// GET /marketnavigation/{nodeId}
// 	/// Returns all sub-nodes of the given node in the market navigation hierarchy.
// 	pub fn get_market_category(&self, node_id: &String) -> Mkts {
// 		let endpoint = format!("/marketnavigation/{}", node_id);
// 		self.client.get_signed(&endpoint)
// 	}

// 	/// GET /markets
// 	/// Returns the details of the given markets.
// 	pub fn get_markets(&self, req: &MarketDetailsReq) -> MarketDetailsRes {
// 		let params: HashMap<String, String> = HashMap::new();
// 		params.insert(String::from("epics"), req.epics.join(","));

// 		match req.filter {
// 			Some(f) => {
// 				params.insert(String::from("filter"), f.to_string());
// 			}
// 		}
		
// 		let endpoint = String::from("/markets");
// 		self.client.get_signed(&endpoint)
// 	}

// 	/// GET /markets/{epic}
// 	/// Returns the details of the given market.
// 	pub fn get_market(&self, epic: &String) -> MarketDetails {
// 		let endpoint = format!("/markets/{}", epic);
// 		self.client.get_signed(&endpoint)
// 	}

// 	/// GET /markets?searchTerm={searchTerm}
// 	pub fn find_markets(&self, search_term: &String) {
		
// 	}

// 	/// GET /prices/{epic}
// 	pub fn get_prices(&self, epic: &String) {

// 	}

// 	/// GET /prices/{epic}/{resolution}/{numPoints}
// 	pub fn get_prices_with_points(&self, epic: &String, resolution: &String, points: u32) {

// 	}

// 	/// GET /prices/{epic}/{resolution}/{startDate}/{endDate}
// 	pub fn get_prices_with_dates(&self, epic: &String, resolution: &String, start: &NaiveDate, end: &NaiveDate) {

// 	}
// }
