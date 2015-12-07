
extern crate iron;
extern crate router;
extern crate urlencoded;
extern crate serde_json as json;
extern crate iron_postgres_middleware;
extern crate r2d2_postgres;
extern crate r2d2;

use iron::{Iron, Chain};
use router::Router;
use iron_postgres_middleware::PostgresMiddleware;

mod v1;
mod error;
mod utils;

fn main() {
    let mut router = Router::new();
    router.get("/api/v1/users/:id", v1::get_user_handler);
    router.get("/api/v1/users", v1::get_users_handler);

    let mut chain = Chain::new(router);

    match PostgresMiddleware::new("postgres://zummenix@localhost/example") {
        Ok(pg_middleware) => {
            {
                let conn = pg_middleware.pool.get().unwrap();
                v1::db_init(&conn);
            }
            chain.link_before(pg_middleware);
        },
        Err(err) => {
            panic!("Database error: {:}", err.description());
        }
    }

    Iron::new(chain).http("localhost:3000").unwrap();
}
