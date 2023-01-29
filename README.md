# krate 📦
<img src="https://user-images.githubusercontent.com/3408176/215298824-61657e9b-5cd3-401e-b7c6-3a31d9876b67.png" width="250"/>

Asynchonously get information and metadata for a Rust Crate published on Crates.io!

`krate` additionally comes with `struct Krate` which contains a partially implemented data model for the &*Crates.io* API/V1 Contract. 

**NOTE:** Currently there is no _publically_ docuemented API contract for the *Crates.io* API/V1 Contract so any changes or `null` values passed via the API could break serialization. 🤷‍♂️

Please see the [crawler policy on Crates.io](https://crates.io/policies#crawlers) if you are planning to use this library to crawl or access the crates data.

## Usage 
`$> cargo add krate`

_main.rs_
```rust

#[tokio::main]
async fn main() {
    // Use Krate::get_async to get information on a particular Krate!
    match krate::get_async("serde", "My User Agent Tool).await {
        Ok(serde_crate) => {
            println!("Krate: {}", serde_crate.krate.name);
            println!("Latest Version: {}", serde_crate.get_latest());
            println!("Description: {}", serde_crate.krate.description );
        },
        Err(e) => println!("Error: {e}"),
    }    
}
```

This is a very small implementation! For more robust client see @TheDuke's [crates-io-api](https://github.com/theduke/crates-io-api) 


