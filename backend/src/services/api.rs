use axum::extract::Path;

use crate::{
    connector::dto::{MetricsSnapshotModel, NodeConfiguration},
    models::bob::{DiskName, IsActive},
};

use super::{
    auth::HttpClient,
    methods::{fetch_configuration, fetch_metrics, fetch_nodes, fetch_vdisks},
    prelude::*,
};

/// Returns count of Physical Disks per status
#[cfg_attr(all(feature = "swagger", debug_assertions),
    utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/disks/count",
        responses(
            (
                status = 200, body = DiskCount,
                content_type = "application/json", 
                description = "Returns a list with count of physical disks per status"
            ),
            (status = 401, description = "Unauthorized")
        ),
        security(("api_key" = []))
))]
#[tracing::instrument(ret, skip(client), level = "info", fields(method = "GET"))]
pub async fn get_disks_count(Extension(client): Extension<HttpBobClient>) -> Json<DiskCount> {
    tracing::info!("get /disks/count : {:?}", client);

    let mut space: FuturesUnordered<_> = client
        .cluster()
        .map(move |node| {
            let handle = node.clone();
            tokio::spawn(async move { (handle.get_disks().await, handle.get_space_info().await) })
        })
        .collect();

    let mut count = DiskCount::new();

    while let Some(res) = space.next().await {
        let Ok((disks, space)) = res else {
            tracing::warn!("couldn't finish request: tokio task failed. Err: {res:?}");
            continue;
        };
        let Ok(GetSpaceInfoResponse::SpaceInfo(space)) = space else {
            tracing::warn!("couldn't finish getSpace request. {space:?}");
            continue;
        };
        let disks = match disks {
            Ok(GetDisksResponse::AJSONArrayWithDisksAndTheirStates(disks)) => disks,
            Ok(GetDisksResponse::PermissionDenied(err)) => {
                count[DiskStatusName::Offline] += 1;
                tracing::warn!("Permission Denied. Err: {err:?}");
                continue;
            }
            Err(err) => {
                count[DiskStatusName::Offline] += 1;
                tracing::warn!("couldn't finish getDisks request. Err: {err}");
                continue;
            }
        };
        let mut active = 0;
        disks.iter().filter(|disk| disk.is_active).for_each(|disk| {
            active += 1;
            match DiskStatus::from_space_info(&space, &disk.name) {
                DiskStatus::Good => count[DiskStatusName::Good] += 1,
                DiskStatus::Offline => count[DiskStatusName::Offline] += 1,
                DiskStatus::Bad(_) => count[DiskStatusName::Bad] += 1,
            }
        });
        count[DiskStatusName::Offline] = (disks.len() - active) as u64;
    }
    tracing::info!("total disks count: {count:?}");

    Json(count)
}

/// Get Nodes count per Status
#[cfg_attr(all(feature = "swagger", debug_assertions),
    utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/nodes/count",
        responses(
            (
                status = 200, body = NodeCount,
                content_type = "application/json",
                description = "Node count list per status"
            ),
            (status = 401, description = "Unauthorized")
        ),
        security(("api_key" = []))
))]
#[tracing::instrument(ret, skip(client), level = "info", fields(method = "GET"))]
pub async fn get_nodes_count(Extension(client): Extension<HttpBobClient>) -> Json<NodeCount> {
    tracing::info!("get /nodes/count : {:?}", client);

    let mut metrics: FuturesUnordered<_> = client
        .cluster()
        .map(move |node| {
            let handle = node.clone();
            tokio::spawn(async move { handle.get_metrics().await })
        })
        .collect();

    let mut count = NodeCount::new();

    while let Some(res) = metrics.next().await {
        if let Ok(Ok(GetMetricsResponse::Metrics(metrics))) = res {
            tracing::trace!("metrics received successfully");
            if Into::<TypedMetrics>::into(metrics).is_bad_node() {
                count[NodeStatusName::Bad] += 1;
            } else {
                count[NodeStatusName::Good] += 1;
            }
        } else {
            tracing::warn!("couldn't receive metrics from node"); // TODO: Some better message
            count[NodeStatusName::Offline] += 1;
        }
    }
    tracing::info!("total nodes per status count: {count:?}");

    Json(count)
}

