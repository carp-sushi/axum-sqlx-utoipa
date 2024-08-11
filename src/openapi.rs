use sqlx_todos::api::routes::{story, task};
use std::error::Error;
use utoipa::OpenApi;

/// Generate project OpenApi docs.
fn main() -> Result<(), Box<dyn Error>> {
    let mut api = story::ApiDoc::openapi();
    api.merge(task::ApiDoc::openapi());
    println!("{}", api.to_pretty_json()?);
    Ok(())
}
