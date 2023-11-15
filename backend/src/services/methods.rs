use super::prelude::*;

pub async fn request_metrics<
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

pub async fn request_vdisks<
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

pub async fn request_space<
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

pub async fn request_status<
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

pub async fn request_disks<
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

pub async fn request_configuration<
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

pub async fn request_nodes<
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
