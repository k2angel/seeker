use std::ffi::OsStr;
use url::Url;

pub fn is_url(value: &str) -> bool {
    matches!(
        Url::parse(value),
        Ok(url) if matches!(url.scheme(), "http" | "https")
    )
}

pub fn is_bms(extention: Option<&OsStr>) -> bool {
    extention.and_then(|e| e.to_str()).is_some_and(|ext| {
        matches!(
            ext.to_ascii_lowercase().as_str(),
            "bms" | "bme" | "bml" | "pms" | "bmw" | "bmson"
        )
    })
}
