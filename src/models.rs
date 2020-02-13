use chrono::NaiveDateTime;

#[cfg(test)]
use diesel::debug_query;
use diesel::insert_into;

use crate::schema::qrcodes;
use crate::establish_connection;

pub type IQr = i64;
type SqliteTime = NaiveDateTime;

#[derive(Insertable)]
#[table_name = "qrcodes"]
pub struct Qrform<'a> {
    pub qr: IQr,
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub images: Option<&'a str>,
}

#[derive(Queryable, Identifiable, AsChangeset, PartialEq, Debug)]
#[table_name = "qrcodes"]
pub struct QrEntry {
    pub id: i32,
    pub qr: IQr,
    pub title: Option<String>,
    pub body: Option<String>,
    pub images: Option<String>,
    pub created_at: SqliteTime,
    pub updated_at: SqliteTime,
}

use rand::Rng;
//use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
pub fn get(entry_id: IQr) -> Option<QrEntry> {
    use crate::schema::qrcodes::dsl::*;
    use crate::diesel::RunQueryDsl;
    use crate::diesel::ExpressionMethods;
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;

    let connection = establish_connection();
    let mut result = qrcodes
        .filter(qr.eq(entry_id))
        .load::<QrEntry>(&connection)
        .expect("Error loading posts");

    match result.len() {
        0 => None,
        _ => Some(result.remove(0)),
    }
}

#[allow(dead_code)]
pub fn add<'a>(titlein: Option<&'a str>, bodyin: Option<&'a str>, imagesin: Option<&'a str>) -> Result<usize, diesel::result::Error> {
    use crate::schema::qrcodes::dsl::*;
    use crate::diesel::RunQueryDsl;

    let mut rng = rand::thread_rng();
    let qrid = rng.gen::<IQr>();

    //let start = SystemTime::now();
    //let unixtime = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let entry: Qrform = Qrform { qr: qrid, title: titlein, body: bodyin, images: imagesin};

    let connection = establish_connection();
    insert_into(qrcodes).values(&entry).execute(&connection)
}

#[allow(dead_code)]
pub fn delete(entry_id: IQr) {
    use crate::schema::qrcodes::dsl::*;
    use crate::diesel::RunQueryDsl;
    use crate::diesel::ExpressionMethods;
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;

    let connection = establish_connection();
    let _num_deleted = diesel::delete(qrcodes.filter(qr.eq(entry_id)))
        .execute(&connection)
        .expect("Error deleting entry");
}

#[allow(dead_code)]
fn update(qrcode: QrEntry) -> Result<usize, diesel::result::Error> {
    use crate::diesel::RunQueryDsl;

    let connection = establish_connection();
    //diesel::update(qrcodes::table).set(&qrcode)
    //qrcode.save_changes(&connection);
    diesel::update(&qrcode).set(&qrcode).execute(&connection)
}
