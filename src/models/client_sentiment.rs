use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Default)]
pub struct SentimentQuery {
    pub market_ids: Option<Vec<String>>,
}

impl Serialize for SentimentQuery {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("SentimentQuery", 1)?;

        match self.market_ids.as_ref() {
            Some(ids) => {
                state.serialize_field("marketIds", &ids.join(","))?;
            }
            None => {
                state.serialize_field("marketIds", &None::<()>)?;
            }
        }

        state.end()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sentiments {
    pub client_sentiments: Vec<Sentiment>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sentiment {
    pub long_position_percentage: f64,
    pub market_id: String,
    pub short_position_percentage: f64,
}
