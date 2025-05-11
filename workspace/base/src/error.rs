use async_graphql::ErrorExtensions;
use heck::ToLowerCamelCase;
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

impl Error {
    pub fn into(self) -> async_graphql::Error {
        match self {
            Error::BadRequest => async_graphql::Error::new("BAD_REQUEST"),
            Error::Unauthenticated => async_graphql::Error::new("UNAUTHENTICATED"),
            Error::Unauthorized => async_graphql::Error::new("UNAUTHORIZED"),
            Error::Forbidden => async_graphql::Error::new("FORBIDDEN"),
            Error::NotFound => async_graphql::Error::new("NOT_FOUND"),
            Error::Conflict => async_graphql::Error::new("CONFLICT"),
            Error::Validation(errors) => {
                async_graphql::Error::new("VALIDATION").extend_with(|_, e| {
                    for (key, value) in errors {
                        e.set(key.to_lower_camel_case(), value);
                    }
                })
            }
            Error::InternalServer(_) => async_graphql::Error::new("INTERNAL_SERVER"),
            Error::NotImplemented => async_graphql::Error::new("NOT_IMPLEMENTED"),
        }
    }
}
