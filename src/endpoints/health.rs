use crate::config::EndpointConfig;
use anyhow::Result;
pub async fn test_endpoint(ep: &EndpointConfig) -> Result<()> {
    let base = ep.upstream_url.trim_end_matches('/');
    let candidates = [format!("{base}/models"), format!("{base}/v1/models")];
    let client = reqwest::Client::new();
    let mut last = None;
    for url in candidates {
        let mut req = client.get(&url);
        if let Some(auth) = ep.resolved_auth_header()? {
            req = req.header("Authorization", auth);
        }
        match req.send().await.and_then(|r| r.error_for_status()) {
            Ok(_) => return Ok(()),
            Err(e) => last = Some(e),
        }
    }
    Err(last.expect("at least one endpoint health candidate"))?
}
