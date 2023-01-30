#[tokio::main]
async fn main() {
    // Build a client and provide your custom user-agent string!
    let client = krate::KrateClientBuilder::new("My Custom Tool User Agent - thelarkinn/krate")
        .build_asnyc()
        .unwrap();

    match client.get_async("serde").await {
        Ok(serde_crate) => {
            println!("Krate: {}", serde_crate.krate.name);
            println!("Latest Version: {}", serde_crate.get_latest());
            println!("Description: {}", serde_crate.krate.description);
        }
        Err(e) => println!("Error: {e}"),
    }
}
