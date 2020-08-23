#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use qrstore::fixtures::get_fixture;
    use qrstore::dynamodb::insert_group;
    use qrstore::dynamodb::get_group;
    use qrstore::dynamodb::crud::delete;
    use qrstore::dynamodb::crud::update;
    use qrstore::types::{QrCodeDB, DbItem};

    #[tokio::test]
    async fn test_crud() {

        println!("Starting test");

        let f = get_fixture().unwrap();
        let r = insert_group(&f);
        let group_before = block_on(get_group(&f.group_id)).unwrap();
        let num_codes = group_before.qrcodes.len();

        for code in group_before.qrcodes.iter().step_by(2) {
            let codedb: QrCodeDB = QrCodeDB::from(code.clone());
            println!("delete {:?}", codedb);
            block_on(delete(&codedb.get_primary_key()));
        }

        let group_after = block_on(get_group(&f.group_id)).unwrap();
        let num_codes_after = group_after.qrcodes.len();

        println!("{}", num_codes);
        println!("{}", num_codes_after);

        assert!(num_codes > num_codes_after);
    //}

    //#[test]
    //fn put_and_get() {
        //let f = get_fixture().unwrap();
        let r2 = insert_group(&f);
        assert!(block_on(get_group(&f.group_id)).is_some());
    //}

    //#[test]
    //fn update_db() {
        //let f = get_fixture().unwrap();
        let r3 = insert_group(&f);
        let qrg = block_on(get_group(&f.group_id)).unwrap();
        let code_before = qrg.qrcodes.first().unwrap();

        let mut code_after = code_before.clone();
        let after_title = String::from("New title");
        code_after.title = Some(after_title.clone());

        let qrCodeDB: QrCodeDB = QrCodeDB::from(code_after);
        block_on(update(&qrCodeDB));

        let qrg_after = block_on(get_group(&f.group_id)).unwrap();
        let code_after_get = qrg_after.qrcodes.first().unwrap();

        assert_ne!(code_before.title, Some(after_title.clone()));
        assert_eq!(Some(after_title), code_after_get.title);
    }
}