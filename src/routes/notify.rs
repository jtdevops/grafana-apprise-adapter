use actix_web::client::Client;
use actix_web::http::header;
use actix_web::{web, HttpRequest, HttpResponse};
use log::{info, error};
use serde_json;
use serde::Serialize;

use crate::apprise;
use crate::grafana::GrafanaPayload;
use crate::state::AppState;

#[derive(Serialize)]
struct AppriseRequest {
    // Required field
    body: String,
    
    // Optional fields
    #[serde(skip_serializing_if = "String::is_empty")]
    title: String,
    
    // type defaults to "info" if not specified, so we'll always include it
    #[serde(rename = "type")]
    notification_type: String,
    
    #[serde(skip_serializing_if = "String::is_empty")]
    tag: String,
}

// Add a struct for query parameters
#[derive(serde::Deserialize)]
pub struct NotifyQueryParams {
    pub tag: Option<String>,
}

pub async fn notify(
    data: web::Json<GrafanaPayload>,
    key: web::Path<String>,
    state: web::Data<AppState>,
    query: web::Query<NotifyQueryParams>,
    req: HttpRequest,
) -> HttpResponse {
    info!("Received request to path: {}", req.path());
    info!("Received Grafana webhook - Title: '{}', State: {:?}, Message: '{}'", 
          data.title, data.state, data.message);
    
    let payload = apprise::ApprisePayload::from(data.into_inner());

    // Use query parameter tag if provided, otherwise use environment variable
    let tag = query.tag.clone().unwrap_or_else(|| state.apprise_tags.clone());

    // Convert notification type to string
    let notification_type = match payload.notification_type {
        apprise::AppriseState::Info => "info",
        apprise::AppriseState::Success => "success",
        apprise::AppriseState::Warning => "warning",
        apprise::AppriseState::Failure => "failure",
    }.to_string();

    // Create the form request payload
    let apprise_request = AppriseRequest {
        body: payload.body,
        title: payload.title,
        notification_type,
        tag,
    };

    let client = Client::default();
    let apprise_url = match apprise::get_apprise_notify_url(&state.apprise_url, &key) {
        Ok(url) => url,
        Err(e) => {
            error!("Failed to construct Apprise URL: {}", e);
            return HttpResponse::BadRequest().finish();
        }
    };

    info!("");
    info!("Using Apprise endpoint: {}", apprise_url);
    
    // Log the payload that will be sent to Apprise
    if let Ok(payload_json) = serde_json::to_string_pretty(&apprise_request) {
        info!("Sending payload to Apprise:\n{}", payload_json);
    }
    
    let authorization_header = req.headers().get(header::AUTHORIZATION);
    if let Some(_) = authorization_header {
        info!("Forwarding authorization header to Apprise");
    }

    let mut request = client.post(apprise_url.as_str())
        .header(header::CONTENT_TYPE, "application/json");
        
    if let Some(auth_header) = authorization_header {
        request = request.set_header(header::AUTHORIZATION, auth_header.clone());
    }

    return match request.send_json(&apprise_request).await {
        Ok(response) => {
            info!("Apprise response status: {}", response.status());
            HttpResponse::new(response.status())
        },
        Err(e) => {
            error!("Failed to send notification to Apprise: {}", e);
            HttpResponse::BadGateway().finish()
        }
    };
}
