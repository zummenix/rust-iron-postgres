
extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::Iron;
use router::Router;

mod v1;
mod error;
mod utils;

fn main() {
    let mut router = Router::new();
    router.get("/api/v1/users/:id", v1::get_user_handler);
    router.get("/api/v1/users", v1::get_users_handler);

    Iron::new(router).http("localhost:3000").unwrap();
}
