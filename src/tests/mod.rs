use crate::client::CraftgateClient;

mod onboarding;

#[cfg(test)]
fn get_test_client() -> CraftgateClient {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY");
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY");
    CraftgateClient::new(true, &api_key, &secret_key)
}
