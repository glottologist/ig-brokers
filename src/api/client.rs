use crate::api::Config;
use std::collections::HashMap;

pub struct Client {
	api_key: Option<String>,
	api_secret: Option<String>,
	config: Config
}

impl Client {
	pub fn new(api_key: Option<String>, api_secret: Option<String>, config: Config) -> Client {
		Client { api_key, api_secret, config }
	}

	pub fn get_signed<T>(&self, endpoint: &String) -> T {

	}

	pub fn get_with_params_signed<T>(&self, endpoint: &String, params: &HashMap<String, String>) -> T {

	}

	pub fn post_signed<T, U>(&self, endpoint: &String, req: &T) -> U {

	}

	pub fn put_signed<T, U>(&self, endpoint: &String, req: &T) -> U {

	}

	pub fn delete_signed<T, U>(&self, endpoint: &String, req: &Option<T>) -> U {

	}
}