/// Returns Total RPS on cluster
#[cfg_attr(all(feature = "swagger", debug_assertions),
    utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/nodes/rps",
        responses(
            (
                status = 200, body = RPS,
                content_type = "application/json",
                description = "RPS list per operation on all nodes"
            ),
            (status = 401, description = "Unauthorized")
        ),
        security(("api_key" = []))
))]
#[tracing::instrument(ret, skip(client), level = "info", fields(method = "GET"))]
pub async fn get_rps(Extension(client): Extension<HttpBobClient>) -> Json<RPS> {
    tracing::info!("get /nodes/rps : {:?}", client);

    let mut metrics: FuturesUnordered<_> = client
        .cluster()
        .map(move |node| {
            let handle = node.clone();
            tokio::spawn(async move { handle.get_metrics().await })
        })
        .collect();

    let mut rps = RPS::new();
    while let Some(res) = metrics.next().await {
        if let Ok(Ok(metrics)) = res {
            tracing::info!("metrics received successfully");
            let GetMetricsResponse::Metrics(metrics) = metrics;
            rps += RPS::from_metrics(&metrics.into());
        } else {
            tracing::warn!("couldn't receive metrics from node"); // TODO: Some better message
        }
    }
    tracing::info!("total rps: {rps:?}");

    Json(rps)
}

/// Return inforamtion about space on cluster
#[cfg_attr(all(feature = "swagger", debug_assertions), 
    utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/nodes/space",
        responses(
            (status = 200, body = SpaceInfo, content_type = "application/json", description = "Cluster Space Information"),
            (status = 401, description = "Unauthorized")
        ),
        security(("api_key" = []))
))]
#[tracing::instrument(ret, skip(client), level = "info", fields(method = "GET"))]
pub async fn get_space(Extension(client): Extension<HttpBobClient>) -> Json<SpaceInfo> {
    tracing::info!("get /space : {:?}", client);
    let mut spaces: FuturesUnordered<_> = client
        .cluster()
        .map(move |node| {
            let handle = node.clone();
            tokio::spawn(async move { handle.get_space_info().await })
        })
        .collect();

    let mut total_space = SpaceInfo::default();
    while let Some(res) = spaces.next().await {
        if let Ok(Ok(space)) = res {
            tracing::info!("space info received successfully");
            let GetSpaceInfoResponse::SpaceInfo(space) = space;
            total_space.total_disk += space.total_disk_space_bytes;
            total_space.free_disk += space.free_disk_space_bytes;
            total_space.used_disk += space.total_disk_space_bytes - space.free_disk_space_bytes;
            total_space.occupied_disk += space.occupied_disk_space_bytes;
        } else {
            tracing::warn!("couldn't receive space info from node"); // Some better message
        }
    }
    tracing::trace!("send response: {total_space:?}");

    Json(total_space)
}

/// Returns list of all known nodes
///
/// # Errors
///
/// This function will return an error if one of the requests to get list of virtual disks or nodes fails
#[cfg_attr(feature = "swagger", utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/nodes",
        responses(
            (status = 200, body = Vec<Node>, content_type = "application/json", description = "Node List"),
            (status = 401, description = "Unauthorized")
        ),
        security(("api_key" = []))
    ))]
pub async fn get_nodes(Extension(client): Extension<HttpBobClient>) -> AxumResult<Json<Vec<Node>>> {
    tracing::info!("get /nodes : {client:?}");

    let mut metrics: FuturesUnordered<_> = client
        .cluster()
        .map(move |node| {
            let handle = node.clone();
            tokio::spawn(async move {
                (
                    handle.get_status().await,
                    handle.get_metrics().await,
                    handle.get_space_info().await,
                )
            })
        })
        .collect();

    let vdisks = get_vdisks(Extension(client.clone())).await.map_err(|err| {
        tracing::error!("{err:?}");
        APIError::RequestFailed
    })?;

    let nodes = fetch_nodes(client.api_main()).await?;
    let vdisks: HashMap<u64, &VDisk> = vdisks.iter().map(|vdisk| (vdisk.id, vdisk)).collect();

    let nodes: HashMap<&NodeName, &dto::Node> =
        nodes.iter().map(move |node| (&node.name, node)).collect();

    let mut res = nodes
        .iter()
        .map(|(&name, node)| {
            let vdisks = node
                .vdisks
                .as_ref()
                .map_or_else(std::vec::Vec::new, |node_vdisks| {
                    node_vdisks
                        .iter()
                        .filter_map(|vdisk| vdisks.get(&(vdisk.id as u64)))
                        .map(|vdisk| (*vdisk).clone())
                        .collect()
                });
            (
                name,
                Node {
                    name: name.clone(),
                    hostname: node.address.clone(),
                    vdisks,
                    status: NodeStatus::Offline,
                    rps: None,
                    alien_count: None,
                    corrupted_count: None,
                    space: None,
                },
            )
        })
        .collect::<HashMap<&NodeName, Node>>();

    let mut counter = 0;
    while let Some(fut) = metrics.next().await {
        let Ok((
            Ok(GetStatusResponse::AJSONWithNodeInfo(status)),
            Ok(GetMetricsResponse::Metrics(metric)),
            Ok(GetSpaceInfoResponse::SpaceInfo(space)),
        )) = fut
        else {
            tracing::warn!("couldn't finish task: tokio task failed.");
            continue;
        };
        if let Some(node) = res.get_mut(&status.name.to_string()) {
            let metric = Into::<TypedMetrics>::into(metric);
            tracing::info!("#{counter}: received metrics successfully.");
            node.status = NodeStatus::from_problems(NodeProblem::default_from_metrics(&metric));
            node.rps = Some(
                metric[RawMetricEntry::PearlGetCountRate].value
                    + metric[RawMetricEntry::PearlPutCountRate].value
                    + metric[RawMetricEntry::PearlExistCountRate].value
                    + metric[RawMetricEntry::PearlDeleteCountRate].value,
            );
            node.alien_count = Some(metric[RawMetricEntry::BackendAlienCount].value);
            node.corrupted_count = Some(metric[RawMetricEntry::BackendCorruptedBlobCount].value);
            node.space = Some(SpaceInfo {
                total_disk: space.total_disk_space_bytes,
                free_disk: space.total_disk_space_bytes - space.used_disk_space_bytes,
                used_disk: space.used_disk_space_bytes,
                occupied_disk: space.occupied_disk_space_bytes,
            });
        }
        counter += 1;
    }
    tracing::trace!("send response: {res:?}");

    Ok(Json(res.values().cloned().collect()))
}

