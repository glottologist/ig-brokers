pub mod api;
pub mod models;

#[cfg(test)]
mod tests {
    use crate::api::{Account, IG, Config};
    use dotenv::dotenv;
    use std::env;

    fn setup() -> (String, String, String, String) {
        dotenv().unwrap();
        let account_id = env::var("IG_ACCOUNT_ID").unwrap();
        let api_key = env::var("IG_API_KEY").unwrap();
        let username = env::var("IG_USERNAME").unwrap();
        let password = env::var("IG_PASSWORD").unwrap();
        (account_id, api_key, username, password)
    }

    #[test]
    fn get_accounts() {
        let (account_id, api_key, username, password) = setup();
        let account: Account = IG::new_with_config(account_id, api_key, username, password, Config::test());
        let res = account.get_accounts();

        match res.as_ref() {
            Err(e) => {
                println!("{}", e);
            },
            _ => ()
        }

        assert_eq!(res.is_ok(), true);
    }
}
