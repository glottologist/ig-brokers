use serde::Deserialize;

#[derive(Debug, Default)]
pub struct SentimentQuery {
	pub market_ids: Option<Vec<String>>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sentiments {
	pub client_sentiments: Vec<Sentiment>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sentiment {
	pub long_position_percentage: f64,
	pub market_id: String,
	pub short_position_percentage: f64
}
