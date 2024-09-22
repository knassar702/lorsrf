use std::usize;

// args.rs
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Lorsrf", about = "SSRF Parameter BruteForce Tool")]
pub struct Args {
    /// Your Targets list
    #[structopt(short, long)]
    pub targets: String,

    /// Set the Timeout of the requests
    #[structopt(short, long, default_value = "10")]
    pub timeout: u64,

    /// The OAST Host (e.g., Burpsuite Collaborator or interactsh.com)
    #[structopt(short, long)]
    pub host: String,

    /// Send proxy
    #[structopt(short, long)]
    pub proxy: Option<String>,
    #[structopt(short, long)]
    pub concurrency: Option<usize>,

    /// Your Parameters Wordlist
    #[structopt(short, long)]
    pub wordlist: Option<String>,

    /// Your Headers
    #[structopt(short, long, default_value = "")]
    pub headers: String,

    /// Use JSON requests via POST method
    #[structopt(short, long)]
    pub json: bool,

    /// Use x-www-form-urlencoded requests via POST method
    #[structopt(short, long)]
    pub form: bool,

    /// Use POST method only
    #[structopt(long)]
    pub post_only: bool,
}
