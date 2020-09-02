extern crate base64;

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


// const VERSION: u8 = 1;

lazy_static! {
    pub static ref BASE_URL: &'static str = "https://api.qrst.dk/";
}

/// Each QR code contains 256 bits
/// The first 192 bits are the group ID
/// The next 64 bits are for the QR code

pub fn to_base64(bytes: &Bytes) -> String {
    encode(&bytes)
}

/*
pub fn vec_to_u256(v: &Vec<u8>) -> Result<u256, DecodeError> {
    slice_to_u256(v.as_slice())
}
*/
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

pub fn gen_uuid_str(name: &str) -> u192 {
    gen_uuid(name.as_bytes())
}

pub fn gen_uuid(name: &[u8]) -> u192 {
    let mut hasher = Sha3_256::new();
    hasher.input(name);
    let result = hasher.result();

    let slice = result.as_slice();
    info!("Hash result {:?}", slice);

    let u192_slice = &slice[..24];
    let u192_val = slice_to_u192(u192_slice);

    match u192_val {
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

pub fn parse_qr_string_val(input: &str) -> Result<DynamoPrimaryKey, base_62::base62::Error> {
    
    let bytes: Vec<u8> = base_62::decode(input)?;

    let mut p: [u8; 24] = Default::default();
    let sl: [u8; 24] = bytes[..24].try_into().unwrap();
    p.copy_from_slice(&sl);

    let s2: [u8; 8] = bytes[24..32].try_into().unwrap();
    let s = u64::from_ne_bytes(s2);

    Ok(DynamoPrimaryKey { partition_key: p, sort_key: s })
}

pub fn gen_qr_scan_val(code: &QrCode) -> String {
    let mut bytes1 : Vec<u8> = code.group_id.into();
    let mut bytes2 : Vec<u8> = u64::to_ne_bytes(code.id).to_vec();
    bytes1.append(&mut bytes2);


    let url = BASE_URL.to_string() + &base_62::encode(&bytes1);
    log::trace!("qr url: {:?}", url);

    url
}

pub fn parse_qr_val(val: String) -> Result<DynamoPrimaryKey, base_62::base62::Error> {
    parse_qr_string_val(&val) //DynamoPrimaryKey::from_str(&val)
}

pub fn gen_qr_scan_val_short(_group: &QrGroup, val: DynamoSearchKey) -> String {
    let url = &val.to_string();

    url.to_string()
}
