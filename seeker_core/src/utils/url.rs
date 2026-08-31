use url::Url;

pub fn is_url(value: &str) -> bool {
    matches!(
        Url::parse(value),
        Ok(url) if matches!(url.scheme(), "http" | "https")
    )
}
