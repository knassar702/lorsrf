mod args;
mod requester;
use crate::requester::*;
use crate::args::args;
use indicatif::ProgressBar;
use scoped_threadpool::Pool;
use std::{
    fs::File, 
    io::{
        BufRead,
        BufReader
    }
};


fn main() {
    let the_args = args();
    let mut pool = Pool::new(the_args.value_of("threads").unwrap().parse().unwrap());
    let urls = File::open(the_args.value_of("targets").unwrap().to_string()).expect("file not found!");
    let _reader = BufReader::new(urls);
    let _requester = Requester{
        timeout:the_args.value_of("timeout").unwrap().parse().unwrap(),
        proxy:the_args.value_of("proxy").unwrap().to_string(),
        headers:extractheaders(the_args.value_of("headers").unwrap()),
        }.build();
    let params = convert_vec( BufReader::new(File::open(the_args.value_of("wordlist").unwrap()).expect("file not found ")) );
    let bar = ProgressBar::new(params.len() as u64 * 4 );
    pool.scoped(|scope|{
        for _url in _reader.lines() {
            let urls = add_parameters(_url.unwrap(),the_args.value_of("host").unwrap(),params.clone());
            for url in urls {
                scope.execute(|| { 

                        match _requester.get(url) {
                            Ok(_done) => {},
                            Err(_e) => {println!("[Err] {:?}",_e)}
                        }
                        bar.inc(1);

                });
            }
        }
        });

}
