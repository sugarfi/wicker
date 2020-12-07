use crate::eval;
use crate::error::error;
use crate::session::{DeleteError, ReadError, Session, SessionType, WriteError};
use url::Url;
use ureq;

#[derive(Clone)]
pub struct WebSession {
    pub url: String,
}

impl Session for WebSession {
    fn read(&self, path: String, ctx: &mut eval::Context) -> Result<String, ReadError> {
        let res = ureq::get(
            Url::join(
                match &Url::parse(&self.url) {
                    Ok(x) => x,
                    Err(e) => {
                        error(format!("{}", e));
                        return Err(ReadError::URLError);
                    }
                },
                &path
            ).unwrap().as_str()
        ).call();
        println!(
            "{}", 
            Url::join(
                &Url::parse(&self.url).unwrap(),
                &path
            ).unwrap().as_str()
        );

        match res.status() {
            200 => Ok(res.into_string().unwrap()),
            404 => Err(ReadError::DoesNotExist),
            401 | 403 => Err(ReadError::NoPermission),

            _ => Err(ReadError::IOError)
        }
    }

    fn write(&self, path: String, ctx: &mut eval::Context) -> Result<(), WriteError> {
        Ok(())
    }

    fn delete(&self, path: String, ctx: &mut eval::Context) -> Result<(), DeleteError> {
        Ok(())
    }

    fn get_type(&self) -> SessionType {
        SessionType::Web
    }
}
