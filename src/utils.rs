
use iron::prelude::*;
use router::Router;

pub trait GetParam {
    fn param(&self, param: &str) -> &str;
}

impl<'a, 'b> GetParam for Request<'a, 'b> {
    fn param(&self, param: &str) -> &str {
        self.extensions.get::<Router>().unwrap().find(param).unwrap_or("")
    }
}
