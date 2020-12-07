pub mod local;
pub mod web;
use crate::eval;
use dyn_clone;

pub enum ReadError {
    NoPermission,
    DoesNotExist,
    IOError,
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

pub enum SessionType {
    Local,
    Web,
}

pub trait Session: dyn_clone::DynClone {
    fn read(&self, path: String, ctx: &mut eval::Context) -> Result<String, ReadError>;
    fn write(&self, path: String, ctx: &mut eval::Context) -> Result<(), WriteError>;
    fn delete(&self, path: String, ctx: &mut eval::Context) -> Result<(), DeleteError>;
    fn get_type(&self) -> SessionType;
}
