use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
	pub account_id: String,
	pub client_id: String,
	pub currency: String,
	pub lightstreamer_endpoint: String,
	pub locale: String,
	pub timezone_offset: f64
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginReq {
	pub identifier: String,
	pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRes {
	pub account_id: String,
	pub client_id: String,
	pub lightstreamer_endpoint: String,
	pub oauth_token: OauthToken,
	pub timezone_offset: f64
}

#[derive(Debug, Deserialize)]
pub struct OauthToken {
	pub access_token: String,
	pub expires_in: String,
	pub refresh_token: String,
	pub scope: String,
	pub token_type: String
}
