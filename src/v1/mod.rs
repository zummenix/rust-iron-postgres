
use iron::status;
use iron::prelude::*;

use urlencoded::UrlEncodedQuery;
use error::*;
use utils::*;

pub fn get_user_handler(r: &mut Request) -> IronResult<Response> {
    let id = try!(r.param("id").parse::<i32>().map_err(|_| bad_request("Could not parse id.")));
    Ok(Response::with((status::Ok, id.to_string())))
}

pub fn get_users_handler(r: &mut Request) -> IronResult<Response> {
    let search = if let Ok(ref hashmap) = r.get_ref::<UrlEncodedQuery>() {
        hashmap.get("search").and_then(|v| v.first())
    } else {
        None
    };
    if let Some(s) = search {
        Ok(Response::with((status::Ok, s.clone())))
    } else {
        Ok(Response::with(status::Ok))
    }
}
