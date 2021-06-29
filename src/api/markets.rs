use chrono::NaiveDate;

pub struct Markets;

impl Markets {
	/// GET /marketnavigation
	pub fn get_market_categories(&self) {

	}

	/// GET /marketnavigation/{nodeId}
	pub fn get_market_category(&self, node: &String) {

	}

	/// GET /markets
	pub fn get_markets(&self) {

	}

	/// GET /markets/{epic}
	pub fn get_market(&self, epic: &String) {

	}

	/// GET /markets?searchTerm={searchTerm}
	pub fn find_markets(&self, search: &String) {

	}

	/// GET /prices/{epic}
	pub fn get_prices(&self, epic: &String) {

	}

	/// GET /prices/{epic}/{resolution}/{numPoints}
	pub fn get_prices_with_points(&self, epic: &String, resolution: &String, points: u32) {

	}

	/// GET /prices/{epic}/{resolution}/{startDate}/{endDate}
	pub fn get_prices_with_dates(&self, epic: &String, resolution: &String, start: &NaiveDate, end: &NaiveDate) {

	}
}
