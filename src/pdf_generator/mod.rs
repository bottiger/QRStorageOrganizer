extern crate printpdf;

use crate::model::qrcode::QrCode;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn make_pdf(_qrcodes: Vec<QrCode>) -> PdfDocumentReference {
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
    let _current_layer = doc.get_page(page1).get_layer(layer1);

    doc
}

pub fn save_pdf(doc: PdfDocumentReference, file: File) -> Result<(), Error> {
    doc.save(&mut BufWriter::new(file))
}
