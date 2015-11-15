
use iron::{Handler, Request, Response, IronResult};
use iron::status;
use router::{Router};
use error::*;

pub struct GetUser;

impl Handler for GetUser {
    fn handle(&self, r: &mut Request) -> IronResult<Response> {
        let id = r.extensions.get::<Router>().unwrap().find("id").unwrap_or("");
        let id = try!(id.parse::<i32>().map_err(|_| bad_request("Could not parse id.")));
        Ok(Response::with((status::Ok, id.to_string())))
    }
}
