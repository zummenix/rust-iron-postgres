
use std::fmt;
use std::error;
use iron::status::Status;
use iron::IronError;
use json::builder::ObjectBuilder;
use json;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub message: String,
    pub status: Status,
}

impl Error {
    pub fn new<T>(message: T, status: Status) -> Self
        where T: Into<String>
    {
        Error {
            message: message.into(),
            status: status,
        }
    }

    pub fn json(&self) -> json::Value {
        ObjectBuilder::new()
            .insert_object("error", |builder| builder.insert("message", &self.message))
            .unwrap()
    }

    pub fn json_string(&self) -> String {
        json::to_string_pretty(&self.json()).unwrap()
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Error: {}", self.message)
    }
}

impl From<Error> for IronError {
    fn from(error: Error) -> Self {
        let response = (error.status, error.json_string());
        IronError::new(error, response)
    }
}

pub fn bad_request<T>(details: T) -> IronError
    where T: Into<String>
{
    Error::new(details, Status::BadRequest).into()
}
