#[cfg(feature = "default")]
use rusoto_core_default::Region;

use crate::dynamodb::qruuid::parse_qr_string_val;
use std::num::ParseIntError;
use base64::DecodeError;
use std::str::FromStr;
use dynomite::AttributeError;
use dynomite::AttributeValue;
#[cfg(feature = "rustls")]
use rusoto_core_rustls::Region;
use std::collections::HashMap;
use std::convert::TryInto;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref TABLE_NAME: &'static str = "qrstorage";
    pub static ref PARTITION_KEY_NAME: &'static str = "qr_group_id";
    pub static ref SORT_KEY_NAME: &'static str = "qr_val";
    pub static ref SORT_KEY_GROUP_VAL: &'static u64 = &0;
}

#[allow(non_camel_case_types)]
pub type u256 = [u8; 32];
#[allow(non_camel_case_types)]
pub type u192 = [u8; 24];
#[allow(non_camel_case_types)]
pub type u256DB = Vec<u8>;
#[allow(non_camel_case_types)]
pub type u192DB = Vec<u8>;

pub type DynamoPartitionKeyDB = u192DB;
pub type DynamoPartitionKey = u192;
pub type DynamoSearchKey = u64;
#[derive(Serialize, Deserialize)]
pub struct DynamoPrimaryKey {
    pub partition_key: DynamoPartitionKey,
    pub sort_key: DynamoSearchKey,
}

pub enum KeyParseError {
    ParseIntError(std::num::ParseIntError),
    ParseStringError(DecodeError),
}

impl From<ParseIntError> for KeyParseError {
    fn from(e: ParseIntError) -> Self {
        KeyParseError::ParseIntError(e)
    }
}

impl From<DecodeError> for KeyParseError {
    fn from(e: DecodeError) -> Self {
        KeyParseError::ParseStringError(e)
    }
}

impl FromStr for DynamoPrimaryKey {
    type Err = base_62::base62::Error;

    fn from_str(url_str: &str) -> Result<Self, Self::Err> {
    
        parse_qr_string_val(url_str)
        /*
        let parts: Vec<&str> = url_str.split('-').collect();
        let byte_vec = base64::decode(parts[0])?;

        let p: DynamoPartitionKey = match vec_to_u192(&byte_vec) {
            Ok(ba) => ba,
            Err(_o) => {
                let sv = base64::decode(parts[0]);
                println!("input: {:?} byte: {:?} length: {:?}", sv, parts[0], parts[0].len());
                return Err(KeyParseError::ParseStringError(DecodeError::InvalidLength))
            },
        };

        let s: DynamoSearchKey = u64::from_str(parts[1])?;

        Ok(DynamoPrimaryKey { partition_key: p, sort_key: s })
        */
    }
}

pub fn slice_to_partition_key(s: &[u8]) -> Result<DynamoPartitionKey, AttributeError> {
    match s.try_into() {
        Ok(v) => Ok(v),
        Err(_e) => Err(AttributeError::InvalidFormat),
    }
}

pub enum DynamoDbType {
    QrGroup,
    QrCode,
    Image,
}

pub trait DbItem {
    fn get_primary_key(&self) -> DynamoPrimaryKey;
    fn get_partition_key(&self) -> DynamoPartitionKey;
    fn get_sort_key(&self) -> DynamoSearchKey;
    fn get_type(&self) -> DynamoDbType;
    fn get_attribute_value_map(&self) -> HashMap<String, AttributeValue>;
    fn get_update_expr(&self) -> (Option<HashMap<String, AttributeValue>>, Option<String>);
}
