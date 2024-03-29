extern crate clap;
use clap::{App, Arg, ArgMatches};

pub fn args() -> ArgMatches<'static> {
    // Create the CLI app
    return App::new("Lorsrf")
        .version("2.1")
        .author("Khaled Nassar <knassar702@gmail.com>")
        .about("SSRF Parameter BruteForce Tool")
        .arg(
            Arg::with_name("targets")
                .help("Your Targets list")
                .required(true)
                .short("u")
                .takes_value(true)
                .long("urls"),
        )
        .arg(
            Arg::with_name("timeout")
                .help("Set the Timeout of the requests")
                .short("t")
                .long("timeout")
                .default_value("10")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("host")
                .help("Your The OAST Host (burpsuite collaborator or interactsh.com)")
                .required(true)
                .takes_value(true)
                .short("c")
                .long("call"),
        )
        .arg(
            Arg::with_name("proxy")
                .help("SendProxy")
                .takes_value(true)
                .default_value("")
                .short("p")
                .long("proxy"),
        )
        .arg(
            Arg::with_name("wordlist")
                .help("Your Parameters Wordlist")
                .long("wordlist")
                .short("w")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("threads")
                .help("Your Threads")
                .long("threads")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("headers")
                .help("Your Headers")
                .long("headers")
                .short("H")
                .takes_value(true)
                .default_value(""),
        )
        .arg(
            Arg::with_name("json")
                .help("Use JSON requests via POST method")
                .long("json")
                .short("j")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("form")
                .help("Use x-www-form-urlencoded requests via POST method")
                .long("form")
                .short("f")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("post-only")
                .help("POST method only")
                .long("post-only")
                .takes_value(false),
        )
        .get_matches();
}
