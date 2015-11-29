
use iron::prelude::*;
use router::Router;
use urlencoded::UrlEncodedQuery;

/// A convenient extenstion for `Request`.
///
/// Returns string for router parameter.
pub trait GetParam {
    /// Returns string for router parameter or empty string if parameter is not
    /// exist.
    ///
    /// # Panics
    /// Panics if `Router` is not presented in request's extensions.
    fn param(&self, key: &str) -> &str;
}

impl<'a, 'b> GetParam for Request<'a, 'b> {
    fn param(&self, key: &str) -> &str {
        self.extensions.get::<Router>().unwrap().find(key).unwrap_or("")
    }
}

/// A convenient extenstion for `Request`.
///
/// Returns string for query key.
pub trait GetQuery {
    /// Returns string for query key or empty string if key isn't exist.
    fn query(&mut self, key: &str) -> &str;
}

impl<'a, 'b> GetQuery for Request<'a, 'b> {
    fn query(&mut self, key: &str) -> &str {
        self.get_ref::<UrlEncodedQuery>().ok()
            .and_then(|m| m.get(key).and_then(|v| v.first()))
            .map_or("", |s| s.as_ref())
    }
}
