use bytes::BytesMut;
use postgres::types::to_sql_checked;
use postgres::types::{IsNull, ToSql, Type};
use std::error::Error;

#[derive(Debug)]
pub struct Wkb {
    pub geometry: Vec<u8>,
}

impl ToSql for Wkb {
    fn to_sql(
        &self,
        _: &Type,
        out: &mut BytesMut,
    ) -> std::result::Result<IsNull, Box<dyn Error + Send + Sync>> {
        out.extend_from_slice(&self.geometry);
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "geometry"
    }

    to_sql_checked!();
}
