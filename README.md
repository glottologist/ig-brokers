# ig-brokers
Unofficial Rust library for the [IG REST API](https://labs.ig.com/rest-trading-api-reference)

## Disclaimer
This is an unofficial implementation, and early in its development.
Everything hasn't been fully tested yet, so please use at your own risk.
Given the following warning, any financial losses incurred from using this will be your own responsibility.

## Usage
Cargo.toml
```toml
[dependencies]
ig-brokers = { git = "https://github.com/satnav/ig-brokers.git" }
```

.env
```
IG_ACCOUNT_ID=<Your IG Account Id>
IG_API_KEY=<Your IG API KEY>
IG_USERNAME=<Your IG Username>
IG_PASSWORD=<Your IG Password>
```

Example
```rust
use dotenv::dotenv;
use ig_brokers::api::IG;
use ig_brokers::models::*;
use std::env;

fn main() {
	// Load .env file
	dotenv().unwrap();

	// Extract environment variables
	let account_id = env::var("IG_ACCOUNT_ID").unwrap();
	let api_key = env::var("IG_API_KEY").unwrap();
	let username = env::var("IG_USERNAME").unwrap();
	let password = env::var("IG_PASSWORD").unwrap();

	// Switch to IG::live for live account
	let api = IG::demo(account_id, api_key, username, password);

	// Call endpoint, for reference use the IG REST API documentation
	// https://labs.ig.com/rest-trading-api-reference
	match api.get_accounts() {
		Ok(res) => println!("{:?}", res),
		Err(e) => println!("{:?}", e)
	};
}
```
