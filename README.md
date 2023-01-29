# krate 📦
Get information and metadata for a Rust Crate published on Crates.io!

`krate` additionally comes with `struct Krate` which contains a partially implemented data model for the &*Crates.io* API/V1 Contract. 

**NOTE:** Currently there is no _publically_ docuemented API contract for the *Crates.io* API/V1 Contract so any changes or `null` values passed via the API could break serialization. 🤷‍♂️

## Usage 
`$> cargo add krate`

_main.rs_
```rust

#[tokio::main]
async fn main() {
    // Use Krate::get_async to get information on a particular Krate!
    match krate::get_async("serde").await {
        Ok(serde_crate) => {
            println!("Krate: {}", serde_crate.krate.name);
            println!("Latest Version: {}", serde_crate.get_latest());
            println!("Description: {}", serde_crate.krate.description );
        },
        Err(e) => println!("Error: {e}"),
    }    
}
```

