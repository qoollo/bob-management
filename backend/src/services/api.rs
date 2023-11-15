use super::prelude::*;

/// Returns count of Physical Disks per status
///
#[cfg_attr(feature = "swagger",
    utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/disks/count",
        responses(
            (status = 200, body = DiskCount, content_type = "application/json", description = "Returns a list with count of physical disks per status"),
            (status = 401, description = "Unauthorized")
        ),
        security(("api_key" = []))
))]
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
        let mut disks_visited = HashSet::new();
        let (disks, space) = match res {
            Ok(d) => d,
            Err(err) => {
                tracing::warn!("couldn't finish request: tokio task failed. Err: {err}");
                continue;
            }
        };
        let space = match space {
            Ok(GetSpaceInfoResponse::SpaceInfo(space)) => space,
            Err(err) => {
                tracing::warn!("couldn't finish getSpace request. Err: {err}");
                continue;
            }
        };
        let disks = match disks {
            Ok(GetDisksResponse::AJSONArrayWithDisksAndTheirStates(disks)) => {
                let mut res = vec![];
                for disk in disks {
                    if disks_visited.insert(disk.name.clone()) {
                        res.push(disk);
                    }
                }
                res
            }
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
        let active_disks = disks.iter().filter(|disk| disk.is_active);
        for disk in active_disks {
            if let Some(&occupied_space) = space.occupied_disk_space_by_disk.get(&disk.name) {
                #[allow(clippy::cast_precision_loss)]
                match disk_status_from_space(&space, occupied_space) {
                    DiskStatus::Good => count[DiskStatusName::Good] += 1,
                    _ => count[DiskStatusName::Bad] += 1,
                }
            } else {
                count[DiskStatusName::Offline] += 1;
            }
        }
    }
    tracing::info!("total disks count: {count:?}");

    Json(count)
}

/// Get Nodes count per Status
///
#[cfg_attr(feature = "swagger", utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/nodes/count",
        responses(
            (status = 200, body = NodeCount, content_type = "application/json", description = "Node count list per status"),
            (status = 401, description = "Unauthorized")
        ),
        security(("api_key" = []))
    ))]
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

    let mut counter = 0;
    while let Some(res) = metrics.next().await {
        if let Ok(Ok(GetMetricsResponse::Metrics(metrics))) = res {
            tracing::info!("#{counter}: metrics received successfully");
            let metrics = Into::<TypedMetrics>::into(metrics);
            if is_bad_node(&metrics) {
                count[NodeStatusName::Bad] += 1;
            } else {
                count[NodeStatusName::Good] += 1;
            }
        } else {
            tracing::warn!("#{counter}: couldn't receive metrics from node");
            count[NodeStatusName::Offline] += 1;
        }
        counter += 1;
    }
    tracing::info!("total nodes per status count: {count:?}");

    Json(count)
}

/// Returns Total RPS on cluster
///
#[cfg_attr(feature = "swagger", utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/nodes/rps",
        responses(
            (status = 200, body = RPS, content_type = "application/json", description = "RPS list per operation on all nodes"),
            (status = 401, description = "Unauthorized")
        ),
        security(("api_key" = []))
    ))]
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
    let mut counter = 0;
    while let Some(res) = metrics.next().await {
        if let Ok(Ok(metrics)) = res {
            tracing::info!("#{counter}: metrics received successfully");

            let GetMetricsResponse::Metrics(metrics) = metrics;
            let metrics = Into::<TypedMetrics>::into(metrics);
            rps[Operation::Get] += metrics[RawMetricEntry::ClusterGrinderGetCountRate].value;
            rps[Operation::Delete] += metrics[RawMetricEntry::ClusterGrinderDeleteCountRate].value;
            rps[Operation::Exist] += metrics[RawMetricEntry::ClusterGrinderExistCountRate].value;
            rps[Operation::Put] += metrics[RawMetricEntry::ClusterGrinderPutCountRate].value;
        } else {
            tracing::warn!("#{counter}: couldn't receive metrics from node");
        }
        counter += 1;
    }
    tracing::info!("total rps: {rps:?}");

    Json(rps)
}

/// Return inforamtion about space on cluster
///
#[cfg_attr(feature = "swagger", utoipa::path(
        get,
        context_path = ApiV1::to_path(),
        path = "/nodes/space",
        responses(
            (status = 200, body = SpaceInfo, content_type = "application/json", description = "Cluster Space Information"),
            (status = 401, description = "Unauthorized")
        ),
        security(("api_key" = []))
    ))]
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
    let mut counter = 0;
    while let Some(res) = spaces.next().await {
        if let Ok(Ok(space)) = res {
            tracing::info!("#{counter}: space info received successfully");

            let GetSpaceInfoResponse::SpaceInfo(space) = space;
            total_space.total_disk += space.total_disk_space_bytes;
            total_space.free_disk += space.free_disk_space_bytes;
            total_space.used_disk += space.total_disk_space_bytes - space.free_disk_space_bytes;
            total_space.occupied_disk += space.occupied_disk_space_bytes;
        } else {
            tracing::warn!("#{counter}: couldn't receive space info from node");
        }
        counter += 1;
    }
    tracing::trace!("send response: {total_space:?}");

    Json(total_space)
}

#[allow(clippy::cast_precision_loss)]
fn is_bad_node(node_metrics: &TypedMetrics) -> bool {
    node_metrics[RawMetricEntry::BackendAlienCount].value != 0
        || node_metrics[RawMetricEntry::BackendCorruptedBlobCount].value != 0
        || node_metrics[RawMetricEntry::HardwareBobCpuLoad].value >= DEFAULT_MAX_CPU
        || (1.
            - (node_metrics[RawMetricEntry::HardwareTotalSpace].value
                - node_metrics[RawMetricEntry::HardwareFreeSpace].value) as f64
                / node_metrics[RawMetricEntry::HardwareTotalSpace].value as f64)
            < DEFAULT_MIN_FREE_SPACE
        || node_metrics[RawMetricEntry::HardwareBobVirtualRam]
            > node_metrics[RawMetricEntry::HardwareTotalRam]
}

#[allow(clippy::cast_precision_loss)]
fn disk_status_from_space(space: &dto::SpaceInfo, occupied_space: u64) -> DiskStatus {
    if ((space.total_disk_space_bytes - occupied_space) as f64
        / space.total_disk_space_bytes as f64)
        < DEFAULT_MIN_FREE_SPACE
    {
        DiskStatus::Bad(vec![DiskProblem::FreeSpaceRunningOut])
    } else {
        DiskStatus::Good
    }
}
