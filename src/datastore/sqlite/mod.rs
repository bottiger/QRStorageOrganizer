use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::{Queryable, Insertable, RunQueryDsl};
use firestore::async_trait;
use schema::groups;
use std::error::Error;
use std::convert::TryInto;

use crate::datastore::datastore::DataStore;
use crate::model::qrgroup::QrGroup;
use crate::model::qrcode::{QrCode, VERSION};
use crate::model::qruuid::lowest_32_bits;
use crate::model::schema::{QrCodeId, QrGroupId};

pub mod schema;

use crate::datastore::sqlite::schema::groups::dsl as groups_dsl;
use crate::datastore::sqlite::schema::qrcode::dsl as qrcode_dsl;

#[derive(Queryable, QueryableByName)]
#[diesel(table_name = groups)]
pub struct QrGroupSqlite {
    pub id: i32,
    pub name: String,
    pub qr_salt: Vec<u8>,
    pub qr_count: i32,
}

#[derive(Queryable, QueryableByName)]
#[diesel(table_name = crate::datastore::sqlite::schema::qrcode)]
pub struct QrCodeSqlite {
    pub id: i32,
    pub group_id: i32,
    pub title: Option<String>,
    pub location: Option<String>,
    pub content: Option<String>,
    pub attachment: Option<Vec<u8>>,
    pub version: i32,
}

pub struct SQLiteDataStore {
    conn: SqliteConnection,
}

fn u64_to_bytes(value: u64) -> [u8; 12] {
    let mut bytes = value.to_le_bytes().to_vec(); // or .to_be_bytes() for big-endian
    bytes.resize(12, 0); // Ensure the length is 12 bytes
    let mut array = [0u8; 12];
    array.copy_from_slice(&bytes);
    array
}

#[async_trait]
impl DataStore for SQLiteDataStore {

    async fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
        
        let mut conn = SqliteConnection::establish(db_path)
            .map_err(|e| format!("Failed to connect to database: {}", e))?;
        

        // Create tables if they do not exist
        diesel::sql_query(
            r#"
            CREATE TABLE IF NOT EXISTS groups (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                qr_salt BLOB NOT NULL,
                qr_count INTEGER NOT NULL
            );
            "#,
        )
        .execute(&mut conn)
        .map_err(|e| format!("Failed to create groups tables: {}", e))?;

        diesel::sql_query(
            r#"
            CREATE TABLE IF NOT EXISTS qrcode (
                id INTEGER PRIMARY KEY,
                group_id INTEGER NOT NULL,
                title TEXT,
                location TEXT,
                content TEXT,
                attachment BLOB,
                version INTEGER NOT NULL,
                FOREIGN KEY (group_id) REFERENCES groups(id)
            );
            "#,
        )
        .execute(&mut conn)
        .map_err(|e| format!("Failed to create qrcode tables: {}", e))?;
    

