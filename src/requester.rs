use isahc::{HttpClient, config::{
        RedirectPolicy, 
        VersionNegotiation,
        SslOption}, prelude::*};
use url::{Url};
use std::{
    fs::File,
    time::Duration,
    io::{BufRead,BufReader},
    path::Path,
};
pub struct Requester {
    pub timeout: u64,
    pub proxy: String,
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

        return client.build().unwrap()
    }

}

pub fn add_parameters(url : &str, payload: &str , wordlist: &str ) -> Vec<String> {
    match Path::new(wordlist).exists() {
        true => {
            let test = File::open(wordlist.to_string()).expect("file not found!");
            let reader = BufReader::new(test);

            let mut scheme = vec![];
            let mut urls = Vec::new();
            for theurl in reader.lines() {
                scheme.push((theurl.unwrap(), payload.to_string()));
                if scheme.len() == 10 {
                    urls.push(Url::parse_with_params(url,&scheme).unwrap().to_string());
                    scheme.clear();
                }
            }
            return urls
        },
        false => {
            println!("[ERR] File not found : {}",wordlist);
            return vec![];
        }
    };
}
