use serde::{Serialize, Deserialize};
use crate::common::cache_strategy::cache_config::Seconds;

#[derive(Deserialize, Serialize)]
pub struct SetValueDto {
    pub key: String,
    pub value: String,
    pub ttl: Seconds,
}
