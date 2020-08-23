use dynomite::AttributeValue;
use dynomite::Attribute;
use dynomite::AttributeError;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct QrItem {
    pub name: String,
}

impl Attribute for QrItem {
    fn into_attr(self: Self) -> AttributeValue {
        AttributeValue {
            s: Some(self.name),
            ..AttributeValue::default()
        }
    }

    fn from_attr(value: AttributeValue) -> Result<Self, AttributeError> {
        value.s.map(|s| QrItem{name: s}).ok_or(AttributeError::InvalidType)
    }
}