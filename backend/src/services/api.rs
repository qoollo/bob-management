use super::prelude::*;

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
