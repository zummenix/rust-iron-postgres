
use std::fmt;
use std::error;
use iron::status::Status;
use iron::IronError;

#[derive(Debug)]
struct Error {
    pub status: Status,
    pub payload: String,
}

impl Error {
    fn new(status: Status, payload: String) -> Self {
        Error {
            status: status,
            payload: payload,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.payload
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Status: {}\nPayload:\n{}", self.status, self.payload)
    }
}

pub fn bad_request<T>(details: T) -> IronError where T: Into<String> {
    let details = format!("{{'details': '{}'}}", details.into());
    let error = Error::new(Status::BadRequest, details.clone());
    IronError::new(error, (Status::BadRequest, details))
}
