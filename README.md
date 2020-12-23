# lorsrf
Bruteforcing on Hidden parameters to find SSRF vulnerability using `GET` and `POST` Methods
## install
* download it 
```bash
➜  git clone https://github.com/knassar702/lorsrf
➜  cd lorsrf
➜  sudo pip3 install requests flask
```
* install <a href='https://www.youtube.com/watch?v=4sF1ATYwr3U'>ngrok tool </a>

## Steps :
### Ngrok
* run your ngrok
` ./ngrok http 9090`
* run `server.py` script and add ngrok port
`python3 server.py 9090`
* run `lorsrf.py` and add ngrok host using `-s` option
### requestbin.com
* login to https://requestbin.com
* copy your host and add it by using `-s` option (without `server.py` file)
## How can i use it .? 

`cat YOUR_LIST.txt | python3 lorsrf.py -t URL_TARGET -s YOUR_HOST -w wordlist.txt`
  ## Examples :
```bash
$ cat paramters.txt | python3 lorsrf.py -t http://target.com -s http://53252.ngrok.io
```
  * add threads
```bash
$ cat paramters.txt | python3 lorsrf.py -t http://target.com -s http://53252.ngrok.io --threads=50
```
  * add timeout
```bash
$ cat paramters.txt | python3 lorsrf.py -t http://target.com -s http://53252.ngrok.io --timeout=4
```
  * add cookies
```bash
$ cat paramters.txt | python3 lorsrf.py -t http://target.com -s http://53252.ngrok.io -c 'user=5&PHPSESSION=5232'
```
  * add headers from text file
```bash

$ cat headers.txt
Cookie: test=1
Auth: Basic TG9yU3JmCg==

$ cat parameters.txt | python3 lorsrf.py -f headers.txt -s 'http://myhost.com' -t 'http://ssrf.hack.com'

---------------------
GET /?parameter={YOUR_HOST} HTTP/1.1
Host: targer.com
Cookie: test=1
Auth: Basic TG9yU3JmCg==
```
  * Follow redirects
```bash
$ cat paramters.txt | python3 lorsrf.py -t http://target.com -s http://53252.ngrok.io -r
```


## Testing
```python3
python3 lorsrf.py -t 'http://testphp.vulnweb.com/showimage.php' -s 'https://YOUR_HOST.com' -w parameters.txt
```
## GIF
<img src='src/lorsrf.gif'>
