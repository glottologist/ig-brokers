use crate::api::Config;

pub trait IG {
	fn new(account_id: String, api_key: String, username: String, password: String) -> Self;
	fn new_with_config(account_id: String, api_key: String, username: String, password: String, config: Config) -> Self;
}
