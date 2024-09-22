mod args;
mod requester;

use crate::args::Args;
use crate::requester::*;
use futures::stream::{self, StreamExt};
use std::{fs::File, io::BufReader};
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let args = Args::from_args();
    let urls_file = File::open(&args.targets).expect("file not found!");
    let reader = BufReader::new(urls_file);

    let requester = Requester {
        timeout: args.timeout,
        proxy: args.proxy.clone(),
        headers: extract_headers(&args.headers).expect("Failed to parse headers"), // Unwrap with a message
    }
    .build()
    .await;

    let params: Vec<String> = if let Some(wordlist_path) = &args.wordlist {
        convert_vec(BufReader::new(
            File::open(wordlist_path).expect("file not found"),
        ))
    } else {
        vec![String::new()]
    };

    let urls = convert_vec(reader);

    // Define the level of concurrency
    let concurrency_limit = args.concurrency.unwrap_or(10); // Default to 10 if no concurrency limit is provided

    stream::iter(urls)
        .for_each_concurrent(concurrency_limit, |url| {
            let requester = requester.clone();
            let params = params.clone();
            let host = args.host.clone();
            async move {
                let urls = add_parameters(url.clone(), &host, params);
                for url in urls {
                    if !args.post_only {
                        if let Ok(_) = requester.get(&url).send().await {
                            // Handle the GET request
                        }
                    }

                    if args.json {
                        if let Ok(_) = requester.post(&url).json(&query(&url)).send().await {
                            // Handle JSON POST request
                        }
                    }

                    if args.form {
                        if let Ok(_) = requester.post(&url).form(&query(&url)).send().await {
                            // Handle form-urlencoded POST request
                        }
                    }
                }
            }
        })
        .await;
}
