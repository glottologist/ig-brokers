pub struct ClientSentiment {
	pub client_sentiments: Vec<Sentiment>
}

pub struct Sentiment {
	pub long_position_percentage: f64,
	pub market_id: String,
	pub short_position_percentage: f64
}
