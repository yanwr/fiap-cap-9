use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Extension};
use reqwest_tracing::{OtelName, TracingMiddleware};
use std::time::Duration;

pub fn new_client(
    name: &str,
    request_timeout: u64,
) -> Result<ClientWithMiddleware, Box<dyn std::error::Error>> {
    let client = ClientBuilder::new(
        Client::builder()
            .timeout(Duration::from_secs(request_timeout))
            .build()?,
    )
    .with_init(Extension(OtelName(String::from(name).into())))
    .with(TracingMiddleware::default())
    .build();
    Ok(client)
}
