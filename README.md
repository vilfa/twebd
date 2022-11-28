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
            Use https, requires a certificate and private key

    -V, --version
            Prints version information


OPTIONS:
    -a, --address <IP>
            Sets the server IP (v4/v6) address

    -d, --directory <ROOT_PATH>
            Sets the server root/public_html/wwwroot directory

    -c, --https-cert <CERT_PATH>
            Path to the server certificate file

    -k, --https-key <KEY_PATH>
            Path to the server private key file

    -l, --loglevel <LOG_LEVEL>
            Sets the server logging verbosity [possible values: error, warn, info, debug, trace]

    -p, --port <PORT>
            Sets the server port number [possible values: 1, .., 65535]

    -t, --threads <N_THREADS>
            Sets the number of worker threads used by the server [possible values: 1, .., 10]
```

  
## License

[MIT](https://github.com/vilfa/twebd/blob/master/LICENSE)
