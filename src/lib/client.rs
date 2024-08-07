
// Import the `reqwest` Client for making HTTP requests
use reqwest::Client;
// Import the `Duration` type from the `tokio` library for setting timeouts
use tokio::time::Duration;

// Function to create and configure an HTTP client
pub fn create_client() -> Client {
    // Build the `reqwest` client with a timeout of 10 seconds
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        // If client creation fails, panic with the provided error message
        .expect("Failed to create client")
}
