extern crate image;
extern crate qrcode;

use image::ImageError;
use image::GrayImage;
use qrcode::QrCode;
use image::Luma;


//use crate::types;


/*
fn to_img() {
    // Encode some data into bits.
    let code = QrCode::new(b"01234567").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("/tmp/qrcode.png").unwrap();
}
*/



// _entry: &types::QrEntry
pub fn to_img() -> GrayImage { //Luma<u8> {
    let code = QrCode::new(b"01234567").unwrap();
    

    let image = code.render::<Luma<u8>>().build();
    //      //let image: GrayImage = code.render::<GrayImage>().min_dimensions(100, 100 ).build();

    image

    //let string = code.render::<char>()
    //    .quiet_zone(false)
    //    .module_dimensions(2, 1)
    //    .build();
    //println!("{}", string);
}

//pub fn from_img(img2: ImageResult<DynamicImage>) -> String {

/*
pub fn from_img() -> String {
    let decoder = bardecoder::default_decoder();

    let img = image::open("D:\\qrstorage\\out\\qr.png").unwrap();
    let results = decoder.decode(&img);
    //for result in results {
    //    println!("{}", result.unwrap());
    //}

    match results.first() {
        Some(x) => {
            match x {
                Ok(v) => v.to_owned(),
                Err(_e) => "failure".to_owned(),
            }
        }
        None => "err".to_owned(),
    }
}
*/

pub fn write(path: &str, image: GrayImage) -> Result<(), ImageError> {
    image.save(path)
}

/*
pub fn to_str(_entry: &QrEntry) -> String {
    let code = QrCode::new(b"01234567").unwrap();
    let string = code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    string
}
*/

