use aws_sdk_dynamodb as dynamodb;

#[tokio::main]
async fn main() -> Result<(), dynamodb::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);

    // ... make some calls with the client

    Ok(())
}
