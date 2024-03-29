use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum VideoQualityMode {
    Auto = 1,
    Full = 2,
}
