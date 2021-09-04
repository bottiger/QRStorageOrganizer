#[cfg(test)]
mod tests {

    use qrstore::config::init_env;
    use qrstore::fixtures::get_fixture;
    use qrstore::im_encoder::write_img;
    use std::fs;
    use std::path::PathBuf;
    use qrstore::dynamodb::qruuid::gen_qr_scan_val;
    use qrstore::dynamodb::qruuid::parse_qr_val;

    fn test_model() {

        env_logger::init();

        init_env();

        let f = get_fixture().unwrap();

        let qr = f.qrcodes.first().unwrap();

        log::info!("QR group id: {:?}", qr.group_id);
        log::info!("QR id: {:?}", qr.id);

        let qr_str = gen_qr_scan_val(qr);

        log::info!("scan_val: {:?}", qr_str);

        let parsed_qr = parse_qr_val(qr_str);

        assert!(parsed_qr.is_ok());

        let parsed_primary_key = parsed_qr.ok().unwrap();

        log::info!("parsed group id: {:?}", parsed_primary_key.partition_key.clone());
        log::info!("parsed id: {:?}", parsed_primary_key.sort_key.clone());

        assert_eq!(parsed_primary_key.partition_key.clone(), qr.group_id);
        assert_eq!(parsed_primary_key.sort_key.clone(), qr.id);
    }
}
