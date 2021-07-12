pub use crate::services::bob::node::Node;
use std::net::SocketAddr;

mod node;

#[derive(Debug)]
pub enum BobError {
    Unreachable,
    UnexpectedResponse,
}

pub async fn get_nodes(addr: &SocketAddr) -> Result<Vec<Node>, BobError> {
    let resp = perform_request(get_addr(addr, "/nodes")).await?;
    let result = resp
        .json::<Vec<Node>>()
        .await
        .map_err(|_| BobError::UnexpectedResponse)?;
    Ok(result)
}

pub async fn is_active(node: &Node) -> Result<bool, BobError> {
    let api_addr = node.get_api_addr();
    let _resp = perform_request(get_addr(&api_addr, "/status")).await?;
    Ok(true) // If we reached this point request was completed
}

async fn perform_request(addr: String) -> Result<reqwest::Response, BobError> {
    reqwest::get(addr).await.map_err(|e| {
        error!("{:?}", e);
        BobError::Unreachable
    })
}

fn get_addr(node_addr: &SocketAddr, absolute_path: &str) -> String {
    let mut result = String::from("http://");
    result.push_str(&node_addr.to_string());
    result.push_str(absolute_path);
    result
}
