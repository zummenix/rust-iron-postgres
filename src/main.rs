
extern crate iron;
extern crate router;
extern crate urlencoded;

use iron::Iron;
use router::Router;

mod v1;
mod error;

fn main() {
    let mut router = Router::new();
    router.get("/api/v1/users/:id", v1::GetUser);
    router.get("/api/v1/users", v1::GetUsers);

    Iron::new(router).http("localhost:3000").unwrap();
}
