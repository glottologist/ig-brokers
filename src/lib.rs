pub mod api;
pub mod models;

#[cfg(test)]
mod tests {
    use crate::api::{Account, IG, Config, ClientSentiment, Dealing, Market};
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

    #[test]
    fn get_client_sentiments() {
        let sentiment: ClientSentiment = get_api();
        let mut query = SentimentQuery::default();
        query.market_ids = Some(vec!["VOD-UK".to_string()]);
        let res = sentiment.get_client_sentiments(&query);

        if let Err(e) = res.as_ref() {
            println!("{}", e);
        }

        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_client_sentiment() {
        let sentiment: ClientSentiment = get_api();
        let res = sentiment.get_client_sentiment(&"VOD-UK".into());
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_related_client_sentiment() {
        let sentiment: ClientSentiment = get_api();
        let res = sentiment.get_related_client_sentiment(&"VOD-UK".into());
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_deal_confirmation() {
        let dealing: Dealing = get_api();

        // Create position
        let mut position = CreatePosition::default();
        position.direction = Direction::Buy.into();
        position.epic = "CS.D.BITCOIN.CFD.IP".to_string().into();
        position.expiry = "-".to_string().into();
        position.force_open = false.into();
        position.guaranteed_stop = false.into();
        position.order_type = OrderType::Market.into();
        position.size = 1.0.into();

        // Get deal confirmation

        // Close deal to clean up
    }

    #[test]
    fn get_market_categories() {
        let market: Market = get_api();
        let res = market.get_market_categories();
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn get_market_category() {
        let market: Market = get_api();
        let res = market.get_market_categories().expect("market call failed");
        let nodes = res.nodes.expect("nodes was null");
        let crypto = nodes.iter().find(|n| n.name == "Cryptocurrency".to_string());
        let crypto = crypto.expect("crypto not found");

        let res = market.get_market_category(&crypto.id);
        assert_eq!(res.is_ok(), true);
    }
}
