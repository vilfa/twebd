use crate::web::http::err::HttpResponseError;
use std::{
    io::{BufReader, Read},
    path::PathBuf,
};

pub struct File {
    file_str: String,
    mime: mime_guess::MimeGuess,
    size: usize,
}

impl File {
    pub fn as_string(&self) -> String {
        self.file_str.to_owned()
    }
    pub fn as_buf(&self) -> Vec<u8> {
        self.file_str.as_bytes().to_vec()
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn mime(&self) -> mime_guess::Mime {
        self.mime.first_or_octet_stream()
    }
}

pub struct FileReader<'a> {
    path: &'a PathBuf,
}

impl FileReader<'_> {
    pub fn new(path: &PathBuf) -> FileReader {
        FileReader { path }
    }
    pub fn read(&self) -> Result<File, HttpResponseError> {
        let handle = std::fs::File::open(self.path)?;
        let mut buf_reader = BufReader::new(handle);
        let mut file_str = String::new();
        let size = buf_reader.read_to_string(&mut file_str)?;
        let mime = mime_guess::from_path(self.path);
        Ok(File {
            file_str,
            mime,
            size,
        })
    }
}
