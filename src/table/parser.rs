use anyhow::{Ok, Result, bail};
use bytes::Bytes;
use chrono::DateTime;
use reqwest::StatusCode;
use reqwest::blocking::{Client, get};
use reqwest::header::{ETAG, IF_MODIFIED_SINCE, IF_NONE_MATCH, LAST_MODIFIED};
use scraper::{Html, Selector};
use serde_json::from_slice;
use url::Url;

use crate::model::table::{Data, Header};

pub fn get_header_url(url: &str) -> Result<Url> {
    let url = Url::parse(url)?;

    if url.path().ends_with("header.json") {
        return Ok(url);
    }

    let client = Client::builder().user_agent("Mozilla/5.0").build()?;

    let body = client.get(url.as_ref()).send()?.text()?;
    let document = Html::parse_document(&body);

    let selector = Selector::parse(r#"meta[name="bmstable"]"#).unwrap();

    let Some(element) = document.select(&selector).next() else {
        bail!("bmstable meta tag not found");
    };

    let Some(content) = element.value().attr("content") else {
        bail!("bmstable meta tag has no content");
    };

    Ok(url.join(content)?)
}

pub fn get_header(url: &str) -> Result<Bytes> {
    let res = get(url)?;

    Ok(res.bytes()?)
}

pub fn parse_header(bytes: &[u8]) -> Result<Header> {
    let header: Header = from_slice(bytes)?;

    Ok(header)
}

pub fn get_data(
    url: &str,
    etag: Option<&str>,
    last_modified: Option<i64>,
) -> Result<(Option<Bytes>, Option<String>, Option<i64>)> {
    let client = Client::new();
    let mut req = client.get(url);

    if let Some(etag) = etag {
        req = req.header(IF_NONE_MATCH, etag);
    } else if let Some(last_modified) = last_modified
        && let Some(dt) = DateTime::from_timestamp(last_modified, 0)
    {
        req = req.header(IF_MODIFIED_SINCE, dt.to_rfc2822());
    }

    let res = req.send()?;

    let new_etag = res
        .headers()
        .get(ETAG)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let new_last_modified = res
        .headers()
        .get(LAST_MODIFIED)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| DateTime::parse_from_rfc2822(v).ok())
        .map(|v| v.timestamp());

    if res.status() == StatusCode::NOT_MODIFIED {
        return Ok((
            None,
            new_etag.or_else(|| etag.map(String::from)),
            new_last_modified.or(last_modified),
        ));
    }

    let res = res.error_for_status()?;

    Ok((Some(res.bytes()?), new_etag, new_last_modified))
}

pub fn parse_data(bytes: &[u8]) -> Result<Vec<Data>> {
    let data: Vec<Data> = from_slice(bytes)?;

    Ok(data)
}
