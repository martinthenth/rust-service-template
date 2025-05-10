use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    BadRequest,
    Unauthenticated,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    Validation(BTreeMap<String, String>),
    InternalServer(String),
    NotImplemented,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
