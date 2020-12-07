use crate::eval;
use crate::session::{DeleteError, ReadError, Session, SessionType, WriteError};
use std::fs;

#[derive(Clone)]
pub struct LocalSession;

impl Session for LocalSession {
    fn read(&self, path: String, ctx: &mut eval::Context) -> Result<String, ReadError> {
        let path = match fs::canonicalize(path) {
            Ok(x) => x,
            Err(_) => {
                return Err(ReadError::DoesNotExist);
            }
        };

        Ok(match String::from_utf8(match fs::read(path) {
            Ok(x) => x,
            Err(_) => {
                return Err(ReadError::IOError);
            }
        }) {
            Ok(x) => x,
            Err(_) => {
                return Err(ReadError::IOError);
            }
        })
    }

    fn write(&self, path: String, ctx: &mut eval::Context) -> Result<(), WriteError> {
        Ok(())
    }

    fn delete(&self, path: String, ctx: &mut eval::Context) -> Result<(), DeleteError> {
        Ok(())
    }

    fn get_type(&self) -> SessionType {
        SessionType::Local
    }
}
