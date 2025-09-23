use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct StatsFile {
    pub stats:        HashMap<String, HashMap<String, u64>>,
    #[serde(rename = "DataVersion")]
    pub data_version: u32,
}
