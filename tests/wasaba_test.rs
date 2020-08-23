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
    async fn test_wasabi() {

    }
}