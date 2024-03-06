use crate::auth::Authentication;
use crate::config::Config;
use crate::models::{LoginReq, LoginRes};
use reqwest::blocking::RequestBuilder;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Client {
    auth: Authentication,
}

impl Client {
    pub fn new(auth: Authentication) -> Client {
        Client { auth }
    }

    pub fn get_signed<T: DeserializeOwned, U: Serialize>(
        &self,
        endpoint: &String,
        version: u8,
        query: Option<U>,
    ) -> Result<T, Error> {
        let url = Authentication::get_url(&self.auth.config, endpoint);
        let mut req = self.set_headers(self.auth.client.get(url), version)?;

        if let Some(query) = query {
            req = req.query(&query);
        }

        println!("Request: {:?}", req);
        let res = req.send()?;
        println!("Response: {:?}", res);
        Ok(res.json::<T>()?)
    }

    pub fn post_signed<T: DeserializeOwned, U: Serialize>(
        &self,
        endpoint: &String,
        version: u8,
        data: Option<U>,
    ) -> Result<T, Error> {
        let url = Authentication::get_url(&self.auth.config, endpoint);
        let mut req = self.set_headers(self.auth.client.post(url), version)?;

        if let Some(data) = data {
            req = req.json(&data);
        }
        println!("Request: {:?}", req);
        let res = req.send()?;
        println!("Response: {:?}", res);
        Ok(res.json::<T>()?)
    }

    pub fn put_signed<T: DeserializeOwned, U: Serialize>(
        &self,
        endpoint: &String,
        version: u8,
        data: Option<U>,
    ) -> Result<T, Error> {
        let url = Authentication::get_url(&self.auth.config, endpoint);
        let mut req = self.set_headers(self.auth.client.put(url), version)?;

        if let Some(data) = data {
            req = req.json(&data);
        }

        println!("Request: {:?}", req);
        let res = req.send()?;
        println!("Response: {:?}", res);
        Ok(res.json::<T>()?)
    }

    pub fn delete_signed<T: DeserializeOwned, U: Serialize>(
        &self,
        endpoint: &String,
        version: u8,
        data: Option<U>,
    ) -> Result<T, Error> {
        let url = Authentication::get_url(&self.auth.config, endpoint);
        let mut req = self.set_headers(self.auth.client.post(url), version)?;

        let mut headers = HeaderMap::new();
        headers.insert("_method", "DELETE".to_string().parse().unwrap());
        req = req.headers(headers);

        if let Some(data) = data {
            req = req.json(&data);
        }

        println!("Request: {:?}", req);

        let res = req.send()?;
        println!("Response: {:?}", res);
        Ok(res.json::<T>()?)
    }

    fn set_headers(&self, req: RequestBuilder, version: u8) -> Result<RequestBuilder, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("IG-ACCOUNT-ID", self.auth.account_id.parse().unwrap());
        headers.insert("X-IG-API-KEY", self.auth.api_key.parse().unwrap());
        if let Some(th) = &self.auth.token {
            headers.insert("X-SECURITY-TOKEN", th.clone());
        }
        headers.insert("VERSION", version.to_string().parse().unwrap());
        println!("Headers {:?}", headers);
        Ok(req.headers(headers))
    }
}
