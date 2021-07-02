pub struct GetSession {
	pub fetch_session_tokens: bool
}

pub struct Session {
	pub account_id: String,
	pub client_id: String,
	pub currency: String,
	pub lightstreamer_endpoint: String,
	pub locale: String,
	pub timezone_offset: f64
}

pub struct LoginReq {
	authentication_request: AuthenticationReq,
}

pub struct AuthenticationReq {
	pub identifier: String,
	pub password: String,
}

pub struct LoginRes {
	pub account_id: String,
	pub client_id: String,
	pub lightstreamer_endpoint: String,
	pub oauth_token: OauthToken,
	pub timezone_offset: f64
}

pub struct OauthToken {
	pub access_token: String,
	pub expires_in: String,
	pub refresh_token: String,
	pub scope: String,
	pub token_type: String
}

pub struct SwitchAccountReq {
	pub account_switch_request: SwitchAccountReqInner
}

pub struct SwitchAccountReqInner {
	pub account_id: String,
	pub default_account: bool
}

pub struct SwitchAccountRes {
	pub dealing_enabled: bool,
	pub has_active_demo_accounts: bool,
	pub has_active_live_accounts: bool,
	pub trailing_stops_enabled: bool
}

pub struct LoginTradingRes {
	pub encryption_key: String,
	pub time_stamp: u64
}

pub struct RefreshTokenReq {
	pub request: RefreshTokenBody
}

pub struct RefreshTokenBody {
	pub refresh_token: String
}