use crate::model::schema::slice_to_partition_key;
use crate::dynamodb::qruuid::slice_to_u192;
use crate::dynamodb::qruuid::vec_to_u192;
use crate::model::qrimage::QrImage;
use crate::model::qrimage::QrImageHash;
use crate::model::qritem::QrItem;
use crate::model::schema::DbItem;
use crate::model::schema::QrVersion;
use crate::model::schema::DynamoDbType;
use crate::model::schema::DynamoPartitionKey;
use crate::model::schema::DynamoPartitionKeyDB;
use crate::model::schema::DynamoPrimaryKey;
use crate::model::schema::DynamoSearchKey;
use dynomite::Attribute;
use dynomite::AttributeValue;
use dynomite::Item;
use std::collections::hash_map::HashMap;

pub const VERSION: QrVersion = 1;

#[derive(Default, Debug, Clone)]
pub struct QrCode {
	pub version: QrVersion,
    pub group_id: DynamoPartitionKey,
    pub id: DynamoSearchKey,
    pub title: Option<String>,
    pub location: Option<String>,
    pub images: Vec<QrImage>,
    pub items: Vec<QrItem>,
	pub content: Option<String>,
}


impl From<QrCodeDB> for QrCode {
    fn from(item: QrCodeDB) -> Self {
        QrCode {
            group_id: slice_to_partition_key(&item.group_id).ok().unwrap(),
            id: item.id,
            title: item.title,
            location: item.location,
            items: item.items,
            images: Vec::new(),
			content: item.content,
            version: VERSION,
        }
    }
}

#[derive(Item, Default, PartialEq, Debug, Clone)]
pub struct QrCodeDB {
    #[dynomite(partition_key)]
    #[dynomite(rename = "qr_group_id")] //remote name
    pub group_id: DynamoPartitionKeyDB,
    #[dynomite(sort_key)]
    #[dynomite(rename = "qr_val")] //remote name
    pub id: DynamoSearchKey,
    pub title: Option<String>,
    #[dynomite(rename = "location2")] //remote name
    pub location: Option<String>,
    pub items: Vec<QrItem>,
    pub image_hashes: Vec<QrImageHash>,
	pub content: Option<String>,
}

impl From<QrCode> for QrCodeDB {
    fn from(item: QrCode) -> Self {
        QrCodeDB {
            group_id: slice_to_partition_key(&item.group_id).ok().unwrap().to_vec(),
            id: item.id,
            title: item.title,
            location: item.location,
            items: item.items,
            image_hashes: item.images.into_iter().map(|i| i.hash32).collect(),
			content: item.content,
        }
    }
}

impl DbItem for QrCodeDB {
    fn get_primary_key(&self) -> DynamoPrimaryKey {
        DynamoPrimaryKey {
            partition_key: self.get_partition_key(),
            sort_key: self.get_sort_key(),
        }
    }
    fn get_partition_key(&self) -> DynamoPartitionKey {
        slice_to_partition_key(&self.group_id).ok().unwrap() //self.group_id
    }
    fn get_sort_key(&self) -> DynamoSearchKey {
        self.id.to_owned()
    }

    fn get_type(&self) -> DynamoDbType {
        DynamoDbType::QrCode
    }

    fn get_attribute_value_map(&self) -> HashMap<String, AttributeValue> {
        self.clone().into()
    }

    fn get_update_expr(&self) -> (Option<HashMap<String, AttributeValue>>, Option<String>) {
        let mut expression_attribute_values = HashMap::new();
        expression_attribute_values
            .insert(":title_val".to_string(), self.title.to_owned().into_attr());
        expression_attribute_values.insert(
            ":location_val".to_string(),
            self.location.to_owned().unwrap().into_attr(),
        );

        let attr_vals = Some(expression_attribute_values);
        let update_expr = Some("SET title = :title_val, location2 = :location_val".to_string());

        (attr_vals, update_expr)
    }
}
