Lorsrf - SSRF Parameter bruteforce tool via POST & GET Request with JSON Support

### Requirement
* Rust
* Cargo
* Git

### install
```bash
>>> apt install gcc pkg-config libssl-dev
>>> cargo install --git https://github.com/knassar702/lorsrf
 
```


### Usage

```bash
$ lorsrf --wordlist parameters.txt --call (YOUR BURP COLLABORATOR or interactsh.com host ) --urls your_targets_list.txt
```

#### example

```bash
$ waybackurls testphp.vulnweb.com > targets.txt
$ lorsrf --wordlist parameters.txt --call c64fsyz2vtc0000341c0gdn8h9oyyyyyb.interactsh.com --urls target.txt
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


### TODO-LIST
* https://github.com/knassar702/lorsrf/projects/1
