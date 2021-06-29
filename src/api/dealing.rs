pub struct Dealing;

impl Dealing {
	/// GET /confirms/{dealReference}
	pub fn get_deal_confirmation(&self) {

	}

	/// GET /positions
	pub fn get_positions(&self) {

	}

	/// GET /positions/{dealId}
	pub fn get_positions_with_deal(&self, deal: &String) {

	}

	/// DELETE /positions/otc
	pub fn close_position(&self) {

	}

	/// POST /positions/otc
	pub fn create_position(&self) {

	}

	/// PUT /positions/otc/{dealId}
	pub fn update_position(&self, deal: &String) {

	}

	/// GET /positions/sprintmarkets
	pub fn get_sprint_market_positions(&self) {

	}

	/// POST /positions/sprintmarkets
	pub fn create_sprint_market_position(&self) {

	}

	/// GET /workingorders
	pub fn get_working_orders(&self) {

	}

	/// POST /workingorders/otc
	pub fn create_working_order(&self) {

	}

	/// DELETE /workingorders/otc/{dealId}
	pub fn delete_working_order(&self, deal: &String) {

	}

	/// PUT /workingorders/otc/{dealId}
	pub fn update_working_order(&self, deal: &String) {

	}
}
