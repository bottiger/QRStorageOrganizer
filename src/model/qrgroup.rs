use crate::model::qruuid::{slice_to_u256, vec_to_u256}; // Add vec_to_u256 import
use crate::model::qrcode::QrCode;
use crate::model::schema::u256;
use crate::model::schema::u256DB;
use crate::model::schema::DbItem;
use crate::model::schema::DynamoDbType;
use crate::model::schema::DynamoPartitionKey;
use crate::model::schema::DynamoPartitionKeyDB;
use crate::model::schema::DynamoPrimaryKey;
use crate::model::schema::DynamoSearchKey;
use crate::model::schema::slice_to_partition_key;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct QrGroup {
    pub group_id: DynamoPartitionKey,
    pub id: DynamoSearchKey,
    pub qr_salt: u256,
    pub qr_count: u32,
    pub qrcodes: Vec<QrCode>,
}

impl From<QrGroupDB> for QrGroup {
    fn from(item: QrGroupDB) -> Self {
        QrGroup {
            group_id: slice_to_partition_key(&item.group_id).ok().unwrap(),
            id: item.id,
            qr_salt: vec_to_u256(&item.qr_salt).ok().unwrap(),
            qr_count: item.qr_count,
            qrcodes: Vec::new(),
        }
    }
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct QrGroupDB {
    pub group_id: DynamoPartitionKeyDB,
    pub id: DynamoSearchKey,
    pub qr_salt: u256DB,
    pub qr_count: u32,
}

impl From<QrGroup> for QrGroupDB {
    fn from(item: QrGroup) -> Self {
        QrGroupDB {
            group_id: slice_to_partition_key(&item.group_id).ok().unwrap().to_vec(),
            id: item.id,
            qr_salt: slice_to_u256(&item.qr_salt).ok().unwrap().to_vec(),
            qr_count: item.qr_count,
        }
    }
}

impl DbItem for QrGroupDB {
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
        DynamoDbType::QrGroup
    }

    /*
    fn get_attribute_value_map(&self) -> HashMap<String, AttributeValue> {
        let clone = self.clone();
        let mut map: HashMap<String, AttributeValue> = clone.into();
        map.remove("qrcodes");

        map
    }

    fn get_update_expr(&self) -> (Option<HashMap<String, AttributeValue>>, Option<String>) {
        let mut expression_attribute_values = HashMap::new();
        expression_attribute_values
            .insert(":count".to_string(), self.qr_count.to_owned().into_attr());
        /*
        expression_attribute_values.insert(
                ":codes".to_string(),
                self.qrcodes.to_owned().into_attr()
            );
            */

        let attr_vals = Some(expression_attribute_values);
        //let update_expr = Some("SET qr_count = :count, qrcodes = :codes".to_string());
        let update_expr = Some("SET qr_count = :count".to_string());

        (attr_vals, update_expr)
    }
    */
}
