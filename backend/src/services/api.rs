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
    tracing::info!("get /nodes/{node_name}/detailed : {client:?}");
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
    let disks = {
        let handle = handle.clone();
        tokio::spawn(async move { handle.clone().get_disks().await })
    };

    let Ok(Ok(GetStatusResponse::AJSONWithNodeInfo(status))) = status.await else {
        return Err(StatusCode::NOT_FOUND.into());
    };

    let mut node = DetailedNode {
        name: status.name,
        hostname: status.address,
        ..Default::default()
    };
    let mut virt_disks: FuturesUnordered<_> = status
        .vdisks
        .iter()
        .flatten()
        .map(|vdisk| {
            let handle = client.clone();
            let id = vdisk.id as u64;
            tokio::spawn(async move { get_vdisk_by_id(&handle, id).await })
        })
        .collect();

    if let (
        Ok(Ok(GetMetricsResponse::Metrics(raw_metrics))),
        Ok(Ok(GetSpaceInfoResponse::SpaceInfo(raw_space))),
    ) = (metrics.await, space_info.await)
    {
        if let Ok(Ok(GetDisksResponse::AJSONArrayWithDisksAndTheirStates(disks))) = disks.await {
            node.disks = disks
                .into_iter()
                .map(|disk| Disk::from_metrics(disk.name, disk.path, &raw_metrics, &raw_space))
                .collect();
        }
        let metrics = Into::<TypedMetrics>::into(raw_metrics);
        node.status = NodeStatus::from_problems(NodeProblem::default_from_metrics(&metrics));
        node.metrics = DetailedNodeMetrics::from_metrics(&metrics, raw_space.into());
    }

    while let Some(vdisk) = virt_disks.next().await {
        if let Ok(Ok(vdisk)) = vdisk {
            node.vdisks.push(vdisk);
        } else {
            tracing::warn!("some warning"); //TODO
        }
    }

    tracing::trace!("send response: {node:?}");

    Ok(Json(node))
}
