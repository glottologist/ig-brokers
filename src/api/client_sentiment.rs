use crate::api::{Client, Config, IG};
use crate::models::{Sentiment, SentimentQuery, Sentiments};

pub struct ClientSentiment {
	client: Client
}

impl ClientSentiment {
	/// GET /clientsentiment
	/// Returns the client sentiment for the given instrument's market.
	pub fn get_client_sentiments(&self, query: &SentimentQuery) -> Result<Sentiments, reqwest::Error> {
		let endpoint: String = "/clientsentiment".into();
		let data: Sentiments = self.client.get_signed(&endpoint, 1, Some(query))?;
		Ok(data)
	}

	/// GET /clientsentiment/{marketId}
	/// Returns the client sentiment for the given instrument's market.
	pub fn get_client_sentiment(&self, market_id: &String) -> Result<Sentiment, reqwest::Error> {
		let endpoint = format!("/clientsentiment/{}", market_id);
		let data: Sentiment = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}

	/// GET /clientsentiment/related/{marketId}
	/// Returns a list of related (what others have traded) client sentiment for the given instrument's market.
	pub fn get_related_client_sentiment(&self, market_id: &String) -> Result<Sentiments, reqwest::Error> {
		let endpoint = format!("/clientsentiment/related/{}", market_id);
		let data: Sentiments = self.client.get_signed(&endpoint, 1, None::<()>)?;
		Ok(data)
	}
}

impl IG for ClientSentiment {
	fn new(account_id: String, api_key: String, username: String, password: String) -> ClientSentiment {
		Self::new_with_config(account_id, api_key, username, password, Config::default())
	}

	fn new_with_config(account_id: String, api_key: String, username: String, password: String, config: Config) -> ClientSentiment {
		let client = Client::new(account_id, api_key, username, password, config);
		ClientSentiment { client }
	}
}
