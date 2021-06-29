use crate::api::Config;

pub trait IG {
	fn new(api_key: Option<String>, api_secret: Option<String>) -> Self;
	fn new_with_config(api_key: Option<String>, api_secret: Option<String>, config: Config) -> Self;
}
