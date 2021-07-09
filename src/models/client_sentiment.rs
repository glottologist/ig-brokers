use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeSeq;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SentimentQuery {
	#[serde(serialize_with = "to_csv")]
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

fn to_csv<S: Serializer>(vec: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error> {
	match vec {
		Some(vec) => {
			println!("apple");
			let mut seq = serializer.serialize_seq(Some(vec.len()))?;
			for e in vec {
				seq.serialize_element(e)?;
			}
			seq.end()
		},
		None => serializer.serialize_none()
	}
}

