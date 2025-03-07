use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GrafanaState {
    Ok,
    Paused,
    Alerting,
    Pending,
    NoData,
}

#[derive(Deserialize, Debug)]
pub struct GrafanaPayload {
    pub title: String,
    pub message: String,
    pub state: GrafanaState,

    #[serde(rename = "imageUrl", default)]
    pub image: String,

    #[serde(default)]
    pub tags: HashMap<String, String>,
}
