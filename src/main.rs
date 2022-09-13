mod args;
mod requester;
use crate::args::args;
use crate::requester::*;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_json::json;
use std::{fs::File, io::BufReader};

#[tokio::main]
async fn main() {
    let the_args = args();
    let threader = rayon::ThreadPoolBuilder::new()
        .num_threads(the_args.value_of("threads").unwrap().parse().unwrap())
        .build()
        .unwrap();
    let urls_file =
        File::open(the_args.value_of("targets").unwrap().to_string()).expect("file not found!");
    let reader = BufReader::new(urls_file);
    let requester = Requester {
        timeout: the_args.value_of("timeout").unwrap().parse().unwrap(),
        proxy: the_args.value_of("proxy").unwrap().to_string(),
        headers: extractheaders(the_args.value_of("headers").unwrap()),
    }
    .build();
    let params: Vec<String> = {
        if the_args.is_present("wordlist") {
            convert_vec(BufReader::new(
                File::open(the_args.value_of("wordlist").unwrap()).expect("file not found "),
            ))
        } else {
            vec![String::from("")]
        }
    };

    let urls = convert_vec(reader);
    let bar = ProgressBar::new(urls.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );
    threader.install(move || {
        urls.par_iter().for_each(|url| {
            let urls = add_parameters(
                url.clone().to_string(),
                the_args.value_of("host").unwrap(),
                params.clone(),
            );
            urls.par_iter().for_each(|url| {
                let url = url;
                if the_args.is_present("post-only") == false {
                    match requester
                        .get(url.clone().as_str().replace("%25METHOD%25", "get").as_str())
                    {
                        Ok(_done) => {}
                        Err(_e) => {}
                    }
                }

                if the_args.is_present("json") == true {
                    match requester.post(
                        {
                            if url.as_str().split_once("?") == None {
                                url.as_str()
                            } else {
                                url.as_str().split_once("?").unwrap().0
                            }
                        },
                        json!(query(
                            url.clone()
                                .as_str()
                                .replace("%25METHOD%25", "post")
                                .as_str()
                        ))
                        .to_string(),
                    ) {
                        Ok(_done) => {}
                        Err(_e) => {}
                    }
                }

                if the_args.is_present("form") == true {
                    match requester.post(
                        {
                            if url.as_str().split_once("?") == None {
                                url.as_str()
                            } else {
                                url.as_str().split_once("?").unwrap().0
                            }
                        },
                        extract_params(
                            url.split_once("?").unwrap().0,
                            query(
                                url.clone()
                                    .as_str()
                                    .replace("%25METHOD%25", "post")
                                    .as_str(),
                            ),
                        ),
                    ) {
                        Ok(_done) => {}
                        Err(_e) => {}
                    }
                }
            bar.inc(1);

            });
        });
    });
}
