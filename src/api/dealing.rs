use crate::api::{Client, Config, IG};
use crate::models::{
	ClosePosition,
	CreatePosition,
	CreateSprintMarketPosition,
	CreateWorkingOrder,
	DealConfirmation,
	DealRef,
	Position,
	Positions,
	SprintMarketPositions,
	UpdatePosition,
	UpdateWorkingOrder,
	WorkingOrders
};

pub struct Dealing {
	client: Client
}

impl Dealing {
	/// GET /confirms/{dealReference}
	/// Returns a deal confirmation for the given deal reference.
	pub fn get_deal_confirmation(&self, deal_reference: &String) -> DealConfirmation {
		let endpoint = format!("/confirms/{}", deal_reference);
		self.client.get_signed(&endpoint)
	}

	/// GET /positions
	/// Returns all open positions for the active account.
	pub fn get_positions(&self) -> Positions {
		let endpoint = String::from("/positions");
		self.client.get_signed(&endpoint)
	}

	/// GET /positions/{dealId}
	/// Returns an open position for the active account by deal identifier.
	pub fn get_positions_with_deal(&self, deal_id: &String) -> Position {
		let endpoint = format!("/positions/{}", deal_id);
		self.client.get_signed(&endpoint)
	}

	/// DELETE /positions/otc
	/// Closes one or more OTC positions.
	pub fn close_position(&self, req: &ClosePosition) -> DealRef {
		let endpoint = String::from("/positions/otc");
		self.client.delete_signed(&endpoint, &Some(req))
	}

	/// POST /positions/otc
	/// Creates an OTC position.
	pub fn create_position(&self, req: &CreatePosition) -> DealRef {
		let endpoint = String::from("/positions/otc");
		self.client.post_signed(&endpoint, req)
	}

	/// PUT /positions/otc/{dealId}
	/// Updates an OTC position.
	pub fn update_position(&self, deal_id: &String, req: &UpdatePosition) {
		let endpoint = format!("/positions/otc/{}", deal_id);
		self.client.put_signed(&endpoint, &Some(req))
	}

	/// GET /positions/sprintmarkets
	/// A list of sprint market positions.
	pub fn get_sprint_market_positions(&self) -> SprintMarketPositions {
		let endpoint = String::from("/positions/sprintmarkets");
		self.client.get_signed(&endpoint)
	}

	/// POST /positions/sprintmarkets
	/// Creates a sprint market position.
	pub fn create_sprint_market_position(&self, req: &CreateSprintMarketPosition) -> DealRef {
		let endpoint = String::from("/positions/sprintmarkets");
		self.client.post_signed(&endpoint, &req)
	}

	/// GET /workingorders
	/// Returns all open working orders for the active account.
	pub fn get_working_orders(&self) -> WorkingOrders {
		let endpoint = String::from("/workingorders");
		self.client.get_signed(&endpoint)
	}

	/// POST /workingorders/otc
	/// Creates an OTC working order.
	pub fn create_working_order(&self, req: &CreateWorkingOrder) -> DealRef {
		let endpoint = String::from("/workingorders/otc");
		self.client.post_signed(&endpoint, req)
	}

	/// DELETE /workingorders/otc/{dealId}
	/// Deletes an OTC working order.
	pub fn delete_working_order(&self, deal_id: &String) -> DealRef {
		let endpoint = format!("/workingorders/otc/{}", deal_id);
		self.client.delete_signed(&endpoint, &None::<()>)
	}

	/// PUT /workingorders/otc/{dealId}
	/// Updates an OTC working order.
	pub fn update_working_order(&self, deal_id: &String, req: &UpdateWorkingOrder) -> DealRef {
		let endpoint = format!("/workingorders/otc/{}", deal_id);
		self.client.put_signed(&endpoint, &Some(req))
	}
}

impl IG for Dealing {
	fn new(api_key: Option<String>, api_secret: Option<String>) -> Dealing {
		Self::new_with_config(api_key, api_secret, Config::default())
	}

	fn new_with_config(api_key: Option<String>, api_secret: Option<String>, config: Config) -> Dealing {
		let client = Client::new(api_key, api_secret, config);
		Dealing { client }
	}
}
