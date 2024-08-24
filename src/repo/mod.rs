use crate::Error;
use sqlx::postgres::PgPool;
use std::sync::Arc;

mod file;
mod sql;
mod story;
mod task;

/// Concrete database logic
pub struct Repo {
    db: Arc<PgPool>,
}

impl Repo {
    /// Constructor
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    /// Get a ref to the connection pool.
    fn db_ref(&self) -> &PgPool {
        self.db.as_ref()
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Internal {
            message: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{
        migrate::Migrator,
        postgres::{PgPool, PgPoolOptions},
    };
    use std::{path::Path, sync::Arc};

    use testcontainers::ContainerAsync as Container;
    use testcontainers_modules::postgres::Postgres;

    /// Given a running Postgres container, set up a connection pool and run migrations.
    pub async fn setup_pg_pool(container: &Container<Postgres>) -> Arc<PgPool> {
        let connection_string = &format!(
            "postgres://postgres:postgres@localhost:{}/postgres",
            container.get_host_port_ipv4(5432).await.unwrap(),
        );

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .min_connections(1)
            .connect(&connection_string)
            .await
            .unwrap();

        println!("Running migrations on test container: {connection_string}");
        let m = Migrator::new(Path::new("./migrations")).await.unwrap();
        m.run(&pool).await.unwrap();

        Arc::new(pool)
    }
}
