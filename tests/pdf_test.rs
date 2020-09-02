#[cfg(test)]
mod tests {

    use qrstore::pdf_generator::new_codes_pdf;
    use qrstore::config::init_env;
    use qrstore::fixtures::get_fixture;
    use qrstore::pdf_generator::QrPdfLayout;
    use qrstore::pdf_generator::make_pdf;
    use qrstore::pdf_generator::save_pdf;
    use std::fs::File;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_pdf() {
        env_logger::init();
        init_env();
        let mut f = get_fixture().unwrap();

        let mut pdf_source = PathBuf::from(&env!("CARGO_MANIFEST_DIR").to_owned());
        pdf_source.push("out");
        pdf_source.push("qrcodes");
        pdf_source.set_extension("pdf");

        let file = File::create(pdf_source).unwrap();

        log::debug!("Writing pdf to location: {:?}", file);

        let doc = new_codes_pdf(&mut f, 100, QrPdfLayout::default()).await.ok().unwrap();

        let res = save_pdf(doc, file);

        assert!(res.is_ok());
    }
}
