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
