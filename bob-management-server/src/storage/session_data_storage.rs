use rocket::http::{Cookie, CookieJar};
use std::net::SocketAddr;
use time::Duration;

const ADDR_COOKIE_NAME: &'static str = "bob-cluster-addr";

pub trait SessionDataStorage {
    fn save_cluster_addr(&self, addr: SocketAddr) -> bool;
    fn find_cluster_addr(&self) -> Option<SocketAddr>;
}

impl SessionDataStorage for CookieJar<'_> {
    fn save_cluster_addr(&self, addr: SocketAddr) -> bool {
        let cookie = Cookie::build(ADDR_COOKIE_NAME, addr.to_string())
            .max_age(Duration::days(365))
            .finish();
        self.add_private(cookie);
        true
    }

    fn find_cluster_addr(&self) -> Option<SocketAddr> {
        self.get_private(ADDR_COOKIE_NAME)
            .and_then(|c| c.value().parse().ok())
    }
}
