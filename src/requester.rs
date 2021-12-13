use isahc::{HttpClient, config::{
        RedirectPolicy, 
        VersionNegotiation,
        SslOption}, prelude::*};
use url::Url;
use std::collections::HashMap;
use regex::Regex;
use std::{
    fs::File,
    time::Duration,
    io::{
    BufRead,
    BufReader},
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

pub fn extract_params(url: &str,params: HashMap<String,String> ) -> String {
    let data = Url::parse_with_params(url , params).unwrap();
    return String::from(data.query().unwrap());
}

pub fn convert_vec(wordlist: BufReader<File> ) -> Vec<String> {
    let mut scheme = Vec::new();
    for data in wordlist.lines() {
        scheme.push(data.unwrap().to_string());
    }
    return scheme;

}


pub fn add_parameters(url : String, payload: &str , wordlist: Vec<String>) -> Vec<String> {
    let mut scheme = vec![];
    let mut urls = Vec::new();
    let url_path = Url::parse(url.as_str()).unwrap();
    for theurl in wordlist {
        scheme.push((
                theurl.clone(), 
                payload.replace("%PARAM%",theurl.clone().as_str()).replace("%PATH%",url_path.path())
                     ));
        if scheme.len() == 10 {
            urls.push(Url::parse_with_params(url.as_str(),&scheme).unwrap().to_string());
            scheme.clear();
        }
    }
    return urls
}



pub fn query(url: &str) -> HashMap<String,String> {
    let parsed_url = Url::parse(url).unwrap();
    let hash_query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();
    return hash_query;
}

