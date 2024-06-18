use crate::Result;

use deadpool_postgres::{Config, ManagerConfig, Object, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub async fn create_pool(uri: &str) -> Result<Pool> {
    let mut cfg = Config::new();
    cfg.url = Some(uri.to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    Ok(pool)
}

// Create table using Client from pool
pub async fn create_table(client: &Object) -> Result<()> {
    client.execute("CREATE TABLE IF NOT EXISTS pio (
                    id BIGINT PRIMARY KEY,
                    properties JSONB,
                    geometry geometry);", &[]).await?;
    Ok(())
}

pub async fn run_job(pool: &Pool) -> Result<()> {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error getting connection from pool: {}", err);
            return Err(err.into());
        }
    };
    create_table(&client).await?;

    for i in 1..10i32 {
        let client = match pool.get().await {
            Ok(client) => client,
            Err(err) => {
                eprintln!("Error getting connection from pool: {}", err);
                continue;
            }
        };

        let stmt = match client.prepare_cached("SELECT 1 + $1").await {
            Ok(stmt) => stmt,
            Err(err) => {
                eprintln!("Error preparing statement: {}", err);
                continue;
            }
        };

        let rows = match client.query(&stmt, &[&i]).await {
            Ok(rows) => rows,
            Err(err) => {
                eprintln!("Error executing query: {}", err);
                continue;
            }
        };

        let value: i32 = rows[0].get(0);
        assert_eq!(value, i + 1);
    }

    Ok(())
}

// "CREATE TABLE IF NOT EXISTS pio (
// id BIGINT PRIMARY KEY,
// properties JSONB NOT NULL,
// geometry geometry NOT NULL);"
