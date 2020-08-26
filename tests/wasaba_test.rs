#[cfg(test)]
mod tests {
    use qrstore::config::init_env;
use qrstore::dynamodb::delete_images;
use qrstore::storage::count_objects;
use qrstore::dynamodb::qruuid::gen_uuid_str;
    use qrstore::fixtures::get_fixture;
    use qrstore::dynamodb::insert_group;

    #[tokio::test]
    async fn test_wasabi() {

        env_logger::init();

        init_env();

        log::debug!("Testing remote storage");

        let mut f = get_fixture().unwrap();
        let gid = gen_uuid_str("Test-insert-uuid");
        f.group_id = gid;
        for mut qr in &mut f.qrcodes {
            qr.group_id = gid;
        };

        assert!(f.qrcodes[0].group_id == gid);

        log::debug!("Created structs");


        let prefix = base64::encode(gid);
        log::warn!("Verify clean start with prefix: {}", prefix);
        let count_before_res = count_objects(prefix.clone()).await;
        let count_before = count_before_res.ok().unwrap();
        assert!(count_before == 0);

        log::debug!("Inserting group with images");
        assert!(insert_group(&f).await.is_ok());

        
        log::debug!("Verify insert with prefx: {:?}", prefix);
        let count_after_res = count_objects(prefix.clone()).await;
        let count_after = count_after_res.ok().unwrap();
        assert!(count_after > 0);

        log::debug!("Removing inserted images");
        for q in f.qrcodes.into_iter() { 
            delete_images(&mut q.clone()).await;
        }

        log::debug!("Verify final state");
        let count_final_res = count_objects(prefix.clone()).await;
        let count_final = count_final_res.ok().unwrap();
        assert!(count_final == 0);    

        log::debug!("Ending test");    
        

    }
}