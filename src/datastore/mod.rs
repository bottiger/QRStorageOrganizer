
pub mod sqlite;
pub mod firebase;


pub mod datastore {
    use std::error::Error;
    use firestore::async_trait;

    use crate::model::qrgroup::QrGroup;
    use crate::model::qrcode::QrCode;
    use crate::model::schema::{QrCodeId, QrGroupId};

    #[async_trait]
    pub trait DataStore: Sized {

        async fn new(db_path: &str) -> Result<Self, Box<dyn Error>>;

        async fn create_group(&mut self, group: &QrGroup) -> Result<(), Box<dyn Error>>;
        async fn get_group(&mut self, id: &QrGroupId) -> Result<Option<QrGroup>, Box<dyn Error>>;
        async fn update_group(&mut self, id: &QrGroupId, group: &QrGroup) -> Result<(), Box<dyn Error>>;
        async fn delete_group(&mut self, id: &QrGroupId) -> Result<(), Box<dyn Error>>;

        async fn create_qrcode(&mut self, group_id: &QrGroupId, item: &QrCode) -> Result<(), Box<dyn Error>>;
        async fn get_qrcode(
            &mut self,
            group_id: &QrGroupId,
            qrcode_id: &QrCodeId,
        ) -> Result<Option<QrCode>, Box<dyn Error>>;

        async fn get_qrcodes(&mut self, group_id: &QrGroupId) -> Result<Vec<QrCode>, Box<dyn Error>>;

        async fn update_qrcode(
            &mut self,
            item: &QrCode,
        ) -> Result<(), Box<dyn Error>>;
        
        async fn delete_qrcode(&mut self, group_id: &QrGroupId, qrcode_id: &QrCodeId) -> Result<(), Box<dyn Error>>;
    }
}