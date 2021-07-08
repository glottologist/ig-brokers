pub mod api;
pub mod models;

#[cfg(test)]
mod tests {
    use crate::api::{Account, IG, Config};
    use crate::models::*;
    use chrono::Utc;
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

    fn get_api<T : IG>() -> T {
        let (account_id, api_key, username, password) = setup();
        IG::new_with_config(account_id, api_key, username, password, Config::test())
    }

    #[test]
    fn get_accounts() {
        let account: Account = get_api();
        let res = account.get_accounts();
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_preferences() {
        let account: Account = get_api();
        let res = account.get_preferences();
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn update_preferences() {
        let account: Account = get_api();
        let pref = Preferences { trailing_stops_enabled: true };
        let res = account.update_preferences(&pref);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_activity_history_1() {
        let account: Account = get_api();
        let mut params = ActivityHistoryQuery::default();
        params.from = Utc::now().naive_local().into();
        let res = account.get_activity_history(&params);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_activity_history_2() {
        let account: Account = get_api();
        let mut query = ActivityHistoryQuery::default();
        query.from = (Utc::now() - chrono::Duration::days(365)).naive_local().into();
        let res = account.get_activity_history(&query);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_transaction_history_1() {
        let account: Account = get_api();
        let mut query = TransactionHistoryQuery::default();
        query.from = Utc::now().naive_local().into();
        let res = account.get_transaction_history(&query);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_transaction_history_2() {
        let account: Account = get_api();
        let mut query = TransactionHistoryQuery::default();
        query.from = (Utc::now() - chrono::Duration::days(365)).naive_local().into();
        let res = account.get_transaction_history(&query);
        assert_eq!(res.is_ok(), true);
    }
}
