use firestore::*;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::error::Error;

use crate::datastore::datastore::DataStore;
use crate::model::qrgroup::QrGroup;
use crate::model::qrcode::{QrCode, VERSION};
use crate::model::schema::{QrCodeId, QrGroupId};
use crate::model::qruuid::{bytes_to_i32, encode_binary};

#[derive(Debug, Serialize, Deserialize)]
pub struct QrGroupFirestore {
    pub id: String,
    pub name: String,
    pub qr_salt: Vec<u8>,
    pub qr_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QrCodeFirestore {
    pub id: String,
    pub group_id: String,
    pub title: Option<String>,
    pub location: Option<String>,
    pub content: Option<String>,
    pub attachment: Option<Vec<u8>>,
    pub version: i32,
}

pub struct FirestoreDataStore {
    db: FirestoreDb,
}

#[async_trait]
impl DataStore for FirestoreDataStore {
    async fn new(project_id: &str) -> Result<Self, Box<dyn Error>> {
        let db = FirestoreDb::new(project_id).await?;
        Ok(Self { db })
    }

    async fn create_group(&mut self, group: &QrGroup) -> Result<(), Box<dyn Error>> {
        let group_id = encode_binary(&group.group_id);
        let group_firestore = QrGroupFirestore {
            id: group_id.clone(),
            name: group.name.clone(),
            qr_salt: group.qr_salt.to_vec(),
            qr_count: group.qr_count as i32,
        };

        self.db
            .fluent()
            .insert()
            .into("groups")
            .document_id(&group_id)
            .object(&group_firestore)
            .execute()
            .await?;

        for qrcode in &group.qrcodes {
            self.create_qrcode(&group.group_id, qrcode).await?;
        }
        Ok(())
    }

    async fn create_qrcode(&mut self, group_id: &QrGroupId, qrcode: &QrCode) -> Result<(), Box<dyn Error>> {
        let qrcode_firestore = QrCodeFirestore {
            id: qrcode.id.to_string(),
            group_id: encode_binary(group_id),
            title: qrcode.title.clone(),
            location: qrcode.location.clone(),
            content: qrcode.content.clone(),
            attachment: qrcode.attachment.clone(),
            version: VERSION as i32,
        };

        self.db
            .fluent()
            .insert()
            .into("qrcodes")
            .document_id(&qrcode_firestore.id)
            .object(&qrcode_firestore)
            .execute()
            .await?;
        Ok(())
    }

