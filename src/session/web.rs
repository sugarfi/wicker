use crate::error::error;
use crate::eval;
use crate::session::{DeleteError, ExecError, ReadError, Session, SessionType, WriteError};
use ureq;
use url::Url;

#[derive(Clone)]
pub struct WebSession {
    pub cwd: String,
    pub url: String,
}

impl Session for WebSession {
    fn read(&self, path: String, ctx: &mut eval::Context) -> Result<String, ReadError> {
        let res = ureq::get(
            Url::parse(
                Url::parse(&self.url)
                    .unwrap()
                    .join(&self.get_cwd())
                    .unwrap()
                    .as_str(),
            )
            .unwrap()
            .join(&path)
            .unwrap()
            .as_str(),
        )
        .call();

        match res.status() {
            200 => Ok(res.into_string().unwrap()),
            404 => Err(ReadError::DoesNotExist),
            401 | 403 => Err(ReadError::NoPermission),
            _ => Err(ReadError::IOError),
        }
    }

    fn write(&self, path: String, data: String, ctx: &mut eval::Context) -> Result<(), WriteError> {
        let res = ureq::put(
            Url::parse(
                Url::parse(&self.url)
                    .unwrap()
                    .join(&self.get_cwd())
                    .unwrap()
                    .as_str(),
            )
            .unwrap()
            .join(&path)
            .unwrap()
            .as_str(),
        )
        .send_string(&data);

        match res.status() {
           200 => Ok(()),
           401 | 403 => Err(WriteError::NoPermission),
           _ => Err(WriteError::IOError),
        }
    }

    fn delete(&self, path: String, ctx: &mut eval::Context) -> Result<(), DeleteError> {
        Ok(())
    }

    // TODO: implement
    fn exec(
        &self,
        path: String,
        _args: Vec<String>,
        _ctx: &mut eval::Context,
    ) -> Result<(), ExecError> {
        error(format!("Command \"{}\" not found", path));
        Ok(())
    }

    fn get_type(&self) -> SessionType {
        SessionType::Web
    }

    fn get_cwd(&self) -> String {
        self.cwd.clone()
    }

    fn set_cwd(&mut self, cwd: String) {
        self.cwd = if cwd.ends_with("/") {
            cwd
        } else {
            format!("{}/", cwd)
        };
    }
}
