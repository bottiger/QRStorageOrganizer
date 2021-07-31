extern crate base64;

use crate::model::schema::slice_to_partition_key;
use std::mem::size_of;
use crate::model::schema::DynamoPartitionKey;
use crate::model::schema::u128;
use crate::model::schema::u192;
use base_62::base62::Error;

use crate::model::schema::DynamoPrimaryKey;
use crate::model::qrcode::QrCode;
use crate::model::schema::DynamoSearchKey;
use harsh::HarshBuilder;

use crate::model::qrgroup::QrGroup;
use crate::model::schema::u256;
use log::info;

use base_62::base62::encode;
use bytes::Bytes;
use sha3::{Digest, Sha3_256};
use std::convert::TryInto;
use vartyint;


// const VERSION: u8 = 1;

lazy_static! {
    pub static ref BASE_URL: &'static str = "https://qrst.dk/";
}

/// Each QR code contains 256 bits
/// The first 192 bits are the group ID
/// The next 64 bits are for the QR code 

/// Each QR code contains 256 bits
/// The first 8 bits are version number
/// The next 192 bits are the group ID
/// The next 56 bits are for the QR code

pub fn to_base64(bytes: &Bytes) -> String {
    encode(&bytes)
}

pub fn vec_to_u128(v: &[u8]) -> Result<u128, base_62::base62::Error> {
    slice_to_u128(v)
}

pub fn slice_to_u128(s: &[u8]) -> Result<u128, base_62::base62::Error> {
    match s.try_into() {
        Ok(v) => Ok(v),
        Err(_e) => Err(Error::BadCharacter{character: 'z'}),
    }
}

pub fn vec_to_u256(v: &[u8]) -> Result<u256, base_62::base62::Error> {
    slice_to_u256(v)
}

pub fn slice_to_u256(s: &[u8]) -> Result<u256, base_62::base62::Error> {
    match s.try_into() {
        Ok(v) => Ok(v),
        Err(_e) => Err(Error::BadCharacter{character: 'z'}),
    }
}

pub fn vec_to_u192(v: &[u8]) -> Result<u192, base_62::base62::Error> {
    slice_to_u192(v)
}

pub fn slice_to_u192(s: &[u8]) -> Result<u192, base_62::base62::Error> {
    match s.try_into() {
        Ok(v) => Ok(v),
        Err(_e) => Err(Error::BadCharacter{character: 'z'}),
    }
}

pub fn from_base64(str: String) -> Result<u256, base_62::base62::Error> {
    base_62::decode(&str).and_then(|v| vec_to_u256(&v))
}

pub fn gen_uuid_str(name: &str) -> DynamoPartitionKey {
    gen_uuid(name.as_bytes())
}

pub fn gen_uuid(name: &[u8]) -> DynamoPartitionKey {
    let mut hasher = Sha3_256::new();
    hasher.input(name);
    let result = hasher.result();

    let slice = result.as_slice();
    info!("Hash result {:?}", slice);

    let u128_slice = &slice[..size_of::<DynamoPartitionKey>()];
    let u128_val = slice_to_partition_key(u128_slice);

    match u128_val {
        Ok(v) => v,
        Err(_e) => panic!("this is a terrible mistake!"),
    }
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
    let url = BASE_URL.to_string() + &gen_encoded_string(code);
    log::trace!("qr url: {:?}", url);

    url
}

pub fn gen_encoded_string(code: &QrCode) -> String {
    let mut bytes1 : Vec<u8> = code.group_id.into();
    // let mut bytes2 : Vec<u8> = u64::to_ne_bytes(code.id).to_vec();
    
    let mut bytes2 = Vec::new();
    vartyint::write_u64(code.id, &mut bytes2);
    
    bytes1.append(&mut bytes2);


    base_62::encode(&bytes1)
}

#[derive(Debug, Clone)]
pub struct QrParsingError;

pub fn parse_qr_val(val: String) -> Result<DynamoPrimaryKey, QrParsingError> {
    let bytes: Vec<u8> = base_62::decode(&val).map_err(|_e| QrParsingError)?;

    let expected_bytesize = size_of::<DynamoPartitionKey>() + size_of::<DynamoSearchKey>();
    log::debug!("Byte length: {:?}. Expect: {}", bytes.len(), expected_bytesize);

    let mut p: [u8; size_of::<DynamoPartitionKey>()] = Default::default();
    let part_key_bytes = bytes.get(0..size_of::<DynamoPartitionKey>()).ok_or(QrParsingError)?;
    let sl: [u8; size_of::<DynamoPartitionKey>()] = part_key_bytes.try_into().map_err(|_e| QrParsingError)?;
    p.copy_from_slice(&sl);


    
    let first_byte = size_of::<DynamoPartitionKey>();
    let num_bytes = bytes.len();
    let _qrid_length = num_bytes - first_byte;

    /*
    let qrid_bytes = bytes.get(first_byte..num_bytes).ok_or(QrParsingError)?;
    let s2 = qrid_bytes.try_into().map_err(|_e| QrParsingError)?;
    let s = u64::from_ne_bytes(s2);
    */

    let qrid_bytes = bytes.get(first_byte..num_bytes).ok_or(QrParsingError)?;
    let (s, _qrid_bytes) = vartyint::read_u64(&qrid_bytes).unwrap();

    Ok(DynamoPrimaryKey { partition_key: p, sort_key: s })
}

pub fn gen_qr_scan_val_short(_group: &QrGroup, val: DynamoSearchKey) -> String {
    let url = &val.to_string();

    url.to_string()
}
