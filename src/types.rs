use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct SecretString(pub String);
impl SecretString {
    pub fn new(v: impl Into<String>) -> Self {
        Self(v.into())
    }
    pub fn expose(&self) -> &str {
        &self.0
    }
    pub fn redacted(&self) -> String {
        redact_secret(&self.0)
    }
}
impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<redacted>")
    }
}
impl fmt::Display for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.redacted())
    }
}

pub fn redact_secret(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    if s.len() <= 6 {
        return "<redacted>".into();
    }
    format!("{}…{}", &s[..4], &s[s.len() - 4..])
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RequestStatus {
    Pending,
    Forwarded,
    Judging,
    Approved,
    Disapproved,
    Retrying,
    HumanReview,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestEvent {
    pub id: Uuid,
    pub endpoint_name: String,
    pub model: Option<String>,
    pub profile: String,
    pub status: RequestStatus,
    pub retry_attempt: u32,
    pub latency_ms: u128,
    pub timestamp: DateTime<Utc>,
    pub request_body: serde_json::Value,
    pub candidate_response: Option<serde_json::Value>,
    pub final_response: Option<serde_json::Value>,
    pub judge_verdict: Option<crate::judge::schema::JudgeVerdict>,
}
impl RequestEvent {
    pub fn new(endpoint_name: String, profile: String, body: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            endpoint_name,
            model: None,
            profile,
            status: RequestStatus::Pending,
            retry_attempt: 0,
            latency_ms: 0,
            timestamp: Utc::now(),
            request_body: body,
            candidate_response: None,
            final_response: None,
            judge_verdict: None,
        }
    }
}
