use crate::{
    web::{HttpBody, HttpHeader, HttpResponse, HttpStatus, HttpVersion},
    APP_NAME, APP_VERSION,
};
use chrono::prelude::*;
use std::collections::HashMap;

impl Default for HttpBody {
    fn default() -> Self {
        HttpBody { tokens: Vec::new() }
    }
}

impl Default for HttpHeader {
    fn default() -> Self {
        let mut headers = HashMap::new();
        headers.insert(String::from("Date"), Utc::now().to_rfc2822());
        headers.insert(
            String::from("Server"),
            format!("{}/{}", APP_NAME, APP_VERSION),
        );
        HttpHeader { headers }
    }
}

impl Default for HttpResponse {
    fn default() -> Self {
        HttpResponse {
            version: HttpVersion::default(),
            status: HttpStatus::default(),
            header: HttpHeader::default(),
            body: HttpBody::default(),
        }
    }
}

impl Default for HttpStatus {
    fn default() -> Self {
        Self::OK
    }
}

impl Default for HttpVersion {
    fn default() -> Self {
        Self::Http11
    }
}
