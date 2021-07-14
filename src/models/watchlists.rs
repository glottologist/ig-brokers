use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watchlists {
	pub watchlists: Vec<Watchlist>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watchlist {
	pub default_system_watchlist: bool,
	pub deleteable: bool,
	pub editable: bool,
	pub id: String,
	pub name: String
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWatchlist {
	pub epics: Vec<String>,
	pub name: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWatchlistResult {
	pub status: CreateWatchlistStatus,
	pub watchlist_id: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CreateWatchlistStatus {
	Success,
	SuccesNotAllInstrumentsAdded
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToWatchlist {
	pub epic: String
}
