
use std::fmt;
use std::error;
use iron::status::Status;
use iron::IronError;
use json::builder::ObjectBuilder;
use json;

#[derive(Debug, PartialEq)]
pub enum Error {
    BadRequest(String),
}

impl Error {
    fn status(&self) -> Status {
        match *self {
            Error::BadRequest(_) => Status::BadRequest,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadRequest(ref e) => e,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BadRequest(ref e) => writeln!(f, "BadRequest: {}", e),
        }
    }
}

impl From<Error> for IronError {
    fn from(error: Error) -> Self {
        let object = ObjectBuilder::new()
            .insert("details", error::Error::description(&error))
            .unwrap();
        let response = (error.status(), json::to_string_pretty(&object).unwrap());
        IronError::new(error, response)
    }
}

pub fn bad_request<T>(details: T) -> IronError
    where T: Into<String>
{
    Error::BadRequest(details.into()).into()
}
