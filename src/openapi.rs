use sqlx_todos::api;
use std::error::Error;

/// Generate project OpenApi docs.
fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", api::docs().to_pretty_json()?);
    Ok(())
}
