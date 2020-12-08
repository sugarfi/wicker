pub mod local;
pub mod web;
use crate::eval;
use dyn_clone;

pub enum ReadError {
    NoPermission,
    DoesNotExist,
    IOError,
    URLError,
}

pub enum WriteError {
    NoPermission,
    IOError,
}

pub enum DeleteError {
    NoPermission,
    DoesNotExist,
    IOError,
}

pub enum ExecError {
    ExecFailure,
}

pub enum SessionType {
    Local,
    Web,
}

pub trait Session: dyn_clone::DynClone {
    fn read(&self, path: String, ctx: &mut eval::Context) -> Result<String, ReadError>;
    fn write(&self, path: String, ctx: &mut eval::Context) -> Result<(), WriteError>;
    fn delete(&self, path: String, ctx: &mut eval::Context) -> Result<(), DeleteError>;
    fn exec(&self, path: String, args: Vec<String>, ctx: &mut eval::Context) -> Result<(), ExecError>;
    fn get_type(&self) -> SessionType;
    fn get_cwd(&self) -> String;
    fn set_cwd(&mut self, cwd: String);
}
