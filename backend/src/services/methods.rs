use super::prelude::*;

/// Fetches metrics from `ApiNoContext` instance.
///
/// # Errors
///
/// This function will return an error if the request to the specified client failed
pub async fn fetch_metrics<
    Context: Send + Sync,
    ApiInterface: ApiNoContext<Context> + Send + Sync,
>(
    client: &ApiInterface,
) -> AxumResult<dto::MetricsSnapshotModel> {
    let GetMetricsResponse::Metrics(metrics) = client.get_metrics().await.map_err(|err| {
        tracing::error!("{err}");
        APIError::RequestFailed
    })?;

    Ok(metrics)
}

/// Fetches vdisks information from `ApiNoContext` instance.
///
/// # Errors
///
/// This function will return an error if the request to the specified client failed or the invalid
/// status code was received
pub async fn fetch_vdisks<
    Context: Send + Sync,
    ApiInterface: ApiNoContext<Context> + Send + Sync,
>(
    client: &ApiInterface,
) -> AxumResult<Vec<dto::VDisk>> {
    let GetVDisksResponse::AJSONArrayOfVdisksInfo(virt_disks) =
        client.get_v_disks().await.map_err(|err| {
            tracing::error!("{err}");
            APIError::RequestFailed
        })?
    else {
        return Err(APIError::InvalidStatusCode(StatusCode::FORBIDDEN).into());
    };

    Ok(virt_disks)
}

/// Fetches space information from `ApiNoContext` instance.
///
/// # Errors
///
/// This function will return an error if .
/// This function will return an error if the request to the specified client failed
pub async fn fetch_space_info<
    Context: Send + Sync,
    ApiInterface: ApiNoContext<Context> + Send + Sync,
>(
    client: &ApiInterface,
) -> AxumResult<dto::SpaceInfo> {
    let GetSpaceInfoResponse::SpaceInfo(space) = client.get_space_info().await.map_err(|err| {
        tracing::error!("{err}");
        APIError::RequestFailed
    })?;

    Ok(space)
}

/// Fetches node status information from `ApiNoContext` instance.
///
/// # Errors
///
/// This function will return an error if the request to the specified client failed
pub async fn fetch_node_status<
    Context: Send + Sync,
    ApiInterface: ApiNoContext<Context> + Send + Sync,
>(
    client: &ApiInterface,
) -> AxumResult<dto::Node> {
    let GetStatusResponse::AJSONWithNodeInfo(node_status) =
        client.get_status().await.map_err(|err| {
            tracing::error!("{err}");
            APIError::RequestFailed
        })?;

    Ok(node_status)
}

/// Fetches disk information on some node from `ApiNoContext` instance.
///
/// # Errors
///
/// This function will return an error if the request to the specified client failed or the invalid
/// status code was received
pub async fn fetch_disks<
    Context: Send + Sync,
    ApiInterface: ApiNoContext<Context> + Send + Sync,
>(
    client: &ApiInterface,
) -> AxumResult<Vec<dto::DiskState>> {
    let GetDisksResponse::AJSONArrayWithDisksAndTheirStates(disks) =
        client.get_disks().await.map_err(|err| {
            tracing::error!("{err}");
            APIError::RequestFailed
        })?
    else {
        tracing::error!(
            "client received invalid status code: {}",
            StatusCode::FORBIDDEN
        );
        return Err(APIError::InvalidStatusCode(StatusCode::FORBIDDEN).into());
    };

    Ok(disks)
}

/// Fetches configuration from `ApiNoContext` instance.
///
/// # Errors
///
/// This function will return an error if the request to the specified client failed or the invalid
/// status code was received
pub async fn fetch_configuration<
    Context: Send + Sync,
    ApiInterface: ApiNoContext<Context> + Send + Sync,
>(
    client: &ApiInterface,
) -> AxumResult<dto::NodeConfiguration> {
    let GetConfigurationResponse::ConfigurationObject(configuration) =
        client.get_configuration().await.map_err(|err| {
            tracing::error!("couldn't get node's configuration: {err}");
            APIError::RequestFailed
        })?
    else {
        tracing::error!("received invalid ststus code: {}", StatusCode::FORBIDDEN);
        return Err(APIError::InvalidStatusCode(StatusCode::FORBIDDEN).into());
    };

    Ok(configuration)
}

