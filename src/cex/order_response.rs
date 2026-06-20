use crate::cex::tag::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub tag: Tag,
}
