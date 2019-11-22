use crate::{
    encode::IsNull,
    mariadb::{
        protocol::{FieldType, ParameterFlag},
        types::MariaDbTypeMetadata,
    },
    Decode, Encode, HasSqlType, MariaDb,
};

impl HasSqlType<[u8]> for MariaDb {
    fn metadata() -> MariaDbTypeMetadata {
        MariaDbTypeMetadata {
            field_type: FieldType::MYSQL_TYPE_BLOB,
            param_flag: ParameterFlag::empty(),
        }
    }
}

impl HasSqlType<Vec<u8>> for MariaDb {
    fn metadata() -> MariaDbTypeMetadata {
        <Self as HasSqlType<[u8]>>::metadata()
    }
}

impl Encode<MariaDb> for [u8] {
    fn encode(&self, buf: &mut Vec<u8>) -> IsNull {
        buf.extend_from_slice(self);
        IsNull::No
    }
}

impl Encode<MariaDb> for Vec<u8> {
    fn encode(&self, buf: &mut Vec<u8>) -> IsNull {
        <[u8] as Encode<MariaDb>>::encode(self, buf)
    }
}

impl Decode<MariaDb> for Vec<u8> {
    fn decode(raw: Option<&[u8]>) -> Self {
        raw.unwrap().into()
    }
}