/// Get Virtual Disks
///
/// # Errors
///
/// This function will return an error if one of the requests to get list of vdisks or nodes fails
#[cfg_attr(feature = "swagger", utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/vdisks",
        responses(
            (status = 200, body = Vec<VDisk>, content_type = "application/json", description = "Virtual disks list"),
            (status = 401, description = "Unauthorized")
        ),
        security(("api_key" = []))
    ))]
pub async fn get_vdisks(
    Extension(client): Extension<HttpBobClient>,
) -> AxumResult<Json<Vec<VDisk>>> {
    tracing::info!("get /vdisks : {client:?}");

    let mut disks: FuturesUnordered<_> = client
        .cluster()
        .map(move |node| {
            let handle = node.clone();
            tokio::spawn(async move { (handle.get_status().await, handle.get_disks().await) })
        })
        .collect();

    let api = client.api_main();
    let nodes = fetch_nodes(api).await?;
    let virt_disks = fetch_vdisks(api).await?;

    let nodes: HashMap<&NodeName, &dto::Node> =
        nodes.iter().map(|node| (&node.name, node)).collect();

    let mut res_disks = HashMap::new();
    while let Some(res) = disks.next().await {
        if let Ok((
            Ok(GetStatusResponse::AJSONWithNodeInfo(status)),
            Ok(GetDisksResponse::AJSONArrayWithDisksAndTheirStates(disks)),
        )) = res
        {
            let mut map = HashMap::new();
            for disk in disks {
                map.insert(disk.name, disk.is_active);
            }
            res_disks.insert(status.name, map);
        } else {
            tracing::warn!("couldn't receive node's space info");
        }
    }

    let mut res = vec![];

    for vdisk in virt_disks {
        let replicas = if let Some(replicas) = vdisk.replicas {
            let mut res = vec![];
            for replica in replicas {
                res.push(if let Some(disks) = res_disks.get(&replica.node) {
                    process_replica(&client, replica, disks, &nodes).await
                } else {
                    Replica {
                        node: replica.node,
                        disk: replica.disk,
                        path: replica.path,
                        status: ReplicaStatus::Offline(vec![ReplicaProblem::NodeUnavailable]),
                    }
                });
            }
            res
        } else {
            vec![]
        };
        let count = replicas
            .iter()
            .filter(|replica| matches!(replica.status, ReplicaStatus::Offline(_)))
            .count();
        let status = if count == 0 {
            VDiskStatus::Good
        } else if count == replicas.len() {
            VDiskStatus::Offline
        } else {
            VDiskStatus::Bad
        };
        let part = client.api_main().get_partitions(vdisk.id).await.ok();
        let partition_count =
            if let Some(GetPartitionsResponse::NodeInfoAndJSONArrayWithPartitionsInfo(part)) = part
            {
                part.partitions.unwrap_or_default().len()
            } else {
                0
            } as u64;
        res.push(VDisk {
            id: vdisk.id as u64,
            status,
            partition_count,
            replicas,
        });
    }
    tracing::trace!("send response: {res:?}");

    Ok(Json(res))
}

