use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeyDefineType {
    Single,
    Multi,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyDefine {
    /// For single mode: [start_time_ms, duration_ms]
    Single(Vec<u64>),
    /// For multi mode: filename (e.g. "key1.wav") or null
    Multi(Option<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechvibesConfig {
    pub id: String,
    pub name: String,
    pub key_define_type: KeyDefineType,
    pub includes_numpad: bool,
    pub sound: String,
    pub defines: HashMap<String, KeyDefine>,
    #[serde(skip)]
    pub path: PathBuf,
}
