use std::{
    fs::File,
    io::{Error, Write},
};

use reqwest::{Client, StatusCode};

#[tokio::main]
async fn main() -> Result<(), String> {
    // Initialize reqwest http-client
    let http_client = Client::new();

    // Send the Get request
    // Here I'm using a fake rest api to test the code. Change the request to match your requirements.
    let response = http_client
        .get("https://jsonplaceholder.typicode.com/todos")
        .send()
        .await
        .map_err(|err| format!("Sending the request failed! Error: {}", err))?;

    match response.status() {
        // If the response status is OK, write the json to the output file
        StatusCode::OK => {
            // Here is how you can extract the response headers.
            let headers = response.headers();
            // Imagine we want to get the content-type header:
            let content_type = headers
                .get("content-type")
                .ok_or(format!("This header does not exist!"))?
                .to_str()
                .map_err(|_err| format!("Failed to convert the header value to str"))?;
            println!("Content-Type: {}", content_type);

            // Now, we will write the fetched response to the data/response.json file.
            let json = response.text().await.unwrap();
            write_to_file("data/response.json", &json)
                .map_err(|err| format!("Could not write to file. Err: {}", err))?;

            Ok(())
        }
        // Otherwise, return the error message
        _ => response
            .text()
            .await
            .map(|_| ())
            .map_err(|_err| "Unknown error!".to_string()),
    }
}

fn write_to_file(path: &str, content: &str) -> Result<(), Error> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}