/// Fetches all known nodes information from `ApiNoContext` instance.
///
/// # Errors
///
/// This function will return an error if the request to the specified client failed or the invalid
/// status code was received
pub async fn fetch_nodes<
    Context: Send + Sync,
    ApiInterface: ApiNoContext<Context> + Send + Sync,
>(
    client: &ApiInterface,
) -> AxumResult<Vec<dto::Node>> {
    let GetNodesResponse::AJSONArrayOfNodesInfoAndVdisksOnThem(nodes) =
        client.get_nodes().await.map_err(|err| {
            tracing::error!("couldn't get nodes list from bob: {err}");
            APIError::RequestFailed
        })?
    else {
        tracing::error!("received invalid ststus code: {}", StatusCode::FORBIDDEN);
        return Err(APIError::InvalidStatusCode(StatusCode::FORBIDDEN).into());
    };

    Ok(nodes)
}

/// Return `VDisk` information by id
///
/// # Errors
///
/// This function will return an error if vdisks information couldn't be fetched or no vdisk with
/// provided id was found
pub async fn get_vdisk_by_id(client: &HttpBobClient, vdisk_id: u64) -> AxumResult<VDisk> {
    let virtual_disks = fetch_vdisks(client.api_main()).await?;
    let virtual_disks = virtual_disks
        .iter()
        .find(|vdisk| vdisk.id as u64 == vdisk_id)
        .ok_or_else(|| StatusCode::NOT_FOUND.into_response())?;
    let clients = virtual_disks
        .replicas
        .iter()
        .flatten()
        .map(|replica| replica.node.clone())
        .collect::<HashSet<_>>()
        .iter()
        .filter_map(|node_name| client.api_secondary(node_name))
        .collect::<Vec<_>>();
    let partition_count = if let Some(handle) = clients.first() {
        handle.get_partitions(vdisk_id as i32).await.map_or_else(
            |_err| 0,
            |parts| {
                if let GetPartitionsResponse::NodeInfoAndJSONArrayWithPartitionsInfo(parts) = parts
                {
                    parts.partitions.unwrap_or_default().len()
                } else {
                    0
                }
            },
        )
    } else {
        0
    };
    let mut disks: FuturesUnordered<_> = clients
        .iter()
        .map(move |&node| {
            let handle = node.clone();
            tokio::spawn(async move { (handle.get_status().await, handle.get_disks().await) })
        })
        .collect();
    let mut replicas: HashMap<_, _> = virtual_disks
        .replicas
        .clone()
        .into_iter()
        .flatten()
        .map(|replica| {
            (
                (replica.disk.clone(), replica.node.clone()),
                Replica {
                    node: replica.node,
                    disk: replica.disk,
                    path: replica.path,
                    status: ReplicaStatus::Offline {
                        problems: vec![ReplicaProblem::NodeUnavailable],
                    },
                },
            )
        })
        .collect();
    while let Some(res) = disks.next().await {
        if let Ok((
            Ok(GetStatusResponse::AJSONWithNodeInfo(status)),
            Ok(GetDisksResponse::AJSONArrayWithDisksAndTheirStates(disks)),
        )) = res
        {
            for disk in disks {
                replicas.insert(
                    (disk.name.clone(), status.name.clone()),
                    Replica {
                        node: status.name.clone(),
                        disk: disk.name,
                        path: disk.path,
                        status: disk
                            .is_active
                            .then_some(ReplicaStatus::Good)
                            .unwrap_or_else(|| ReplicaStatus::Offline {
                                problems: vec![ReplicaProblem::DiskUnavailable],
                            }),
                    },
                );
            }
        } else {
            tracing::warn!("couldn't receive node's space info");
        }
    }

    let replicas: Vec<_> = replicas.into_values().collect();
    let count = replicas
        .iter()
        .filter(|replica| matches!(replica.status, ReplicaStatus::Offline { .. }))
        .count();
    let status = if count == 0 {
        VDiskStatus::Good
    } else if count == replicas.len() {
        VDiskStatus::Offline
    } else {
        VDiskStatus::Bad
    };

    Ok(VDisk {
        id: vdisk_id,
        status,
        partition_count: partition_count as u64,
        replicas,
    })
}
