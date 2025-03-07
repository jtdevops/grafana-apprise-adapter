use actix_web::client::Client;
use actix_web::http::header;
use actix_web::{web, HttpRequest, HttpResponse};
use log::{info, error};
use serde_json;

use crate::apprise;
use crate::grafana::GrafanaPayload;
use crate::state::AppState;

pub async fn notify(
    data: web::Json<GrafanaPayload>,
    key: web::Path<String>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> HttpResponse {
    info!("Received request to path: {}", req.path());
    info!("Received Grafana webhook - Title: '{}', State: {:?}, Message: '{}'", 
          data.title, data.state, data.message);
    
    let payload = apprise::ApprisePayload::from(data.into_inner());
    
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
    if let Ok(payload_json) = serde_json::to_string_pretty(&payload) {
        info!("Sending payload to Apprise:\n{}", payload_json);
    }
    
    let authorization_header = req.headers().get(header::AUTHORIZATION);
    if let Some(_auth) = authorization_header {
        info!("Forwarding authorization header to Apprise");
    }

    let mut request = client.post(apprise_url.as_str());
    if let Some(auth_header) = authorization_header {
        request = request.set_header(header::AUTHORIZATION, auth_header.clone());
    }

    return match request.send_json(&payload).await {
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
