use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum DaemonMessage {
    #[serde(rename = "request")]
    Request(DaemonRequest),
    #[serde(rename = "response")]
    Response(DaemonResponse),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DaemonRequest {
    pub protocol: u8,
    pub request_id: Option<String>,
    pub command: String,
    #[serde(default)]
    pub args: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DaemonResponse {
    pub protocol: u8,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub request_id: Option<String>,
    pub status: ResponseStatus,
    pub data: Option<Value>,
    pub error: Option<DaemonError>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Ok,
    Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DaemonError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

impl DaemonResponse {
    pub fn ok(request: &DaemonRequest, data: impl Serialize) -> Self {
        DaemonResponse {
            protocol: request.protocol,
            msg_type: "response".to_string(),
            request_id: request.request_id.clone(),
            status: ResponseStatus::Ok,
            data: Some(serde_json::to_value(data).unwrap_or(Value::Null)),
            error: None,
        }
    }

    pub fn error(
        request: Option<&DaemonRequest>,
        code: &str,
        message: &str,
        details: Option<Value>,
    ) -> Self {
        let protocol = request.map(|r| r.protocol).unwrap_or(1);
        let request_id = request.and_then(|r| r.request_id.clone());

        DaemonResponse {
            protocol,
            msg_type: "response".to_string(),
            request_id,
            status: ResponseStatus::Error,
            data: None,
            error: Some(DaemonError {
                code: code.to_string(),
                message: message.to_string(),
                details,
            }),
        }
    }
}
