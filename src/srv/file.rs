use crate::{cli::CliOpt, web::http::response::HttpResponseError};
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

impl From<std::io::Error> for HttpResponseError {
    fn from(e: std::io::Error) -> Self {
        HttpResponseError::FileReaderError(e)
    }
}

pub struct ServerRootBuilder {
    root: PathBuf,
    other: Vec<CliOpt>,
}

impl ServerRootBuilder {
    pub fn new(opts: Vec<CliOpt>) -> ServerRootBuilder {
        let mut server_root_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Directory(v) => server_root_builder.root = v.to_path_buf(),
                cli_opt => server_root_builder.other.push(cli_opt.to_owned()),
            }
        }
        server_root_builder
    }
    pub fn root(&self) -> PathBuf {
        self.root.to_path_buf()
    }
    pub fn other(&self) -> Vec<CliOpt> {
        self.other.to_vec()
    }
}

impl Default for ServerRootBuilder {
    fn default() -> Self {
        ServerRootBuilder {
            root: PathBuf::from("./"),
            other: Vec::new(),
        }
    }
}
