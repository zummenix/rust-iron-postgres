
use iron::status;
use iron::prelude::*;

use error::*;
use utils::*;

pub fn get_user_handler(r: &mut Request) -> IronResult<Response> {
    let id = try!(r.path_param("id").parse::<i32>().map_err(|_| bad_request("Could not parse id.")));
    Ok(Response::with((status::Ok, id.to_string())))
}

pub fn get_users_handler(r: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, r.query_param("search"))))
}
