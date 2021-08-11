use crate::services::bob::{get_nodes, is_active, Node};
use crate::storages::session_data_storage::*;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)] // TODO add proper eq and ord
pub struct NodeDto {
    name: String,
    addr: String,
    active: bool,
    vdisks: Vec<VDiskDto>,
}

#[derive(Serialize)]
pub struct VDiskDto {
    id: u32,
    replicas: Vec<ReplicaDto>,
}

#[derive(Serialize)]
pub struct ReplicaDto {
    disk: String,
    path: String,
}

impl NodeDto {
    fn new(node: &Node, active: bool) -> Self {
        let vdisks = node
            .vdisks
            .iter()
            .filter(|vd| vd.replicas.iter().any(|r| r.node == node.name))
            .map(|vd| {
                let replicas = vd
                    .replicas
                    .iter()
                    .filter(|r| r.node == node.name)
                    .map(|r| ReplicaDto {
                        disk: r.disk.clone(),
                        path: r.path.clone(),
                    })
                    .collect();
                VDiskDto {
                    id: vd.id,
                    replicas,
                }
            })
            .collect();
        NodeDto {
            name: node.name.clone(),
            addr: node.address.to_string(),
            active,
            vdisks,
        }
    }
}

#[derive(Serialize)]
pub struct IndexContext {
    nodes: Vec<NodeDto>,
    error: Option<String>,
}

impl IndexContext {
    fn from_nodes(nodes: Vec<NodeDto>) -> Self {
        Self { nodes, error: None }
    }

    fn from_error(error: String) -> Self {
        Self {
            error: Some(error),
            nodes: vec![],
        }
    }
}

#[get("/cluster")]
pub async fn get(cookie_jar: &CookieJar<'_>) -> Result<Template, Redirect> {
    if let Some(addr) = cookie_jar.find_cluster_addr() {
        let nodes_from_bob = get_nodes(&addr).await;
        let context = match nodes_from_bob {
            Ok(nodes) => {
                let mut result = vec![];
                for node in &nodes {
                    let active = is_active(node).await.unwrap_or(false);
                    result.push(NodeDto::new(node, active));
                }
                result.sort_by(|x, y| x.name.cmp(&y.name));
                IndexContext::from_nodes(result)
            }
            Err(e) => IndexContext::from_error(format!("{:?}", e)),
        };

        Ok(Template::render("cluster/index", context))
    } else {
        Err(Redirect::to(uri!(crate::routes::auth::login::get)))
    }
}