    async fn get_group(&mut self, group_id: &QrGroupId) -> Result<Option<QrGroup>, Box<dyn Error>> {
        let group_doc: Option<QrGroupFirestore> = self
            .db
            .fluent()
            .select()
            .by_id_in("groups")
            .obj()
            .one(encode_binary(group_id))
            .await?;

        if let Some(group) = group_doc {
            let qrcodes = self.get_qrcodes(group_id).await?;
            Ok(Some(QrGroup {
                group_id: group_id.clone(),
                id: bytes_to_i32(group_id),
                name: group.name,
                qr_salt: group.qr_salt.try_into().unwrap(),
                qr_count: group.qr_count as u32,
                qrcodes,
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_qrcode(&mut self, group_id: &QrGroupId, qrcode_id: &QrCodeId) -> Result<Option<QrCode>, Box<dyn Error>> {
        let qrcode_id_str = qrcode_id.to_string();

        // Fetch the QR code document by its ID
        let qrcode_doc: Option<QrCodeFirestore> = self
            .db
            .fluent()
            .select()
            .by_id_in("qrcodes")
            .obj()
            .one(qrcode_id_str.clone())
            .await?;

        if let Some(qrcode) = qrcode_doc {
            Ok(Some(QrCode {
                id: qrcode.id.parse().unwrap_or_default(),
                group_id: group_id.clone(),
                title: qrcode.title,
                location: qrcode.location,
                content: qrcode.content,
                attachment: qrcode.attachment,
                version: qrcode.version as u8,
                images: vec![],
                items: vec![],
            }))
        } else {
            Ok(None)
        }
    }


    async fn get_qrcodes(&mut self, group_id: &QrGroupId) -> Result<Vec<QrCode>, Box<dyn Error>> {
        let group_id_str = encode_binary(group_id);
        let qrcodes: Vec<QrCodeFirestore> = self
            .db
            .fluent()
            .select()
            .from("qrcodes")
            .filter(|q| q.field("group_id").equal(group_id_str.clone()))
            .obj()
            .query()
            .await?;

        Ok(qrcodes
            .into_iter()
            .map(|q| QrCode {
                id: q.id.parse().unwrap_or_default(),
                group_id: group_id.clone(),
                title: q.title,
                location: q.location,
                content: q.content,
                attachment: q.attachment,
                version: q.version as u8,
                images: vec![],
                items: vec![],
            })
            .collect())
    }

    async fn update_group(&mut self, group_id: &QrGroupId, group: &QrGroup) -> Result<(), Box<dyn Error>> {
        let group_id_str = encode_binary(group_id);
        let group_firestore = QrGroupFirestore {
            id: group_id_str.clone(),
            name: group.name.clone(),
            qr_salt: group.qr_salt.to_vec(),
            qr_count: group.qr_count as i32,
        };

        self.db
            .fluent()
            .update()
            .in_col("groups")
            .document_id(&group_id_str)
            .object(&group_firestore)
            .execute()
            .await?;

        self.delete_qrcodes(group_id).await?;
        for qrcode in &group.qrcodes {
            self.create_qrcode(group_id, qrcode).await?;
        }
        Ok(())
    }

    async fn update_qrcode(&mut self, qrcode: &QrCode) -> Result<(), Box<dyn Error>> {
        let qrcode_id = qrcode.id.to_string();
        let qrcode_firestore = QrCodeFirestore {
            id: qrcode_id.clone(),
            group_id: encode_binary(&qrcode.group_id),
            title: qrcode.title.clone(),
            location: qrcode.location.clone(),
            content: qrcode.content.clone(),
            attachment: qrcode.attachment.clone(),
            version: qrcode.version as i32,
        };

        self.db
            .fluent()
            .update()
            .in_col("qrcodes")
            .document_id(&qrcode_id)
            .object(&qrcode_firestore)
            .execute()
            .await?;
        Ok(())
    }

    async fn delete_group(&mut self, group_id: &QrGroupId) -> Result<(), Box<dyn Error>> {
        let group_id_str = encode_binary(group_id);

        self.delete_qrcodes(group_id).await?;
        self.db
            .fluent()
            .delete()
            .from("groups")
            .document_id(&group_id_str)
            .execute()
            .await?;
        Ok(())
    }

    async fn delete_qrcode(&mut self, group_id: &QrGroupId, qrcode_id: &QrCodeId) -> Result<(), Box<dyn Error>> {
        let qrcode_id_str = qrcode_id.to_string();

        self.db
            .fluent()
            .delete()
            .from("qrcodes")
            .document_id(&qrcode_id_str)
            .execute()
            .await?;
        Ok(())
    }
}

impl FirestoreDataStore {
    async fn delete_qrcodes(&mut self, group_id: &QrGroupId) -> Result<(), Box<dyn Error>> {
        let group_id_str = encode_binary(group_id);

        let qrcodes: Vec<String> = self
            .db
            .fluent()
            .select()
            .from("qrcodes")
            .filter(|q| q.field("group_id").equal(group_id_str.clone()))
            .obj()
            .query()
            .await?
            .into_iter()
            .map(|doc: QrCodeFirestore| doc.id)
            .collect();

        for qrcode_id in qrcodes {
            self.db
                .fluent()
                .delete()
                .from("qrcodes")
                .document_id(&qrcode_id)
                .execute()
                .await?;
        }
        Ok(())
    }
}
