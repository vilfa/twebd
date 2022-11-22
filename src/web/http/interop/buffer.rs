use crate::web::http::{
    delim,
    interop::ToBuf,
    native::{
        HttpBody, HttpHeader, HttpLine, HttpMethod, HttpResponse, HttpResponseLine, HttpStatus,
        HttpVersion,
    },
};

impl ToBuf for HttpBody {
    fn to_buf(&self) -> Vec<u8> {
        self.tokens.join("").into_bytes()
    }
}

impl ToBuf for HttpHeader {
    fn to_buf(&self) -> Vec<u8> {
        let mut buf = String::new();
        for (key, value) in self.headers.iter() {
            buf.push_str(&format!("{}: {}\r\n", &key, &value));
        }
        buf.push_str(delim::CRLF);
        buf.into_bytes()
    }
}

impl ToBuf for HttpLine {
    fn to_buf(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.append(&mut self.method.to_buf());
        buf.append(&mut delim::WSPC.as_bytes().to_vec());
        buf.append(&mut self.uri.to_str().unwrap().as_bytes().to_vec());
        buf.append(&mut delim::WSPC.as_bytes().to_vec());
        buf.append(&mut self.version.to_buf());
        buf.append(&mut delim::CRLF.as_bytes().to_vec());
        buf
    }
}

impl ToBuf for HttpMethod {
    fn to_buf(&self) -> Vec<u8> {
        match self {
            Self::Options => b"OPTIONS".to_vec(),
            Self::Get => b"GET".to_vec(),
            Self::Head => b"HEAD".to_vec(),
            Self::Post => b"POST".to_vec(),
            Self::Put => b"PUT".to_vec(),
            Self::Delete => b"DELETE".to_vec(),
            Self::Trace => b"TRACE".to_vec(),
            Self::Connect => b"CONNECT".to_vec(),
            Self::Patch => b"PATCH".to_vec(),
        }
    }
}

impl ToBuf for HttpResponse {
    fn to_buf(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let bufs = vec![
            HttpResponseLine::new(self.version, self.status).to_buf(),
            self.header.to_buf(),
            self.body.to_buf(),
        ];
        for mut b in bufs {
            buf.append(&mut b);
        }
        buf
    }
}

impl ToBuf for HttpResponseLine {
    fn to_buf(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.append(&mut self.version.to_buf());
        buf.append(&mut delim::WSPC.as_bytes().to_vec());
        buf.append(&mut self.status.to_buf());
        buf.append(&mut delim::CRLF.as_bytes().to_vec());
        buf
    }
}

impl ToBuf for HttpStatus {
    fn to_buf(&self) -> Vec<u8> {
        format!("{}", self).as_bytes().to_vec()
    }
}

impl ToBuf for HttpVersion {
    fn to_buf(&self) -> Vec<u8> {
        match self {
            Self::Http11 => b"HTTP/1.1".to_vec(),
            Self::Http20 => b"HTTP/2.0".to_vec(),
            Self::Http30 => b"HTTP/3.0".to_vec(),
        }
    }
}
