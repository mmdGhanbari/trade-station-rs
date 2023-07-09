use dto::{AccessTokenResponse, GetAccessTokenParams};
use reqwest::{Client, StatusCode};

mod dto;

#[tokio::main]
async fn main() {
    // Make sure to replace these with actual values.
    // We can fetch these secrets from the process env to improve safety. But, here, we keep it simple.
    let client_id = "your_client_id".to_string();
    let client_secret = "your_client_secret".to_string();
    let refresh_token = "your_refresh_token".to_string();

    let params = GetAccessTokenParams::new(client_id, client_secret, refresh_token);

    println!("Fetching the access-token...");

    match get_access_token(params).await {
        Ok(token) => println!("{:#?}", token),
        Err(err) => println!("Error: {}", err),
    }
}

async fn get_access_token(params: GetAccessTokenParams) -> Result<AccessTokenResponse, String> {
    // Initialize reqwest http-client
    let http_client = Client::new();

    // Send the POST request
    let response = http_client
        .post("https://signin.tradestation.com/oauth/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .map_err(|err| format!("Sending GetAccessToken req failed!\nError: {}", err))?;

    match response.status() {
        // If the response status is OK, parse it into AccessTokenResponse
        StatusCode::OK => {
            let token = response
                .json::<AccessTokenResponse>()
                .await
                .map_err(|err| {
                    format!(
                        "AccessTokenResponse deserialization failed!\nError: {}",
                        err
                    )
                })?;
            Ok(token)
        }
        // Otherwise, return the error message
        _ => {
            let err_message = response
                .text()
                .await
                .unwrap_or("unknown error!".to_string());
            Err(err_message)
        }
    }
}
