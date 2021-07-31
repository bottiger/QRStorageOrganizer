extern crate printpdf;

use crate::model::schema::DynamoSearchKey;
use rusoto_dynamodb::UpdateItemError;
use rusoto_core::RusotoError;
use crate::dynamodb::crud::update;
use crate::model::qrgroup::QrGroup;
use crate::model::qrgroup::QrGroupDB;
use crate::dynamodb::qruuid::gen_qr_scan_val;
use crate::im_encoder::to_img;
use crate::model::qrcode::QrCode;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub struct QrPdfLayout {
	pdf_width: scale::Mm,
	pdf_height: scale::Mm,
	pdf_margin_horizontal: scale::Mm,
    pdf_margin_vertical: scale::Mm,
	num_qr_horizontal: u8,
	num_qr_vertical: u8,
}

impl Default for QrPdfLayout {
    fn default() -> QrPdfLayout {
        QrPdfLayout { 
        	pdf_width: Mm(210.0),
        	pdf_height: Mm(297.0),
        	pdf_margin_horizontal: Mm(5.0),
        	pdf_margin_vertical: Mm(5.0),
        	num_qr_horizontal: 6,
        	num_qr_vertical: 10,
        }
   }
}

pub async fn new_codes_pdf(group: &mut QrGroup, amount: usize, layout: QrPdfLayout) -> Result<PdfDocumentReference, RusotoError<UpdateItemError>> {

	let mut new_codes = Vec::<QrCode>::with_capacity(amount);

	for i in 0..amount {
		let qr_id: DynamoSearchKey = (group.qr_count + (i as u32)).into();
		log::debug!("New qrID: {:?}", qr_id);
		let new_code = QrCode {
            group_id: group.group_id,
            id: qr_id,
            title: None,
            location: None,
            images: Vec::new(),
            items: Vec::new(),
			content: None,
        };

		new_codes.push(new_code);
	}

	group.qr_count = group.qr_count + amount as u32;
	update(&QrGroupDB::from(group.clone())).await?;

	Ok(make_pdf(&new_codes, layout))
}

pub fn make_pdf(qrcodes: &Vec<QrCode>, layout: QrPdfLayout) -> PdfDocumentReference {
    let (doc, page1, layer1) = PdfDocument::new("QR codes", layout.pdf_width, layout.pdf_height, "Layer 1");

    let mut page = 1;
    let mut col  = 0;
    let mut row  = 0;
    let curr_page = page1;
    let curr_layer = layer1;
    let mut current_layer = doc.get_page(curr_page).get_layer(curr_layer);

    let img_offset_x = (layout.pdf_width - layout.pdf_margin_horizontal - layout.pdf_margin_horizontal)  / layout.num_qr_horizontal as f64;
    let img_offset_y = (layout.pdf_height - layout.pdf_margin_vertical - layout.pdf_margin_vertical) / layout.num_qr_vertical as f64;

    for qrcode in qrcodes {
    	let dyn_img = to_img(&qrcode);
    	let image2 = Image::from_dynamic_image(&dyn_img);

    	// translate x, translate y, rotate, scale x, scale y
    	// by default, an image is optimized to 300 DPI (if scale is None)
    	// rotations and translations are always in relation to the lower left corner
    	let x_margin = layout.pdf_margin_horizontal;
    	let x_image_offset = img_offset_x * col as f64;
    	let x_center_in_column_offset = Mm(5.0); // make this dynamic

    	let x_pos = Some(x_margin + x_image_offset + x_center_in_column_offset);
    	let y_pos = Some((layout.pdf_height - layout.pdf_margin_vertical) - img_offset_y * (row+1) as f64);

    	log::debug!("Insert code: {:?} at x: {:?} y: {:?} at page {:?}", gen_qr_scan_val(&qrcode), x_pos, y_pos, page);
    	image2.add_to_layer(current_layer.clone(), x_pos, y_pos, None, None, None, None);

    	let new_row = (col + 1) % layout.num_qr_horizontal == 0;
    	if new_row {
    		col = 0;
    		row = row + 1;
    	} else {
    		col = col + 1;
    	}

    	let new_page = (row + 1) > layout.num_qr_vertical;
    	if new_page {
    		let (new_page, new_layer) = doc.add_page(layout.pdf_width, layout.pdf_height,format!("Page {}, Layer 1", page));
    		current_layer = doc.get_page(new_page).get_layer(new_layer);

    		page = page + 1;
    		row = 0;
    		col = 0;
    	}
    }

	doc
}

pub fn save_pdf(doc: PdfDocumentReference, file: File) -> Result<(), Error> {
    doc.save(&mut BufWriter::new(file))
}
