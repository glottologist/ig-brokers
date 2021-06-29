pub struct Config {
	pub base_url: String
}

impl Config {
	pub fn default() -> Config {
		Config {
			base_url: String::from("https://api.ig.com/gateway/deal")
		}
	}

	pub fn test() -> Config {
		Config {
			base_url: String::from("https://demo-api.ig.com/gateway/deal")
		}
	}
}
