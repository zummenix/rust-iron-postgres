
use iron::status;
use iron::prelude::*;

use error::*;
use utils::*;

pub use self::db_init::db_init;

mod db_init;

pub fn get_user_handler(request: &mut Request) -> IronResult<Response> {
    let id = try!(request.path_param("id")
                         .parse::<i32>()
                         .map_err(|_| bad_request("Could not parse id.")));
    Ok(Response::with((status::Ok, id.to_string())))
}

pub fn get_users_handler(request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, request.query_param("search"))))
}
