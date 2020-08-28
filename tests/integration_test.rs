#[cfg(test)]
mod tests {

    use qrstore::config::init_env;
    use qrstore::dynamodb::crud::delete;
    use qrstore::dynamodb::crud::update;
    use qrstore::dynamodb::get_group;
    use qrstore::dynamodb::insert_group;
    use qrstore::fixtures::get_fixture;
    use qrstore::model::qrcode::QrCodeDB;
    use qrstore::model::schema::DbItem;

    #[tokio::test]
    async fn test_crud() {
        env_logger::init();

        init_env();

        log::debug!("Starting test");
        println!("Starting test");

        let f = get_fixture().unwrap();
        assert!(insert_group(&f).await.is_ok());

        let group_before = get_group(&f.group_id).await.unwrap();
        let num_codes = group_before.qrcodes.len();

        for code in group_before.qrcodes.iter().step_by(2) {
            let codedb: QrCodeDB = QrCodeDB::from(code.clone());
            log::debug!("delete {:?}", codedb);
            assert!(delete(&codedb.get_primary_key()).await.is_ok());
        }

        let group_after = get_group(&f.group_id).await.unwrap();
        let num_codes_after = group_after.qrcodes.len();

        log::debug!("{}", num_codes);
        log::debug!("{}", num_codes_after);

        assert!(num_codes > num_codes_after);

        let r2 = insert_group(&f);
        assert!(get_group(&f.group_id).await.is_ok());

        let r3 = insert_group(&f);
        let qrg = get_group(&f.group_id).await.unwrap();
        let code_before = qrg.qrcodes.first().unwrap();

        let mut code_after = code_before.clone();
        let after_title = String::from("New title");
        code_after.title = Some(after_title.clone());

        let qrCodeDB: QrCodeDB = QrCodeDB::from(code_after);
        update(&qrCodeDB).await;

        let qrg_after = get_group(&f.group_id).await.unwrap();
        let code_after_get = qrg_after.qrcodes.first().unwrap();

        assert_ne!(code_before.title, Some(after_title.clone()));
        assert_eq!(Some(after_title), code_after_get.title);
    }
}
