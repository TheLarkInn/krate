use anyhow::Result;

fn main() -> Result<()> {
    let user_agent = "get_multi() example from thelarkinn/krate";
    let crates = krate::get_multi(
        vec!["is-wsl", "is-docker", "is-interactive", "krate"],
        user_agent,
    )?;

    println!("Hi my name is Sean Larkin, and here are some of my Rust crates:\n");

    for krate in crates {
        println!("📦 Name: {}", krate.krate.name);
        println!("🦀 {}", krate.krate.description);
        println!("🎉 Latest Version: {}\n", krate.get_latest());
    }

    Ok(())
}
