pub fn endpoint_slug(name: &str) -> String {
    name.to_lowercase().replace(' ', "-")
}
