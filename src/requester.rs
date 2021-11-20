use isahc::{HttpClient, config::{
        RedirectPolicy, 
        VersionNegotiation,
        SslOption}, prelude::*};
use url::{Url};
use std::collections::HashMap;
use regex::Regex;
use std::{
    fs::File,
    time::Duration,
    io::{BufRead,BufReader},
};

pub struct Requester {
    pub timeout: u64,
    pub proxy: String,
    pub headers: HashMap<String,String>,
}

impl Requester {

    pub fn build(&self) -> HttpClient {
        let mut client = HttpClient::builder()
                .version_negotiation(VersionNegotiation::http11())
                .redirect_policy(RedirectPolicy::None)
                .timeout(Duration::from_secs(self.timeout))               
                .ssl_options(SslOption::DANGER_ACCEPT_INVALID_CERTS | SslOption::DANGER_ACCEPT_REVOKED_CERTS);
        
        if self.proxy != "" {
            client = client.proxy(Some(self.proxy.parse().unwrap()));
        }

        if self.headers.len() > 0 {
            client = client.default_headers(self.headers.clone());
        } 

        return client.build().unwrap()
    }

}

pub fn extractheaders(headers: &str ) -> HashMap<String, String> {
    let headers_finder = Regex::new(r"(.*):\s(.*)").unwrap();
    let mut headers_found = HashMap::new();
    for cap in headers_finder.captures_iter(str::replace(headers, "\\n", "\n").as_str()) {
        headers_found.insert(cap[1].to_string(),cap[2].to_string());
    };
    return headers_found;
}
pub fn add_parameters(url : &str, payload: &str , wordlist: BufReader<File> ) -> Vec<String> {
    let mut scheme = vec![];
    let mut urls = Vec::new();
    for theurl in wordlist.lines() {
        scheme.push((theurl.unwrap(), payload.to_string()));
        if scheme.len() == 10 {
            urls.push(Url::parse_with_params(url,&scheme).unwrap().to_string());
            scheme.clear();
        }
    }
    return urls
}