        Ok(Self { conn })
    }

    async fn delete_group(&mut self, group_id_in: &QrGroupId) -> Result<(), Box<dyn Error>> {
        let id_remove = lowest_32_bits(group_id_in);

        diesel::delete(qrcode_dsl::qrcode.filter(qrcode_dsl::group_id.eq(id_remove))).execute(&mut self.conn)?;
        diesel::delete(groups_dsl::groups.filter(groups_dsl::id.eq(id_remove))).execute(&mut self.conn)?;

        Ok(())
    }

    async fn create_group(&mut self, group: &QrGroup) -> Result<(), Box<dyn Error>> {
    
        let group_id_32 = lowest_32_bits(&group.group_id);

        diesel::insert_into(groups_dsl::groups)
            .values((
                groups_dsl::id.eq(group_id_32),
                groups_dsl::name.eq(&group.name),
                groups_dsl::qr_salt.eq(&group.qr_salt),
                groups_dsl::qr_count.eq(group.qr_count as i32),
            ))
            .execute(&mut self.conn)?;

        for q in &group.qrcodes {
            self.create_qrcode(&group.group_id, q).await?;
        };

        Ok(())
    }

    async fn create_qrcode(&mut self, group_id: &QrGroupId, qrcode: &QrCode) -> Result<(), Box<dyn Error>> {

        let group_id_32 = lowest_32_bits(group_id);
       
        let result = diesel::insert_into(qrcode_dsl::qrcode)
        .values((
            qrcode_dsl::id.eq(qrcode.id as i32),
            qrcode_dsl::group_id.eq(group_id_32),
            qrcode_dsl::title.eq(&qrcode.title),
            qrcode_dsl::location.eq(&qrcode.location),
            qrcode_dsl::content.eq(&qrcode.content),
            qrcode_dsl::version.eq(VERSION as i32),
            qrcode_dsl::attachment.eq(qrcode.attachment.as_ref()),
        ))
        .execute(&mut self.conn);

        Ok(())
    }

    async fn get_group(&mut self, group_id_in: &QrGroupId) -> Result<Option<QrGroup>, Box<dyn Error>> {

        let group_id_32 = lowest_32_bits(group_id_in);

        let result: Option<QrGroupSqlite> = groups::table
            .filter(schema::groups::dsl::id.eq(group_id_32))
            .first::<QrGroupSqlite>(&mut self.conn)
            .optional()?;

            if let Some(group) = result {
                let qrcodes = self.get_qrcodes(group_id_in).await?;
                Ok(Some(QrGroup {
                    group_id: group_id_in.clone(),
                    id: group_id_32,
                    name: group.name,
                    qr_salt: group.qr_salt.try_into().unwrap(),
                    qr_count: group.qr_count as u32,
                    qrcodes,
                }))
            } else {
                Ok(None)
            }
    } 

    async fn get_qrcode(&mut self, group_id_in: &QrGroupId, qrcode_id_in: &QrCodeId) -> Result<Option<QrCode>, Box<dyn Error>> {

        let group_id_32 = lowest_32_bits(group_id_in);
    
        let result: Option<QrCodeSqlite> = qrcode_dsl::qrcode
            .filter(qrcode_dsl::group_id.eq(group_id_32))
            .filter(qrcode_dsl::id.eq(*qrcode_id_in as i32))
            .first::<QrCodeSqlite>(&mut self.conn)
            .optional()?;
    
        if let Some(q) = result {
            let group = self.get_group(group_id_in).await?.ok_or("Group not found")?;
            Ok(Some(QrCode {
                id: q.id as u64,
                group_id: group_id_in.clone(),
                title: q.title,
                location: q.location,
                content: q.content,
                attachment: q.attachment,
                version: q.version as u8,
                images: vec![],
                items: vec![],
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_qrcodes(&mut self, group_id_in: &QrGroupId) -> Result<Vec<QrCode>, Box<dyn Error>> {
        use schema::qrcode::dsl::*;

        let result: Vec<QrCodeSqlite> = qrcode
            .filter(group_id.eq(group_id))
            .load::<QrCodeSqlite>(&mut self.conn)?;

        Ok(result.into_iter().map(|q| {
            QrCode {
                id: q.id as u64,
                group_id: group_id_in.clone(),
                title: q.title,
                location: q.location,
                content: q.content,
                attachment: q.attachment,
                version: q.version as u8,
                images: vec![],
                items: vec![],
            }
        }).collect())
    }

    async fn update_group(&mut self, group_id_in: &QrGroupId, group_in: &QrGroup) -> Result<(), Box<dyn Error>> {

        let group_id_32 = lowest_32_bits(group_id_in);

        diesel::update(groups::table.filter(schema::groups::dsl::id.eq(group_id_32)))
            .set(schema::groups::dsl::name.eq(&group_in.name))
            .execute(&mut self.conn)?;

        // Clear existing QR codes for this group
        self.delete_qrcodes(group_id_in)?;

        for qrcode in &group_in.qrcodes {
            self.create_qrcode(group_id_in, qrcode).await?;
        }

        Ok(())
    }

    async fn update_qrcode(&mut self, qrcode_in: &QrCode) -> Result<(), Box<dyn Error>> {

        let group_id_32 = lowest_32_bits(&qrcode_in.group_id);

        diesel::update(qrcode_dsl::qrcode
            .filter(schema::qrcode::dsl::group_id.eq(group_id_32))
            .filter(schema::qrcode::dsl::id.eq(qrcode_in.id as i32)))
            .set((
                schema::qrcode::dsl::title.eq(&qrcode_in.title),
                //schema::qrcode::dsl::quantity.eq(qrcode_in.items.len() as i32),
            ))
            .execute(&mut self.conn)?;

        Ok(())
    }

    async fn delete_qrcode(&mut self, group_id_in: &QrGroupId, qrcode_id_in: &u64) -> Result<(), Box<dyn Error>> {

        let group_id_32 = lowest_32_bits(group_id_in);

        diesel::delete(qrcode_dsl::qrcode
            .filter(schema::qrcode::dsl::group_id.eq(group_id_32))
            .filter(schema::qrcode::dsl::id.eq(*qrcode_id_in as i32)))
            .execute(&mut self.conn)?;

        Ok(())
    }
}

impl SQLiteDataStore {
    fn delete_qrcodes(&mut self, group_id_in: &QrGroupId) -> Result<(), Box<dyn Error>> {

        let group_id_32 = lowest_32_bits(group_id_in);

        diesel::delete(qrcode_dsl::qrcode.filter(schema::qrcode::dsl::group_id.eq(group_id_32))).execute(&mut self.conn)?;
        Ok(())
    }
}
