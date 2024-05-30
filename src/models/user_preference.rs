use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum UserPreferenceValue {
    Flag(bool),
    Stringy(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPreference {
    pub id: String,
    pub value: UserPreferenceValue,
}
