use crate::Result;

pub mod pool;
pub mod async_job;

use postgres::{Client, NoTls, CopyInWriter};

pub fn create_connection(uri: &str) -> Result<Client> {
    let client = Client::connect(uri, NoTls)?;
    Ok(client)
}

pub fn create_table(client: &mut Client) -> Result<()> {
    client.execute("CREATE TABLE IF NOT EXISTS pio (
                    id INT,
                    properties JSONB,
                    geometry geometry);", &[])?;
    Ok(())
}

pub fn create_binary_writer<'a>(client: &'a mut Client, sql: &str) -> Result<CopyInWriter<'a>> {
    let sink:CopyInWriter = client.copy_in(sql)?;
    Ok(sink)
}
