use crate::{
    cli::{Build, CliOpt, Other},
    srv::err::ServerRootError,
};
use std::path::PathBuf;

pub struct ServerRootBuilder {
    root: PathBuf,
    _other: Vec<CliOpt>,
}

impl Build<Self, PathBuf, ServerRootError> for ServerRootBuilder {
    fn from(opts: Vec<CliOpt>) -> Self {
        let mut server_root_builder = Self::default();
        for opt in opts {
            match opt {
                CliOpt::Directory(v) => server_root_builder.root = v.to_path_buf(),
                cli_opt => server_root_builder.add_other(cli_opt.to_owned()),
            }
        }
        server_root_builder
    }
    fn build(&self) -> Result<PathBuf, ServerRootError> {
        Ok(self.root.to_path_buf())
    }
}

impl Other for ServerRootBuilder {
    fn add_other(&mut self, o: CliOpt) {
        self._other.push(o);
    }
    fn other(&self) -> Vec<CliOpt> {
        self._other.to_vec()
    }
}

impl Default for ServerRootBuilder {
    fn default() -> Self {
        ServerRootBuilder {
            root: PathBuf::from("./"),
            _other: Vec::new(),
        }
    }
}
