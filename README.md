Lorsrf - SSRF Parameter bruteforce tool

### Requirement
* Rust
* Cargo
* Git

### install
```bash
>>> git clone https://github.com/knassar702/lorsrf
>>> cd lorsrf
>>> cargo build --release
>>> sudo ln -s target/release/lorsrf /usr/bin/lorsrf
>>> lorsrf --help
 
```


### Usage

```bash
$ lorsrf --wordlist parameters.txt --call (YOUR BURP COLLABORATOR or interactsh.com host ) --urls your_targets_list.txt
```


more options

```bash
USAGE:
    lorsrf [OPTIONS] --call <host> --urls <targets> --wordlist <wordlist>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -H, --headers <headers>      Your Headers [default: ]
    -c, --call <host>            Your The OAST Host (burpsuite collaborator or interactsh.com)
    -p, --proxy <proxy>          SendProxy [default: ]
    -u, --urls <targets>         Your Targets list
        --threads <threads>      Your Threads [default: 10]
    -t, --timeout <timeout>      Set the Timeout of the requests [default: 10]
    -w, --wordlist <wordlist>    Your Parameters Wordlist
```
