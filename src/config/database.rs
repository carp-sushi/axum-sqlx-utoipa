use crate::config::Config;
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use std::sync::Arc;

impl Config {
    pub fn db_pool_opts(&self) -> PgPoolOptions {
        let schema = Arc::new(self.db_schema.clone());
        PgPoolOptions::new()
            .max_connections(self.db_max_connections)
            .after_connect(move |conn, _meta| {
                let schema = Arc::clone(&schema);
                Box::pin(async move {
                    conn.execute(format!("SET search_path = '{}';", schema).as_ref())
                        .await?;
                    Ok(())
                })
            })
    }
}
