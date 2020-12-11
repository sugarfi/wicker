use crate::eval;
use crate::session::{DeleteError, ExecError, ReadError, Session, SessionType, WriteError};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[derive(Clone)]
pub struct LocalSession {
    pub cwd: String,
}

impl Session for LocalSession {
    fn read(&self, path: String, ctx: &mut eval::Context) -> Result<String, ReadError> {
        let path = if Path::new(&path).is_relative() {
            Path::new(&self.get_cwd())
                .join(&path)
                .to_str()
                .unwrap()
                .to_string()
        } else {
            Path::new(&path).to_str().unwrap().to_string()
        };

        let path = match fs::canonicalize(path) {
            Ok(x) => x,
            Err(_) => {
                return Err(ReadError::DoesNotExist);
            }
        };

        Ok(
            match String::from_utf8(match fs::read(path) {
                Ok(x) => x,
                Err(_) => {
                    return Err(ReadError::IOError);
                }
            }) {
                Ok(x) => x,
                Err(_) => {
                    return Err(ReadError::IOError);
                }
            },
        )
    }

    fn write(&self, path: String, data: String, ctx: &mut eval::Context) -> Result<(), WriteError> {
        let path = if Path::new(&path).is_relative() {
            Path::new(&self.get_cwd()).join(&path)
        } else {
            Path::new(&path).to_path_buf()
        };

        let mut file = match fs::File::create(path) {
            Ok(x) => x,
            Err(_) => {
                return Err(WriteError::IOError);
            }
        };

        match file.write_all(data.as_bytes()) {
            Ok(x) => x,
            Err(_) => {
                return Err(WriteError::IOError);
            }
        }

        Ok(())
    }

    fn delete(&self, path: String, ctx: &mut eval::Context) -> Result<(), DeleteError> {
        let path = if Path::new(&path).is_relative() {
            Path::new(&self.get_cwd())
                .join(&path)
                .to_str()
                .unwrap()
                .to_string()
        } else {
            Path::new(&path).to_str().unwrap().to_string()
        };

        let path = match fs::canonicalize(path) {
            Ok(x) => x,
            Err(_) => {
                return Err(DeleteError::DoesNotExist);
            }
        };

        if Path::new(&path).is_dir() {
            match fs::remove_dir(path) {
                Ok(_) => { },
                Err(_) => {
                    return Err(DeleteError::IOError);
                }
            }
        } else {
            match fs::remove_file(path) {
                Ok(_) => { },
                Err(_) => {
                    return Err(DeleteError::IOError);
                }
            }
        }

        Ok(())
    }

    fn exec(
        &self,
        path: String,
        args: Vec<String>,
        ctx: &mut eval::Context,
    ) -> Result<(), ExecError> {
        let cmd = Command::new(path).args(args).status();

        match cmd {
            Ok(x) => Ok(()),
            Err(e) => Err(ExecError::ExecFailure),
        }
    }

    fn get_type(&self) -> SessionType {
        SessionType::Local
    }

    fn get_cwd(&self) -> String {
        self.cwd.clone()
    }

    fn set_cwd(&mut self, cwd: String) {
        self.cwd = cwd;
    }
}
