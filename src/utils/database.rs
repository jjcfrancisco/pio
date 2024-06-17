use crate::Result;

use postgres::{Client, NoTls};

pub fn create_connection(uri: &str) -> Result<Client> {
    let client = Client::connect(uri, NoTls)?;
    Ok(client)
}

pub fn check_health(client: &mut Client) -> Result<()> {
    let rows = client.query("SELECT 1", &[])?;
    for row in rows {
        let one: i32 = row.get(0);
        assert_eq!(one, 1);
    }
    Ok(())
}

pub struct Database {
    pub client: Client,
}

impl Database {
    pub fn new(uri: &str) -> Result<Self> {
        let client = create_connection(uri)?;
        Ok(Database { client })
    }
    pub fn check_health(&mut self) -> Result<()> {
        check_health(&mut self.client)?;
        Ok(())
    }
    pub fn create_table(&mut self) -> Result<()> {
        self.client.batch_execute(
            "CREATE TABLE IF NOT EXISTS pio (
              id BIGINT PRIMARY KEY,
              properties JSONB,
              geometry geometry);",
        )?;
        Ok(())
    }
}

// "CREATE TABLE IF NOT EXISTS pio (
// id BIGINT PRIMARY KEY,
// properties JSONB NOT NULL,
// geometry geometry NOT NULL);"
