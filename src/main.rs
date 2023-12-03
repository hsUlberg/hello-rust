use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // The URL of the API endpoint
    let url = "https://ssr.finanstilsynet.no/api/v2/instruments";

    // Make a GET request
    let response = reqwest::get(url).await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the response text
        let response_text = response.text().await?;
        println!("Response Text: {}", response_text);
    } else {
        // Handle the error
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}
