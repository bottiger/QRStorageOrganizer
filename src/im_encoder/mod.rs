extern crate image;
extern crate qrcode;
//extern crate qrcodegen;

use image::DynamicImage;

use crate::model::qrcode::QrCode;

use crate::dynamodb::qruuid::gen_qr_scan_val;
use image::ImageError;

use image::Luma;
use qrcode::QrCode as QrCodeGen;
use std::path::Path;

//use crate::types;

pub fn to_img(qr: &QrCode) -> DynamicImage {
    let val = gen_qr_scan_val(qr);
    // Encode some data into bits.
    let code = QrCodeGen::new(val).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    DynamicImage::ImageLuma8(image)
    // Save the image.
    //image.save(path).unwrap();
}

pub fn write_img(path: &Path, qr: &QrCode) -> Result<(), ImageError> {
    let image = to_img(qr);
    image.save(path)
}
