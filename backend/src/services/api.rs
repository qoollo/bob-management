use super::prelude::*;

// TODO: For methods, that requires information from all nodes (/disks/count, /nodes/rps, etc.),
// think of better method of returning info
// another thread that constantly updates info in period and cache the results?

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

/// Returns simple list of all known nodes
///
/// # Errors
///
/// This function will return an error if a call to the primary node will fail
#[cfg_attr(feature = "swagger", utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/nodes/list",
        responses(
            (
                status = 200, body = Vec<dto::Node>,
                content_type = "application/json",
                description = "Simple Node List"
            ),
            (status = 401, description = "Unauthorized")
        ),
        security(("api_key" = []))
    ))]
pub async fn get_nodes_list(
    Extension(client): Extension<HttpBobClient>,
) -> AxumResult<Json<Vec<dto::Node>>> {
    tracing::info!("get /nodes/list : {client:?}");
    fetch_nodes(client.api_main()).await.map(Json)
}

/// Returns simple list of all known vdisks
///
/// # Errors
///
/// This function will return an error if a call to the primary node will fail
#[cfg_attr(feature = "swagger", utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/vdisks/list",
        responses(
            (
                status = 200, body = Vec<dto::VDisk>,
                content_type = "application/json",
                description = "Simple Node List"
            ),
            (status = 401, description = "Unauthorized")
        ),
        security(("api_key" = []))
    ))]
pub async fn get_vdisks_list(
    Extension(client): Extension<HttpBobClient>,
) -> AxumResult<Json<Vec<dto::VDisk>>> {
    tracing::info!("get /vdisks/list : {client:?}");
    fetch_vdisks(client.api_main()).await.map(Json)
}
/// Returns vdisk inforamtion by their id
///
/// # Errors
///
/// This function will return an error if a call to the main node will fail or vdisk with
/// specified id not found
#[cfg_attr(feature = "swagger", utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/vdisks/{vdisk_id}",
        responses(
            (
                status = 200, body = VDisk,
                content_type = "application/json",
                description = "VDisk Inforamtion"
            ),
            (status = 401, description = "Unauthorized"),
            (status = 404, description = "VDisk not found"),
        ),
        security(("api_key" = []))
    ))]
pub async fn get_vdisk_info(
    Extension(client): Extension<HttpBobClient>,
    Path(vdisk_id): Path<u64>,
) -> AxumResult<Json<VDisk>> {
    tracing::info!("get /vdisks/{vdisk_id} : {client:?}");
    get_vdisk_by_id(&client, vdisk_id).await.map(Json)
}

/// Returns node inforamtion by their node name
///
/// # Errors
///
/// This function will return an error if a call to the specified node will fail or node with
/// specified name not found
#[cfg_attr(feature = "swagger", utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/nodes/{node_name}",
        responses(
            (
                status = 200, body = Node,
                content_type = "application/json",
                description = "Node Inforamtion"
            ),
            (status = 401, description = "Unauthorized"),
            (status = 404, description = "Node not found"),
        ),
        security(("api_key" = []))
    ))]
