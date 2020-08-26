#[cfg(feature = "default")]
use rusoto_core_default::Region;

#[cfg(feature = "rustls")]
use rusoto_core_rustls::Region;
use dynomite::AttributeError;
use dynomite::AttributeValue;
use std::collections::HashMap;
use std::convert::TryInto;

//use self::dynomite::Attributes;

lazy_static! {
    pub static ref TABLE_NAME: &'static str = "qr-storage";
    pub static ref PARTITION_KEY_NAME: &'static str = "qr_group_id";
    pub static ref SORT_KEY_NAME: &'static str = "qr_val";
    pub static ref SORT_KEY_GROUP_VAL: &'static str = "qrgroup";
}


#[warn(non_camel_case_types)]
pub type u256 = [u8; 32];
#[warn(non_camel_case_types)]
pub type u256DB = Vec<u8>;
pub type DynamoPartitionKeyDB = u256DB;
pub type DynamoPartitionKey = u256;
pub type DynamoSearchKey    = String;
pub struct DynamoPrimaryKey {
    pub partition_key: DynamoPartitionKey,
    pub sort_key: DynamoSearchKey,
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