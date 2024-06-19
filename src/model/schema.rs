#[cfg(feature = "default")]
use rusoto_core_default::Region;

use std::{array::TryFromSliceError, num::ParseIntError};
use base64::DecodeError;
use std::str::FromStr;
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
pub type u128 = [u8; 12];
#[allow(non_camel_case_types)]
pub type u128DB = Vec<u8>;
#[allow(non_camel_case_types)]
pub type u192 = [u8; 24];
#[allow(non_camel_case_types)]
pub type u192DB = Vec<u8>;
#[allow(non_camel_case_types)]
pub type u256 = [u8; 32];
#[allow(non_camel_case_types)]
pub type u256DB = Vec<u8>;

pub type QrVersion = u8;
pub type DynamoPartitionKeyDB = u128DB;
pub type DynamoPartitionKey = u128;
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


pub fn slice_to_partition_key(s: &[u8]) -> Result<DynamoPartitionKey, TryFromSliceError> {
    match s.try_into() {
        Ok(v) => Ok(v),
        Err(_e) => Err(_e),
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
    //fn get_attribute_value_map(&self) -> HashMap<String, AttributeValue>;
    //fn get_update_expr(&self) -> (Option<HashMap<String, AttributeValue>>, Option<String>);
}
