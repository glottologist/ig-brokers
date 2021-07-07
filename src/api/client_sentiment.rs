// use crate::api::{Client, Config, IG};
// use crate::models::{ClientSentiment as CSentiment, Sentiment};
// use std::collections::HashMap;

// pub struct ClientSentiment {
// 	client: Client
// }

// impl ClientSentiment {
// 	/// GET /clientsentiment
// 	/// Returns the client sentiment for the given instrument's market.
// 	pub fn get_client_sentiments(&self, market_ids: Option<Vec<String>>) -> CSentiment {
// 		let params: HashMap<String, String> = HashMap::new();
// 		match market_ids {
// 			Some(ids) => {
// 				params.insert(String::from("marketIds"), ids.join(","));
// 			}
// 		};		

// 		let endpoint = String::from("/clientsentiment");
// 		self.client.get_with_params_signed(&endpoint, &params)
// 	}

// 	/// GET /clientsentiment/{marketId}
// 	/// Returns the client sentiment for the given instrument's market.
// 	pub fn get_client_sentiment(&self, market_id: &String) -> Sentiment {
// 		let endpoint = format!("/clientsentiment/{}", market_id);
// 		self.client.get_signed(&endpoint)
// 	}

// 	/// GET /clientsentiment/related/{marketId}
// 	/// Returns a list of related (what others have traded) client sentiment for the given instrument's market.
// 	pub fn get_related_client_sentiment(&self, market_id: &String) -> CSentiment {
// 		let endpoint = format!("/clientsentiment/related/{}", market_id);
// 		self.client.get_signed(&endpoint)
// 	}
// }

// impl IG for ClientSentiment {
// 	fn new(api_key: Option<String>, api_secret: Option<String>) -> ClientSentiment {
// 		Self::new_with_config(api_key, api_secret, Config::default())
// 	}

// 	fn new_with_config(api_key: Option<String>, api_secret: Option<String>, config: Config) -> ClientSentiment {
// 		let client = Client::new(api_key, api_secret, config);
// 		ClientSentiment { client }
// 	}
// }