pub async fn get_node_info(
    Extension(client): Extension<HttpBobClient>,
    Path(node_name): Path<NodeName>,
) -> AxumResult<Json<Node>> {
    tracing::info!("get /nodes/{node_name} : {client:?}");
    let handle = Arc::new(
        client
            .api_secondary(&node_name)
            .cloned()
            .ok_or(StatusCode::NOT_FOUND)?,
    );

    let status = {
        let handle = handle.clone();
        tokio::spawn(async move { handle.get_status().await })
    };
    let metrics = {
        let handle = handle.clone();
        tokio::spawn(async move { handle.clone().get_metrics().await })
    };
    let space_info = {
        let handle = handle.clone();
        tokio::spawn(async move { handle.clone().get_space_info().await })
    };

    let Ok(Ok(GetStatusResponse::AJSONWithNodeInfo(status))) = status.await else {
        return Err(StatusCode::NOT_FOUND.into());
    };

    let mut vdisks: FuturesUnordered<_> = status
        .vdisks
        .iter()
        .flatten()
        .map(|vdisk| {
            let handle = client.clone();
            let id = vdisk.id as u64;
            tokio::spawn(async move { get_vdisk_by_id(&handle, id).await })
        })
        .collect();

    let mut node = Node {
        name: status.name.clone(),
        hostname: status.address.clone(),
        vdisks: vec![],
        status: NodeStatus::Offline,
        rps: None,
        alien_count: None,
        corrupted_count: None,
        space: None,
    };
    if let (
        Ok(Ok(GetMetricsResponse::Metrics(metric))),
        Ok(Ok(GetSpaceInfoResponse::SpaceInfo(space))),
    ) = (metrics.await, space_info.await)
    {
        let metric = Into::<TypedMetrics>::into(metric);
        node.status = NodeStatus::from_problems(NodeProblem::default_from_metrics(&metric));
        node.rps = Some(RPS::from_metrics(&metric));
        node.alien_count = Some(metric[RawMetricEntry::BackendAlienCount].value);
        node.corrupted_count = Some(metric[RawMetricEntry::BackendCorruptedBlobCount].value);
        node.space = Some(SpaceInfo::from(space));
    }

    while let Some(vdisk) = vdisks.next().await {
        if let Ok(Ok(vdisk)) = vdisk {
            node.vdisks.push(vdisk);
        } else {
            tracing::warn!("some warning"); //TODO
        }
    }

    Ok(Json(node))
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

// Bad function, 6 args :(
async fn process_vdisks_for_node(
    client: HttpBobClient,
    virt_disks: &[dto::VDisk],
    node_name: NodeName,
    all_disks: &HashMap<DiskName, IsActive>,
    nodes: &HashMap<&NodeName, &dto::Node>,
    partitions_count_on_vdisk: &HashMap<i32, usize>,
) -> Vec<VDisk> {
    let mut res_replicas = vec![];
    let mut res_vdisks = vec![];
    for (vdisk, replicas) in virt_disks
        .iter()
        .filter_map(|vdisk| vdisk.replicas.as_ref().map(|repl| (vdisk, repl)))
        .filter(|(_, replicas)| replicas.iter().any(|replica| replica.node == node_name))
    {
        for replica in replicas {
            res_replicas.push((
                vdisk.id,
                process_replica(&client, replica.clone(), all_disks, nodes).await,
            ));
        }
        res_vdisks.push(VDisk {
            id: vdisk.id as u64,
            status: if res_replicas
                .iter()
                .any(|(_, replica)| matches!(replica.status, ReplicaStatus::Offline(_)))
            {
                VDiskStatus::Bad
            } else {
                VDiskStatus::Good
            },
            partition_count: partitions_count_on_vdisk
                .get(&vdisk.id)
                .copied()
                .unwrap_or_default() as u64,
            replicas: res_replicas
                .iter()
                .filter(|(id, _)| id == &vdisk.id)
                .map(|(_, replica)| replica.clone())
                .collect(),
        });
    }

    res_vdisks
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
            (status = 200, body = TypedMetrics, content_type = "application/json", description = "Node's metrics"),
            (status = 401, description = "Unauthorized"),
            (status = 404, description = "Node Not Found")
        ),
        security(("api_key" = []))
    ))]
pub async fn raw_metrics_by_node(
    Extension(client): Extension<HttpBobClient>,
    Path(node_name): Path<NodeName>,
) -> AxumResult<Json<TypedMetrics>> {
    Ok(Json(
        fetch_metrics(
            &client
                .api_secondary(&node_name)
                .cloned()
                .ok_or(StatusCode::NOT_FOUND)?,
        )
        .await?
        .into(),
    ))
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
) -> AxumResult<Json<dto::NodeConfiguration>> {
    Ok(Json(
        fetch_configuration(
            &client
                .api_secondary(&node_name)
                .cloned()
                .ok_or(StatusCode::NOT_FOUND)?,
        )
        .await?,
    ))
}

/// Get Detailed Information on Node
///
/// # Errors
///
/// This function will return an error if the server was unable to get node'a client
/// or one of the requests to get information from the node fails
#[cfg_attr(feature = "swagger", utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/nodes/{node_name}/detailed",
        params (
            ("id", description = "Node's ID")
        ),
        responses(
            (status = 200, body = DetailedNode, content_type = "application/json", description = "Detailed Node information"),
            (status = 401, description = "Unauthorized"),
            (status = 404, description = "Node Not Found")
        ),
        security(("api_key" = []))
    ))]
