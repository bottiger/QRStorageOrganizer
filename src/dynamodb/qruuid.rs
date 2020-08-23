extern crate base64;

use harsh::{HarshBuilder};
use uuid::Uuid;
use log::info;
use crate::model::qrgroup::QrGroup;
use crate::model::schema::u256;
use crate::model::schema::DynamoPartitionKey;
use sha3::{Digest, Sha3_256};
use base64::{encode, decode, DecodeError};
use bytes::Bytes;
use std::convert::TryInto;

const VERSION: u8 = 1;

pub fn to_base64(bytes: Bytes) -> String {
    let encoded = encode(bytes);

    encoded
}

pub fn vec_to_u256(v: &Vec<u8>) -> Result<u256, DecodeError> {
    slice_to_u256(v.as_slice())
}

pub fn slice_to_u256(s: &[u8]) -> Result<u256, DecodeError> {
    match s.try_into() {
        Ok(v) => Ok(v),
        Err(E) => Err(DecodeError::InvalidLength),
    }
}

pub fn from_base64(str: String) -> Result<u256, DecodeError> {
    base64::decode(&str).and_then(|v| vec_to_u256(&v))
}

pub fn gen_uuid_str(name: &str) -> u256 {
    gen_uuid(name.as_bytes())
}

pub fn gen_uuid(name: &[u8]) -> u256 {

    let mut hasher = Sha3_256::new();
    hasher.input(name);
    let result = hasher.result();

    slice_to_u256(result.as_slice()).ok().unwrap()
}


pub fn gen_qr_id(group: &QrGroup, val: u64) -> harsh::Result<String> {

    let harsh = HarshBuilder::new().salt(group.qr_salt.to_owned().to_vec()).init()?;

    let valvec = vec![val];
    let qr_id = harsh.encode(&valvec).unwrap();
    info!("Calculating ID. from {:?} => {:?}", val, qr_id);
    Ok(qr_id)
}
