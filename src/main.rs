use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};

const DEFAULT_REGION : &str = "ap-southeast-2";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else(DEFAULT_REGION);
    let region = region_provider.region().await;
    println!("region = {:#?}", region);
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let resp = client.list_tables().send().await?;
    let names = resp.table_names().unwrap_or_default();

    println!("Tables {{");
    for name in names {
        println!("  {}", name);
    }
    println!("}}");

    println!("Found {} tables", names.len());

    Ok(())
}
