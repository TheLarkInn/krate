fn main() {
    let client = krate::KrateClientBuilder::new("My Custom Tool User Agent - thelarkinn/krate")
        .build_sync()
        .unwrap();

    match client.get("syn") {
        Ok(syn_crate) => {
            println!("Krate: {}", syn_crate.krate.name);
            println!("Latest Version: {}", syn_crate.get_latest());
            println!("Description: {}", syn_crate.krate.description);

            println!(
                "Here are the features for version 1.0.107: {:?}",
                syn_crate.get_features_for_version("1.0.107").unwrap()
            )
        }
        Err(e) => println!("Error: {e}"),
    }
}
