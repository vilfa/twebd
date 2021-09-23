use crate::{
    cli::parser,
    srv::{HttpServer, HttpsServer, Server},
};

pub fn run() {
    let matches = parser::parse_args();
    match parser::parse_matches(&matches) {
        Ok((opts, https)) => {
            if https {
                HttpServer::new(opts).listen();
            } else {
                HttpsServer::new(opts).listen();
            }
        }
        Err(e) => err!("{:?}", e),
    }
}
