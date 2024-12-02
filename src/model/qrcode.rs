use crate::model::qrimage::QrImage;
use crate::model::qritem::QrItem;
use crate::model::schema::QrVersion;

use super::schema::QrCodeId;
use super::schema::QrGroupId;

pub const VERSION: QrVersion = 1;

#[derive(Default, Debug, Clone)]
pub struct QrCode {
	pub version: QrVersion,
    pub group_id: QrGroupId,
    pub id: QrCodeId,
    pub title: Option<String>,
    pub location: Option<String>,
    pub images: Vec<QrImage>,
    pub items: Vec<QrItem>,
	pub content: Option<String>,
    pub attachment: Option<Vec<u8>>,
}
