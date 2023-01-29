
#[tokio::main]
async fn main() {
    // Use Krate::get_async to get information on a particular Krate!
    match krate::get_async("serde").await {
        Ok(serde_crate) => {
            println!("Krate: {}", serde_crate.krate.name);
            println!("Latest Version: {}", serde_crate.get_latest());
            println!("Description: {}", serde_crate.krate.description );
        },
        Err(e) => println!("Error: {}", e),
    }    
}