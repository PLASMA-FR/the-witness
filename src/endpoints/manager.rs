use crate::config::{EndpointConfig, WitnessConfig};
use anyhow::{bail, Result};
pub fn add_endpoint(cfg: &mut WitnessConfig, ep: EndpointConfig) -> Result<()> {
    if ep.name.trim().is_empty() {
        bail!("endpoint name is required")
    }
    if !ep.upstream_url.starts_with("http") {
        bail!("upstream_url must start with http")
    }
    cfg.add_or_replace_endpoint(ep);
    Ok(())
}
pub fn set_enabled(cfg: &mut WitnessConfig, name: &str, enabled: bool) -> Result<()> {
    let ep = cfg
        .endpoints
        .iter_mut()
        .find(|e| e.name == name)
        .ok_or_else(|| anyhow::anyhow!("endpoint not found: {name}"))?;
    ep.enabled = enabled;
    Ok(())
}
