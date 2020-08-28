#[cfg(test)]
mod tests {

    use qrstore::config::init_env;
    use qrstore::fixtures::get_fixture;
    use qrstore::im_encoder::write_img;
    use std::fs;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_filesystem() {
        env_logger::init();

        init_env();

        let f = get_fixture().unwrap();

        let mut image_source = PathBuf::from(&env!("CARGO_MANIFEST_DIR").to_owned());
        image_source.push("out");
        image_source.push("qr_test");
        image_source.set_extension("jpg");

        log::debug!("Remove file if it exists in advance");
        let _rmres = fs::remove_file(&image_source);

        log::debug!("Verify the file does not exist before writing it");
        assert_eq!(image_source.is_file(), false);

        log::debug!("Write file to disk");
        write_img(&image_source, &f.qrcodes[0]);

        log::debug!(
            "Verify the file ({:?}) does exist after writing it",
            image_source
        );
        assert!(image_source.is_file());
    }
}
