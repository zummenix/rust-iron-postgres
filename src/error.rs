
use std::fmt;
use std::error;
use iron::status::Status;
use iron::IronError;

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
        let details = format!("{{'details': '{}'}}", error::Error::description(&error));
        let response = (error.status(), details);
        IronError::new(error, response)
    }
}

pub fn bad_request<T>(details: T) -> IronError
    where T: Into<String>
{
    Error::BadRequest(details.into()).into()
}
