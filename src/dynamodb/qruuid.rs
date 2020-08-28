extern crate base64;

use crate::model::qrcode::QrCode;
use crate::model::schema::DynamoSearchKey;
use harsh::HarshBuilder;

use crate::model::qrgroup::QrGroup;
use crate::model::schema::u256;
use log::info;

use base64::{encode, DecodeError};
use bytes::Bytes;
use sha3::{Digest, Sha3_256};
use std::convert::TryInto;

// const VERSION: u8 = 1;

/// Each QR code contains 256 bits
/// The first 192 bits are the group ID
/// The next 64 bits are for the QR code

pub fn to_base64(bytes: Bytes) -> String {
    encode(bytes)
}

/*
pub fn vec_to_u256(v: &Vec<u8>) -> Result<u256, DecodeError> {
    slice_to_u256(v.as_slice())
}
*/
pub fn vec_to_u256(v: &[u8]) -> Result<u256, DecodeError> {
    slice_to_u256(v)
}

pub fn slice_to_u256(s: &[u8]) -> Result<u256, DecodeError> {
    match s.try_into() {
        Ok(v) => Ok(v),
        Err(_e) => Err(DecodeError::InvalidLength),
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

pub fn gen_qr_id(group: &QrGroup, val: DynamoSearchKey) -> harsh::Result<String> {
    let harsh = HarshBuilder::new()
        .salt(group.qr_salt.to_owned().to_vec())
        .init()?;

    let valvec = vec![val];
    let qr_id = harsh.encode(&valvec).unwrap();
    info!("Calculating ID. from {:?} => {:?}", val, qr_id);
    Ok(qr_id)
}

pub fn gen_qr_scan_val(code: &QrCode) -> String {
    let prefix = "https://qrst.dk/".to_owned();
    prefix + &base64::encode(code.group_id) + "-" + &code.id.to_string()
}

pub fn gen_qr_scan_val_short(_group: &QrGroup, val: DynamoSearchKey) -> String {
    let url = &val.to_string();

    url.to_string()
}