async fn process_replica(
    client: &HttpBobClient,
    replica: dto::Replica,
    disks: &HashMap<DiskName, IsActive>,
    nodes: &HashMap<&NodeName, &dto::Node>,
) -> Replica {
    let mut status = ReplicaStatus::Good;
    if let Some(disk_state) = disks.get(&replica.disk) {
        if !disk_state {
            status = ReplicaStatus::Offline(vec![ReplicaProblem::DiskUnavailable]);
        }
    } else {
        status = ReplicaStatus::Offline(vec![ReplicaProblem::DiskUnavailable]);
    }

    if let Some(node) = nodes.get(&replica.node) {
        if !is_node_online(client, node).await {
            status = match status {
                ReplicaStatus::Good => {
                    ReplicaStatus::Offline(vec![ReplicaProblem::DiskUnavailable])
                }
                ReplicaStatus::Offline(mut problems) => {
                    problems.push(ReplicaProblem::NodeUnavailable);
                    ReplicaStatus::Offline(problems)
                }
            }
        }
    } else {
        status = match status {
            ReplicaStatus::Good => ReplicaStatus::Offline(vec![ReplicaProblem::DiskUnavailable]),
            ReplicaStatus::Offline(mut problems) => {
                problems.push(ReplicaProblem::NodeUnavailable);
                ReplicaStatus::Offline(problems)
            }
        }
    }

    Replica {
        node: replica.node,
        disk: replica.disk,
        path: replica.path,
        status,
    }
}

async fn is_node_online(client: &HttpBobClient, node: &dto::Node) -> bool {
    (client.probe_socket(&node.name).await).map_or(false, |code| code == StatusCode::OK)
}

fn proccess_disks(
    disks: &[dto::DiskState],
    space: &dto::SpaceInfo,
    metrics: &dto::MetricsSnapshotModel,
) -> Vec<Disk> {
    let mut res_disks = vec![];
    let mut visited_disks = HashSet::new();
    for disk in disks {
        if !visited_disks.insert(disk.name.clone()) {
            tracing::warn!(
                "disk {} with path {} duplicated, skipping...",
                disk.name,
                disk.path
            );
            continue;
        }
        res_disks.push(Disk {
            name: disk.name.clone(),
            path: disk.path.clone(),
            status: DiskStatus::from_space_info(space, &disk.name),
            total_space: space.total_disk_space_bytes,
            used_space: space
                .occupied_disk_space_by_disk
                .get(&disk.name.clone())
                .copied()
                .unwrap_or_default(),
            iops: metrics
                .metrics
                .get(&format!("hardware.disks.{:?}_iops", disk.name))
                .cloned()
                .unwrap_or_default()
                .value,
        });
    }

    res_disks
}

/// Get Raw Metrics from Node
///
/// # Errors
///
/// This function will return an error if the server was unable to get node'a client or the request to get metrics fails
#[cfg_attr(feature = "swagger", utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/nodes/{node_name}/metrics",
        responses(
            (status = 200, body = MetricsSnapshotModel, content_type = "application/json", description = "Node's metrics"),
            (status = 401, description = "Unauthorized"),
            (status = 404, description = "Node Not Found")
        ),
        security(("api_key" = []))
    ))]
pub async fn raw_metrics_by_node(
    Extension(client): Extension<HttpBobClient>,
    Path(node_name): Path<NodeName>,
) -> AxumResult<Json<MetricsSnapshotModel>> {
    let client = get_client_by_node(&client, node_name).await?;

    Ok(Json(fetch_metrics(client.as_ref()).await?))
}

/// Get Configuration from Node
///
/// # Errors
///
/// This function will return an error if the server was unable to get node'a client or the request to get configuration fails
#[cfg_attr(feature = "swagger", utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/nodes/{node_name}/configuration",
        responses(
            (status = 200, body = NodeConfiguration, content_type = "application/json", description = "Node's configuration"),
            (status = 401, description = "Unauthorized"),
            (status = 404, description = "Node Not Found")
        ),
        security(("api_key" = []))
    ))]
pub async fn raw_configuration_by_node(
    Extension(client): Extension<HttpBobClient>,
    Path(node_name): Path<NodeName>,
) -> AxumResult<Json<NodeConfiguration>> {
    let client = get_client_by_node(&client, node_name).await?;

    Ok(Json(fetch_configuration(client.as_ref()).await?))
}

async fn get_client_by_node(
    client: &HttpBobClient,
    node_name: NodeName,
) -> AxumResult<Arc<HttpClient>> {
    let nodes = fetch_nodes(client.api_main()).await?;

    let node = nodes
        .iter()
        .find(|node| node.name == node_name)
        .ok_or_else(|| {
            tracing::error!("Couldn't find specified node");
            APIError::RequestFailed
        })?;

    client
        .cluster_with_addr()
        .get(&node.name)
        .ok_or_else(|| {
            tracing::error!("Couldn't find specified node");
            APIError::RequestFailed.into()
        })
        .cloned()
}
