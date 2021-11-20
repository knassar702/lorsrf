mod args;
mod requester;
use requester::*;
use args::args;
use scoped_threadpool::Pool;
use std::{fs::File, io::{BufRead,BufReader} };


fn start(url : String ) -> () {
        let the_args = args();
        let requester = Requester{
            timeout:the_args.value_of("timeout").unwrap().parse().unwrap(),
            proxy:the_args.value_of("proxy").unwrap().to_string(),
            headers:extractheaders(the_args.value_of("headers").unwrap()),
            }.build();
        let newurl = add_parameters(url.as_str(),the_args.value_of("host").unwrap(),the_args.value_of("wordlist").unwrap());
        for theurl in newurl {
            match requester.get(theurl) {
                Ok(requester) => {
                    requester
                },
                Err(e) => {
                    println!("[ERR] {:?} : {:?}",url, e.kind().to_string());
                    return;
                }
            };
        }
    }


fn main() {
    let the_args = args();
    extractheaders("TT: fff\nCCV: yyy");
    let mut pool = Pool::new(the_args.value_of("threads").unwrap().parse().unwrap());
    let urls = File::open(the_args.value_of("targets").unwrap().to_string()).expect("file not found!");
    let reader = BufReader::new(urls);
    pool.scoped(|scope|{
            for url in reader.lines() {
                scope.execute(move ||start(url.unwrap()))
            }
        });

}