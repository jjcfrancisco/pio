use crate::Result;
use bytes::Bytes;
use tokio_postgres::{Client, CopyInSink, NoTls};

pub async fn create_connection(uri: &str) -> Result<Client> {
    let (client, connection) = tokio_postgres::connect(uri, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

pub async fn create_table(client: &Client) -> Result<()> {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS pio (
                    id BIGINT PRIMARY KEY,
                    properties JSONB,
                    geometry geometry);",
            &[],
        )
        .await?;
    Ok(())
}

pub async fn create_sink(client: &Client, sql: &str) -> Result<CopyInSink<Bytes>> {
    let sink: CopyInSink<Bytes> = client.copy_in(sql).await?;
    Ok(sink)
}

