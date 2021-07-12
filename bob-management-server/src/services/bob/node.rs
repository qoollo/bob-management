use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct Node {
    pub name: String,
    pub address: SocketAddr,
    pub vdisks: Vec<VDisk>,
}

impl Node {
    pub fn get_api_addr(&self) -> SocketAddr {
        let mut clone = self.address.clone();
        clone.set_port(8000);
        clone
    }
}

#[derive(Deserialize)]
pub struct VDisk {
    pub id: u32,
    pub replicas: Vec<Replica>,
}

#[derive(Deserialize)]
pub struct Replica {
    pub node: String,
    pub disk: String,
    pub path: String,
}