// use crate::api::{Client, Config, IG};
// use crate::models::{GetSession, LoginReq, LoginRes, LoginTradingRes, OauthToken, RefreshTokenReq, Session, SwitchAccountReq, SwitchAccountRes};
// use std::collections::HashMap;

// pub struct Login {
// 	client: Client
// }

// impl Login {
// 	/// DELETE /session
// 	/// Log out of the current session.
// 	pub fn logout(&self) {
// 		let endpoint = String::from("/session");
// 		self.client.delete_signed(&endpoint, &None::<()>)
// 	}

// 	/// GET /session
// 	/// Returns the user's session details and optionally tokens.
// 	pub fn get_session(&self, req: &GetSession) -> Session {
// 		let params: HashMap<String, String> = HashMap::new();
// 		params.insert(String::from("fetchSessionTokens"), req.fetch_session_tokens.to_string());

// 		let endpoint = String::from("/session");
// 		self.client.get_with_params_signed(&endpoint, &params)
// 	}

// 	/// POST /session
// 	/// Creates a trading session, obtaining session tokens for subsequent API access.
// 	/// Please note that region-specific login restrictions may apply.
// 	pub fn login(&self, req: &LoginReq) -> LoginRes {
// 		let endpoint = String::from("/session");
// 		self.client.post_signed(&endpoint, &req)
// 	}

// 	/// PUT /session
// 	/// Switches active accounts, optionally setting the default account.
// 	pub fn switch_account(&self, req: &SwitchAccountReq) -> SwitchAccountRes {
// 		let endpoint = String::from("/session");
// 		self.client.put_signed(&endpoint, &Some(req))
// 	}

// 	/// GET /session/encryptionKey
// 	/// Creates a trading session, obtaining session tokens for subsequent API access.
// 	pub fn login_trading(&self) -> LoginTradingRes {
// 		let endpoint = String::from("/session/encryptionKey");
// 		self.client.get_signed(&endpoint)
// 	}

// 	/// POST /session/refresh-token
// 	pub fn refresh_trading(&self, req: RefreshTokenReq) -> OauthToken {
// 		let endpoint = String::from("/session/refresh-token");
// 		self.client.post_signed(&endpoint, &req)
// 	}
// }

// impl IG for Login {
// 	fn new(api_key: Option<String>, api_secret: Option<String>) -> Login {
// 		Self::new_with_config(api_key, api_secret, Config::default())
// 	}

// 	fn new_with_config(api_key: Option<String>, api_secret: Option<String>, config: Config) -> Login {
// 		let client = Client::new(api_key, api_secret, config);
// 		Login { client }
// 	}
// }

