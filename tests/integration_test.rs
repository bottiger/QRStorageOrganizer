#[cfg(test)]
mod tests {

    use std::fs;
    use std::path::Path;

    use qrstore::config::init_env;
    use qrstore::datastore::datastore::DataStore;
    use qrstore::fixtures::get_fixture;
    use qrstore::model::qrcode::QrCode;
    use qrstore::datastore::sqlite::SQLiteDataStore;


    #[tokio::test]
    async fn test_crud() {
        env_logger::init();
        init_env();

        log::debug!("Starting test");
        println!("Starting test");

        let db_name = "test.db";
        reset_database(db_name);

        let mut datastore = SQLiteDataStore::new(db_name).await.unwrap();
        let f = get_fixture().unwrap();

        assert_eq!(f.qrcodes.len(), 4);

        assert!(datastore.create_group(&f).await.is_ok());

        let group_before = datastore.get_group(&f.group_id).await.unwrap().unwrap();
        let num_codes = group_before.qrcodes.len();

        assert_eq!(num_codes, 4);

        for code in group_before.qrcodes.iter().step_by(2) {
            let codedb: QrCode = code.clone();
            log::debug!("delete {:?}", codedb);
            assert!(datastore.delete_qrcode(&group_before.group_id, &codedb.id).await.is_ok());
        }

        let group_after = datastore.get_group(&f.group_id).await.unwrap().unwrap();
        let num_codes_after = group_after.qrcodes.len();

        log::debug!("{}", num_codes);
        log::debug!("{}", num_codes_after);

        assert!(num_codes > num_codes_after);

        // Update a QR code's title
        let mut code_to_update = group_after.qrcodes.first().unwrap().clone();
        code_to_update.title = Some("Updated Title".to_string());
        assert!(datastore.update_qrcode(&code_to_update).await.is_ok());

        let updated_group = datastore.get_group(&f.group_id).await.unwrap().unwrap();
        let updated_code = updated_group.qrcodes.iter().find(|code| code.id == code_to_update.id).unwrap();
        assert_eq!(updated_code.title, Some("Updated Title".to_string()));

        // Delete the group
        assert!(datastore.delete_group(&f.group_id).await.is_ok());
        let deleted_group = datastore.get_group(&f.group_id).await.unwrap();
        assert!(deleted_group.is_none());

        /*
        assert!(datastore.create_group(&f).await.is_ok());
        assert!(datastore.get_group(&f.group_id).await.is_ok());

        assert!(datastore.create_group(&f).await.is_ok());

        let qrg = datastore.get_group(&f.group_id).await.unwrap().unwrap();
        let code_before = qrg.qrcodes.first().unwrap();

        let mut code_after = code_before.clone();
        let after_title = String::from("New title");
        code_after.title = Some(after_title.clone());

        let qr_code_db: QrCode = code_after.clone();
        datastore.update_qrcode(&qr_code_db).await.unwrap();

        let qrg_after = datastore.get_group(&f.group_id).await.unwrap().unwrap();
        let code_after_get = qrg_after.qrcodes.first().unwrap();

        assert_ne!(code_before.title, Some(after_title.clone()));
        assert_eq!(Some(after_title), code_after_get.title);
        */
    }

    fn reset_database(db_name: &str) {
        if Path::new(db_name).exists() {
            if let Err(e) = fs::remove_file(db_name) {
                eprintln!("Failed to delete the file: {}", e);
            } else {
                println!("File '{}' deleted successfully.", db_name);
            }
        } else {
            println!("File '{}' does not exist.", db_name);
        }
    }
}
