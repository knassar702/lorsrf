use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Proxy};
use std::collections::HashMap;
use std::time::Duration;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use url::Url;

#[derive(Debug)]
pub struct Requester {
    pub timeout: u64,
    pub proxy: Option<String>,
    pub headers: HeaderMap<HeaderValue>,
}

impl Requester {
    pub async fn build(&self) -> Client {
        let client_builder = Client::builder()
            .timeout(Duration::from_secs(self.timeout))
            .danger_accept_invalid_certs(true);

        let client_builder = if let Some(ref proxy_url) = self.proxy {
            client_builder.proxy(Proxy::all(proxy_url).unwrap())
        } else {
            client_builder
        };

        let client_builder = if !self.headers.is_empty() {
            client_builder.default_headers(self.headers.clone())
        } else {
            client_builder
        };

        client_builder.build().unwrap()
    }
}

pub fn extract_headers(headers: &str) -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let mut header_map = HeaderMap::new();

    for line in headers.lines() {
        if let Some((name, value)) = line.split_once(':') {
            let header_name = HeaderName::from_bytes(name.trim().as_bytes())?;
            let header_value = HeaderValue::from_str(value.trim())?;
            header_map.insert(header_name, header_value);
        }
    }

    Ok(header_map)
}

pub fn convert_vec(wordlist: BufReader<File>) -> Vec<String> {
    wordlist.lines().filter_map(|line| line.ok()).collect()
}

pub fn add_parameters(url: String, payload: &str, wordlist: Vec<String>) -> Vec<String> {
    let mut scheme = vec![];
    let mut urls = Vec::new();
    let url_path = Url::parse(&url).unwrap();

    if wordlist.len() == 1 {
        let mut params = vec![];
        for (key, value) in url_path.query_pairs() {
            drop(value);
            params.push((
                key.clone(),
                payload
                    .replace("%PARAM%", &key)
                    .replace("%PATH%", url_path.path())
                    .replace("%QUERY%", url_path.query().unwrap_or(""))
                    .replace("%HOST%", url_path.host_str().unwrap_or("")),
            ));
        }
        urls.push(
            Url::parse_with_params(url.split_once('?').unwrap_or((&url, "")).0, &params)
                .unwrap()
                .to_string(),
        );
    } else {
        for theurl in wordlist {
            scheme.push((
                theurl.clone(),
                payload
                    .replace("%PARAM%", &theurl)
                    .replace("%PATH%", url_path.path())
                    .replace("%QUERY%", url_path.query().unwrap_or(""))
                    .replace("%HOST%", url_path.host_str().unwrap_or("")),
            ));

            if scheme.len() == 10 {
                urls.push(Url::parse_with_params(&url, &scheme).unwrap().to_string());
                scheme.clear();
            }
        }
    }

    urls
}

pub fn query(url: &str) -> HashMap<String, String> {
    Url::parse(url)
        .unwrap()
        .query_pairs()
        .into_owned()
        .collect()
}