pub async fn get_detailed_node_info(
    Extension(client): Extension<HttpBobClient>,
    Path(node_name): Path<NodeName>,
) -> AxumResult<Json<DetailedNode>> {
    let mut all_disks: FuturesUnordered<_> = client
        .cluster()
        .map(move |node| {
            let handle = node.clone();
            tokio::spawn(async move { handle.get_disks().await })
        })
        .collect();

    let node_client = get_client_by_node(&client, node_name.clone()).await?;

    let virt_disks = fetch_vdisks(node_client.as_ref()).await?;

    let mut all_partitions: FuturesUnordered<_> = virt_disks
        .iter()
        .map(|vdisk| {
            let id = vdisk.id;
            let handle = client.api_main().clone();
            tokio::spawn(async move { (id, handle.get_partitions(id).await) })
        })
        .collect();

    let metrics = fetch_metrics(node_client.as_ref()).await?;
    let typed_metrics: TypedMetrics = metrics.clone().into();

    let space = fetch_space_info(node_client.as_ref()).await?;
    let node_status = fetch_node_status(node_client.as_ref()).await?;
    let disks = fetch_disks(node_client.as_ref()).await?;

    let res_disks = proccess_disks(&disks, &space, &metrics);
    let nodes = fetch_nodes(node_client.as_ref()).await?;

    let nodes = nodes
        .iter()
        .map(|node| (&node.name, node))
        .collect::<HashMap<&NodeName, &dto::Node>>();

    let mut proc_disks = HashMap::new();
    while let Some(disks) = all_disks.next().await {
        let Ok(Ok(GetDisksResponse::AJSONArrayWithDisksAndTheirStates(disks))) = disks else {
            tracing::error!("couldn't get disk inforamtion from node");
            continue;
        };
        for disk in disks {
            proc_disks.insert(disk.name, disk.is_active);
        }
    }

    let mut res_partitions = HashMap::new();
    while let Some(partitions) = all_partitions.next().await {
        let Ok((id, Ok(GetPartitionsResponse::NodeInfoAndJSONArrayWithPartitionsInfo(partitions)))) =
            partitions
        else {
            // tracing::error!("couldn't get Partition inforamtion from node"); // Too noisy
            continue;
        };
        if let Some(partitions) = partitions.partitions {
            res_partitions.insert(id, partitions.len());
        }
    }

    let virtual_disks = process_vdisks_for_node(
        client,
        &virt_disks,
        node_name,
        &proc_disks,
        &nodes,
        &res_partitions,
    )
    .await;

    let mut rps = RPS::new();

    let status = NodeStatus::from_problems(NodeProblem::default_from_metrics(&typed_metrics));

    rps[Operation::Get] = typed_metrics[RawMetricEntry::PearlGetCountRate].value;
    rps[Operation::Put] = typed_metrics[RawMetricEntry::PearlPutCountRate].value;
    rps[Operation::Exist] = typed_metrics[RawMetricEntry::PearlExistCountRate].value;
    rps[Operation::Delete] = typed_metrics[RawMetricEntry::PearlDeleteCountRate].value;

    let result = Json(DetailedNode {
        name: node_status.name,
        hostname: node_status.address,
        vdisks: virtual_disks,
        status,
        metrics: DetailedNodeMetrics {
            rps,
            alien_count: typed_metrics[RawMetricEntry::BackendAlienCount].value,
            corrupted_count: typed_metrics[RawMetricEntry::BackendCorruptedBlobCount].value,
            space: SpaceInfo {
                total_disk: space.total_disk_space_bytes,
                free_disk: space.total_disk_space_bytes - space.used_disk_space_bytes,
                used_disk: space.used_disk_space_bytes,
                occupied_disk: space.occupied_disk_space_bytes,
            },
            cpu_load: typed_metrics[RawMetricEntry::HardwareBobCpuLoad].value,
            total_ram: typed_metrics[RawMetricEntry::HardwareTotalRam].value,
            used_ram: typed_metrics[RawMetricEntry::HardwareUsedRam].value,
            descr_amount: typed_metrics[RawMetricEntry::HardwareDescrAmount].value,
        },
        disks: res_disks,
    });
    tracing::trace!("send response: {result:?}");

    Ok(result)
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
