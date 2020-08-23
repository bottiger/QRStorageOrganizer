extern crate dynomite;

use std::fmt;
use dynomite::Item;
use dynomite::error::AttributeError;
use dynomite::Attribute;
use rusoto_dynamodb::AttributeValue;
use dynomite::FromAttributes;
use chrono::NaiveDateTime;

use std::collections::HashMap;
use std::convert::From;
use image::RgbImage;

use std::convert::TryInto;

use crate::dynamodb::qruuid::vec_to_u256;
use crate::dynamodb::qruuid::slice_to_u256;





/*
impl fmt::Debug for QrImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Image)")
    }
}
*/

/*
impl Copy for QrImage { }

impl Clone for QrImage {
    fn clone(&self) -> Self {
        *self
    }
}
*/




/*
impl Attribute for DynamoPartitionKey {
    fn into_attr(self: Self) -> AttributeValue {
        self.0.to_vec().into_attr()
    }

    fn from_attr(value: AttributeValue) -> Result<Self, AttributeError> {
        value
            .b
            .ok_or(AttributeError::InvalidType)
            .and_then(|bs| slice_to_partition_key(bs.as_ref()))
    }
}
*/

/*
impl Item for DynamoPartitionKey {
    fn key(&self) -> Attributes {
        let mut attrs = HashMap::new();
        attrs.insert("id".into(), "123".to_string().into_attr());
        attrs
    }
}
*/


/*
impl fmt::Debug for DynamoPartitionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0.to_vec())
    }
}
*/

/*
impl Attribute for DynamoPrimaryKey {
    fn into_attr(self: Self) -> AttributeValue {
        AttributeValue {
            s: Some(self.name),
            ..AttributeValue::default()
        }
    }

    fn from_attr(value: AttributeValue) -> Result<Self, AttributeError> {
        value.s.map(|s| QrItem{name: s}).ok_or(AttributeError::InvalidType)
    }
}
*/

















/*

// Old types
pub type IQr = i64;
type SqliteTime = NaiveDateTime;

//#[derive(Insertable)]
//#[table_name = "qrcodes"]
pub struct Qrform<'a> {
    pub qr: IQr,
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub images: Option<&'a str>,
}

//#[derive(Queryable, Identifiable, AsChangeset, PartialEq, Debug)]
//#[table_name = "qrcodes"]
pub struct QrEntry {
    pub id: i32,
    pub qr: IQr,
    pub title: Option<String>,
    pub body: Option<String>,
    pub images: Option<String>,
    pub created_at: SqliteTime,
    pub updated_at: SqliteTime,
}

*/
