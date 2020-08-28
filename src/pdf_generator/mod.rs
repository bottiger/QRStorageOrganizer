extern crate printpdf;

use image::bmp::BmpDecoder;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Cursor;

pub fn make_pdf() -> PdfDocumentReference {
    // qrcodes: Vec<QrCode>
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // currently, the only reliable file format is bmp (jpeg works, but not in release mode)
    // this is an issue of the image library, not a fault of printpdf

    //let mut image_file = File::open("C:\\Users\\arbo\\Google Drive\\qrgithub\\assets\\pdf-img.bmp").unwrap();
    //let image = Image::try_from(image::bmp::BmpDecoder::new(&mut image_file).unwrap()).unwrap();
    //let image_bytes = include_bytes!("/mnt/c/Users/arbo/Google Drive/qrgithub/assets/pdf-img.bmp");
    let image_bytes =
        include_bytes!("C:\\Users\\arbo\\Google Drive\\qrgithub\\assets\\pdf-img.bmp");
    let mut reader = Cursor::new(image_bytes.as_ref());

    let decoder = BmpDecoder::new(&mut reader).unwrap();
    let image2 = Image::try_from(decoder).unwrap();

    log::debug!("decoded image: {:?}", image2);

    // translate x, translate y, rotate, scale x, scale y
    // by default, an image is optimized to 300 DPI (if scale is None)
    // rotations and translations are always in relation to the lower left corner
    image2.add_to_layer(
        current_layer,
        Some(Mm(100.0)),
        Some(Mm(100.0)),
        None,
        None,
        None,
        None,
    );

    doc
}

pub fn save_pdf(doc: PdfDocumentReference, file: File) -> Result<(), Error> {
    let res = doc.save(&mut BufWriter::new(file));

    res
}
