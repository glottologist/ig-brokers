use crate::api::Config;
use crate::models::{LoginReq, LoginRes};
use reqwest::Error;
use reqwest::blocking::RequestBuilder;
use reqwest::header::HeaderMap;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct Client {
	account_id: String,
	api_key: String,
	username: String,
	password: String,
	client: reqwest::blocking::Client,
	config: Config
}

fn get_url(config: &Config, endpoint: &String) -> String {
	format!("https://{}{}", config.base_url, endpoint)
}

impl Client {
	pub fn new(account_id: String, api_key: String, username: String, password: String, config: Config) -> Client {
		Client {
			account_id,
			api_key,
			username,
			password,
			config,
			client: reqwest::blocking::Client::new()
		}
	}

	pub fn get_signed<T: DeserializeOwned>(&self, endpoint: &String) -> Result<T, Error> {
		let url = get_url(&self.config, endpoint);
		let req = self.set_headers(self.client.get(url))?;
		let res = req.send()?;
		Ok(res.json::<T>()?)
	}

	pub fn put_signed<T: DeserializeOwned, U: Serialize>(&self, endpoint: &String, data: &Option<U>) -> Result<T, Error> {
		let url = get_url(&self.config, endpoint);
		let mut req = self.set_headers(self.client.put(url))?;

		if let Some(body) = data {
			req = req.json(&body);
		}

		let res = req.send()?;
		Ok(res.json::<T>()?)
	}

	fn get_token(&self) -> Result<LoginRes, Error> {
		let login = LoginReq {
			identifier: self.username.clone(),
			password: self.password.clone()
		};

		let mut headers = HeaderMap::new();
		headers.insert("X-IG-API-KEY", self.api_key.parse().unwrap());
		headers.insert("IG-ACCOUNT-ID", self.account_id.parse().unwrap());
		headers.insert("VERSION", "3".parse().unwrap());

		let url = get_url(&self.config, &"/session".into());
		let res = self.client.post(&url).headers(headers).json(&login).send()?;
		res.json::<LoginRes>()
	}

	fn set_headers(&self, req: RequestBuilder) -> Result<RequestBuilder, Error> {
		let token = self.get_token()?;
		let authorization = format!("Bearer {}", token.oauth_token.access_token);

		let mut headers = HeaderMap::new();
		headers.insert("IG-ACCOUNT-ID", self.account_id.parse().unwrap());
		headers.insert("X-IG-API-KEY", self.api_key.parse().unwrap());
		headers.insert("Authorization", authorization.parse().unwrap());
		headers.insert("VERSION", "1".parse().unwrap());
		Ok(req.headers(headers))
	}

	// pub fn get_with_params_signed<T>(&self, endpoint: &String, params: &HashMap<String, String>) -> T {
	// 	let res = reqwest::post
	// }

	// pub fn post_signed<T, U>(&self, endpoint: &String, req: &T) -> U {

	// }

	// pub fn delete_signed<T, U>(&self, endpoint: &String, req: &Option<T>) -> U {

	// }
}
