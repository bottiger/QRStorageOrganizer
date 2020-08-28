extern crate fxhash;

use crate::model::schema::DynamoPartitionKeyDB;
use crate::model::schema::DynamoSearchKey;
use dynomite::Item;
use image::load_from_memory;
use image::ImageError;
use image::RgbImage;

pub type QrImageHash = u32;

#[derive(Debug, Clone)]
pub struct QrImage {
    pub hash32: QrImageHash,
    pub image: RgbImage,
}

impl QrImage {
    pub fn new(data: Vec<u8>) -> Result<QrImage, ImageError> {
        let rgb_image = load_from_memory(&data)?.to_rgb();
        let qr_image = QrImage {
            image: rgb_image.clone(),
            hash32: fxhash::hash32(&rgb_image),
        };

        Ok(qr_image)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Format {
    None,
    Jpg,
    Png,
    WebP,
}

impl Default for Format {
    fn default() -> Self {
        Format::None
    }
}

#[derive(Item, Default, PartialEq, Debug, Clone)]
pub struct QrImageDB {
    #[dynomite(partition_key)]
    #[dynomite(rename = "qr_group_id")] //remote name
    pub group_id: DynamoPartitionKeyDB,
    #[dynomite(sort_key)]
    #[dynomite(rename = "qr_val")] //remote name
    pub id: DynamoSearchKey,
    binary_data: Vec<u8>,
    binary_format: Vec<u32>, //Format,
}
