use crate::api::{Client, Config, IG};
use crate::models::{Application, UpdateApplication};

pub struct General {
	client: Client
}

impl General {
	/// GET /operations/application
	/// Returns a list of client owned applications.
	pub fn get_applications(&self) -> Application {
		let endpoint = String::from("/operations/application");
		self.client.get_signed(&endpoint)
	}

	/// PUT /operations/application
	/// Alters the details of a given user application.
	pub fn update_application(&self, req: &UpdateApplication) -> Application {
		let endpoint = String::from("/operations/application");
		self.client.put_signed(&endpoint, &Some(req))
	}

	/// PUT /operations/application/disable
	/// Disables the current application key from processing further requests.
	/// Disabled keys may be reenabled via the My Account section on our web dealing platform.
	pub fn disable_application(&self) -> Application {
		let endpoint = String::from("/operations/application/disable");
		self.client.put_signed(&endpoint, &None::<()>)
	}
}

impl IG for General {
	fn new(api_key: Option<String>, api_secret: Option<String>) -> General {
		Self::new_with_config(api_key, api_secret, Config::default())
	}

	fn new_with_config(api_key: Option<String>, api_secret: Option<String>, config: Config) -> General {
		let client = Client::new(api_key, api_secret, config);
		General { client }
	}
}
