use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelKey {
    pub cluster: String,
    pub key: String,
}
