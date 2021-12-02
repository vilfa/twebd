# twebd

Twebd is a **t**iny **web** **d**aemon with very simple multi-threading and support for both http and https. 
This project was made for fun, its aim is to provide simple web server functionaility for personal projects.


## Installation

Clone the repo.
```bash
$ git clone https://github.com/vilfa/twebd
$ cd twebd
```

You can then install the package with cargo and run it as any other executable
```bash
$ cargo install --path .
$ twebd [FLAGS] [OPTIONS]
```
**OR**

just compile and run it directly.
```bash
$ cargo r -- [FLAGS] [OPTIONS]
```
## Usage/Examples

```
USAGE:
    twebd [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       
            Prints help information

    -s, --https      
            Use https. You must also specify certificate and private key files, which can be generated using the openssl
            utility.
    -V, --version    
            Prints version information


OPTIONS:
    -a, --address <IPV4_OR_6>               
            Sets the server IP address. Both IPv4 and IPv6 are supported.

    -f, --directory <ROOT_PATH>             
            Sets the server root directory. This is the so called public_html/wwwroot directory, from which web content
            is served.
        --https-cert <CERT_PATH>            
            File path to server certificate file. This is the certificate that the server presents to the web browser
            when negotiating a TLS session.
        --https-priv-key <PRIV_KEY_PATH>    
            File path to the server key file. This is the key used for negotiating the TLS cipher suite with the
            browser.
    -l, --loglevel <LEVEL>                  
            Sets the server logging verbosity. Anything higher than info is not recommended, as it is very verbose.
            [possible values: error, warn, info, debug, trace]
    -p, --port <PORT>                       
            Sets the server port number [possible values: 1..65535]. Please note, that ports lower than including 1024
            are system reserved and cannot be used, unless running as root which SHOULD NOT be done.
    -t, --threads <N_THREADS>               
            Sets the number of threads used by the server [possible values: 1..10]. Multi-threading is only supported
            for the http traffic. A reasonable maximum is set at 10 threads.
```

  
## License

[MIT](https://github.com/vilfa/twebd/blob/master/LICENSE)
