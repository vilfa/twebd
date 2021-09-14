use crate::{cli::CliOpt, web::http::response::HttpResponseError};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

pub struct FileReader<'a> {
    path: &'a PathBuf,
}

impl FileReader<'_> {
    pub fn new(path: &PathBuf) -> FileReader {
        FileReader { path }
    }
    pub fn read_as_bytes(&self) -> Result<Vec<u8>, HttpResponseError> {
        let content = self.read()?;
        Ok(content.as_bytes().to_vec())
    }
    pub fn read_as_string(&self) -> Result<String, HttpResponseError> {
        let content = self.read()?;
        Ok(content)
    }
    fn read(&self) -> Result<String, HttpResponseError> {
        let handle = File::open(self.path)?;
        let mut buf_reader = BufReader::new(handle);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content)?;
        Ok(content)
    }
}

impl From<std::io::Error> for HttpResponseError {
    fn from(e: std::io::Error) -> Self {
        HttpResponseError::FileReaderError(e)
    }
}

pub struct ServerRootBuilder {
    root: PathBuf,
}

impl ServerRootBuilder {
    pub fn new(opts: Vec<CliOpt>) -> ServerRootBuilder {
        let mut server_root_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Directory(v) => server_root_builder.root = v.to_path_buf(),
                _ => {}
            }
        }
        server_root_builder
    }
    pub fn root(&self) -> PathBuf {
        self.root.to_path_buf()
    }
}

impl Default for ServerRootBuilder {
    fn default() -> Self {
        ServerRootBuilder {
            root: PathBuf::from("./"),
        }
    }
}
