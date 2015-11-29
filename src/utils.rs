
use iron::prelude::*;
use router::Router;
use urlencoded::UrlEncodedQuery;

/// A convenient extenstion for `Request`.
pub trait IronRequestExt {
    /// Returns path parameter or empty string if parameter is not exist.
    ///
    /// # Panics
    /// Panics if a `Router` has not been registered with Iron.
    fn path_param(&self, key: &str) -> &str;

    /// Returns query parameter or empty string if parameter is not exist.
    fn query_param(&mut self, key: &str) -> &str;
}

impl<'a, 'b> IronRequestExt for Request<'a, 'b> {
    fn path_param(&self, key: &str) -> &str {
        self.extensions.get::<Router>().unwrap().find(key).unwrap_or("")
    }

    fn query_param(&mut self, key: &str) -> &str {
        self.get_ref::<UrlEncodedQuery>().ok()
            .and_then(|m| m.get(key).and_then(|v| v.first()))
            .map_or("", |s| s.as_ref())
    }
}
