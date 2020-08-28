extern crate fxhash;

use crate::dynamodb::qruuid::gen_uuid_str;
use crate::model::qrcode::QrCode;
use crate::model::qrgroup::QrGroup;
use crate::model::qrimage::QrImage;
use crate::model::qritem::QrItem;
use crate::model::schema::SORT_KEY_GROUP_VAL;
use image::DynamicImage;
use image::ImageError;
use std::error::Error;

use std::path::PathBuf;

fn get_fixture_image() -> Result<DynamicImage, ImageError> {
    let mut img_source_dir = PathBuf::from(&env!("CARGO_MANIFEST_DIR").to_owned());
    img_source_dir.push("assets");
    img_source_dir.push("tux");
    img_source_dir.set_extension("jpg");

    let img = image::open(img_source_dir);

    img
}

fn get_fixture_image2() -> Result<DynamicImage, ImageError> {
    let mut img_source_dir = PathBuf::from(&env!("CARGO_MANIFEST_DIR").to_owned());
    img_source_dir.push("assets");
    img_source_dir.push("tux-org");
    img_source_dir.set_extension("png");

    let img = image::open(img_source_dir);

    img
}

fn get_fixture_image_rgb() -> QrImage {
    let rgb_image = get_fixture_image().ok().unwrap().to_rgb();
    QrImage {
        image: rgb_image.clone(),
        hash32: fxhash::hash32(&rgb_image),
    }
}

fn get_fixture_image_rgb2() -> QrImage {
    let rgb_image = get_fixture_image2().ok().unwrap().to_rgb();
    QrImage {
        image: rgb_image.clone(),
        hash32: fxhash::hash32(&rgb_image),
    }
}

pub fn get_fixture_code() -> Result<QrCode, Box<dyn Error>> {
    let gid = gen_uuid_str("Test-uuid-2");
    let _qrgroup = QrGroup {
        group_id: gid,
        id: (*SORT_KEY_GROUP_VAL).to_owned(),
        qr_salt: *b"37AD14CBE27C4F3A544984B488513724", // "3kTMd".to_string(),
        qr_count: 4,
        qrcodes: vec![],
    };

    let qrcode_1 = QrCode {
        group_id: gid,
        id: 1, //gen_qr_id(&qrgroup, 1)?,
        title: Some("Attic (brown box)".to_string()),
        location: Some("Attic".to_string()),
        images: vec![],
        items: vec![],
    };

    Ok(qrcode_1)
}

pub fn get_fixture() -> Result<QrGroup, Box<dyn Error>> {
    let gid = gen_uuid_str("Test-uuid");
    let mut qrgroup = QrGroup {
        group_id: gid,
        id: (*SORT_KEY_GROUP_VAL).to_owned(),
        qr_salt: *b"37AD14CBE27C4F3A544984B488513724", // "3kTMd".to_string(),
        qr_count: 4,
        qrcodes: vec![],
    };

    let qr_item_1 = QrItem {
        name: "Garden hose".to_string(),
    };
    let qr_item_2 = QrItem {
        name: "Soldering iron".to_string(),
    };
    let qr_item_3 = QrItem {
        name: "Laptop".to_string(),
    };
    let qr_item_4 = QrItem {
        name: "Games".to_string(),
    };
    let qr_item_5 = QrItem {
        name: "Toys".to_string(),
    };

    //let qr_image_1 = QrImage

    let qrcode_1 = QrCode {
        group_id: gid,
        id: 1, //gen_qr_id(&qrgroup, 1)?,
        title: Some("Attic (brown box)".to_string()),
        location: Some("Attic".to_string()),
        images: vec![],
        items: vec![qr_item_1, qr_item_2, qr_item_3],
    };
    let qrcode_2 = QrCode {
        group_id: gid,
        id: 2, //gen_qr_id(&qrgroup, 2)?,
        title: Some("Bag of tricks".to_string()),
        location: Some("Attic".to_string()),
        images: vec![],
        items: vec![],
    };
    let qrcode_3 = QrCode {
        group_id: gid,
        id: 3, //gen_qr_id(&qrgroup, 3)?,
        title: Some("Left drawer".to_string()),
        location: Some("Kitchen".to_string()),
        images: vec![get_fixture_image_rgb(), get_fixture_image_rgb2()],
        items: vec![qr_item_4],
    };
    let qrcode_4 = QrCode {
        group_id: gid,
        id: 4, //gen_qr_id(&qrgroup, 4)?,
        title: Some("Car".to_string()),
        location: None,
        images: vec![get_fixture_image_rgb2()],
        items: vec![qr_item_5],
    };

    let qr_codes = vec![qrcode_1, qrcode_2, qrcode_3, qrcode_4];

    qrgroup.qrcodes = qr_codes;

    Ok(qrgroup)
}
