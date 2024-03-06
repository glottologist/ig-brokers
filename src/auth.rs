use crate::config::Config;
use crate::models::{LoginReq};


use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;



use std::option::Option;

#[derive(Debug)]
pub struct Authentication {
    pub account_id: String,
    pub api_key: String,
    pub username: String,
    pub password: String,
    pub config: Config,
    pub token: Option<HeaderValue>,
    pub client: reqwest::blocking::Client,
}

impl Authentication {
    pub fn new(
        account_id: String,
        api_key: String,
        username: String,
        password: String,
        config: Config,
    ) -> Self {
        let client = reqwest::blocking::Client::new();
        let login_token = Self::login(
            &account_id,
            &api_key,
            &username,
            &password,
            &config,
            &client,
        );
        Self {
            account_id,
            api_key,
            username,
            password,
            config,
            token: login_token,
            client,
        }
    }

    pub fn get_url(config: &Config, endpoint: &String) -> String {
        format!("https://{}{}", config.base_url, endpoint)
    }

    fn login(
        account_id: &str,
        api_key: &str,
        username: &str,
        password: &str,
        config: &Config,
        client: &reqwest::blocking::Client,
    ) -> Option<HeaderValue> {
        let login = LoginReq {
            identifier: username.to_owned(),
            password: password.to_owned(),
        };

        println!("Login  username {}", username);
        println!("Login  password {}", password);
        let mut headers = HeaderMap::new();
        headers.insert("X-IG-API-KEY", api_key.parse().unwrap());
        headers.insert("IG-ACCOUNT-ID", account_id.parse().unwrap());
        headers.insert("VERSION", "2".parse().unwrap());

        let url = Self::get_url(config, &"/session".into());
        println!("Login url {}", url);
        let login_response = client.post(&url).headers(headers).json(&login).send();

        println!("Login response {:?}", login_response);
        match login_response {
            Ok(r) => r.headers().get("X-SECURITY-TOKEN").cloned(),
            Err(_) => None,
        }
    }
}
