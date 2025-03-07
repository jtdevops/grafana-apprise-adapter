use serde::Serialize;
use std::env;
use url::{ParseError, Url};

use crate::grafana::{GrafanaPayload, GrafanaState};

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AppriseState {
    Info,
    Success,
    Warning,
    Failure,
}

impl From<GrafanaState> for AppriseState {
    fn from(gf_state: GrafanaState) -> AppriseState {
        match gf_state {
            GrafanaState::Ok => AppriseState::Success,
            GrafanaState::Paused => AppriseState::Info,
            GrafanaState::Alerting => AppriseState::Failure,
            GrafanaState::Pending => AppriseState::Info,
            GrafanaState::NoData => AppriseState::Warning,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ApprisePayload {
    pub title: String,
    pub body: String,

    #[serde(rename = "type")]
    pub notification_type: AppriseState,
}

impl From<GrafanaPayload> for ApprisePayload {
    fn from(gf_payload: GrafanaPayload) -> ApprisePayload {
        ApprisePayload {
            title: gf_payload.title,
            body: gf_payload.message,
            notification_type: AppriseState::from(gf_payload.state),
        }
    }
}

pub fn get_apprise_notify_url(host: &Url, key: &str) -> Result<Url, ParseError> {
    host.join(&format!("/notify/{}", key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::set_var;
    use url::Url;

    #[test]
    fn test_get_apprise_notify_url() {
        let apprise_url = Url::parse("http://apprise:8080").unwrap();
        assert_eq!(
            get_apprise_notify_url(&apprise_url, "foo")
                .unwrap()
                .as_str(),
            "http://apprise:8080/notify/foo"
        );
    }
}
