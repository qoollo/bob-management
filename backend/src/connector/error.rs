use super::prelude::*;

impl From<GetAlienResponse> for StatusCode {
    fn from(value: GetAlienResponse) -> Self {
        match value {
            GetAlienResponse::AlienNodeName(_) => StatusCode::OK,
        }
    }
}

impl From<GetAlienDirResponse> for StatusCode {
    fn from(value: GetAlienDirResponse) -> Self {
        match value {
            GetAlienDirResponse::Directory(_) => StatusCode::OK,
            GetAlienDirResponse::PermissionDenied(_) => StatusCode::FORBIDDEN,
            GetAlienDirResponse::NotAcceptableBackend(_) => StatusCode::NOT_ACCEPTABLE,
        }
    }
}

impl From<GetDisksResponse> for StatusCode {
    fn from(value: GetDisksResponse) -> Self {
        match value {
            GetDisksResponse::AJSONArrayWithDisksAndTheirStates(_) => StatusCode::OK,
            GetDisksResponse::PermissionDenied(_) => StatusCode::FORBIDDEN,
        }
    }
}

impl From<GetMetricsResponse> for StatusCode {
    fn from(value: GetMetricsResponse) -> Self {
        match value {
            GetMetricsResponse::Metrics(_) => StatusCode::OK,
        }
    }
}

impl From<GetNodesResponse> for StatusCode {
    fn from(value: GetNodesResponse) -> Self {
        match value {
            GetNodesResponse::AJSONArrayOfNodesInfoAndVdisksOnThem(_) => StatusCode::OK,
            GetNodesResponse::PermissionDenied => StatusCode::FORBIDDEN,
        }
    }
}

impl From<GetPartitionResponse> for StatusCode {
    fn from(value: GetPartitionResponse) -> Self {
        match value {
            GetPartitionResponse::AJSONWithPartitionInfo(_) => StatusCode::OK,
            GetPartitionResponse::PermissionDenied(_) => StatusCode::FORBIDDEN,
            GetPartitionResponse::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl From<GetRecordsResponse> for StatusCode {
    fn from(value: GetRecordsResponse) -> Self {
        match value {
            GetRecordsResponse::RecordsCount(_) => StatusCode::OK,
            GetRecordsResponse::PermissionDenied(_) => StatusCode::FORBIDDEN,
            GetRecordsResponse::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl From<GetReplicasLocalDirsResponse> for StatusCode {
    fn from(value: GetReplicasLocalDirsResponse) -> Self {
        match value {
            GetReplicasLocalDirsResponse::AJSONArrayWithDirs(_) => StatusCode::OK,
            GetReplicasLocalDirsResponse::PermissionDenied(_) => StatusCode::FORBIDDEN,
            GetReplicasLocalDirsResponse::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl From<GetStatusResponse> for StatusCode {
    fn from(value: GetStatusResponse) -> Self {
        match value {
            GetStatusResponse::AJSONWithNodeInfo(_) => StatusCode::OK,
        }
    }
}

impl From<GetVDiskResponse> for StatusCode {
    fn from(value: GetVDiskResponse) -> Self {
        match value {
            GetVDiskResponse::AJSONWithVdiskInfo(_) => StatusCode::OK,
            GetVDiskResponse::PermissionDenied(_) => StatusCode::FORBIDDEN,
            GetVDiskResponse::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

impl From<GetVDisksResponse> for StatusCode {
    fn from(value: GetVDisksResponse) -> Self {
        match value {
            GetVDisksResponse::AJSONArrayOfVdisksInfo(_) => StatusCode::OK,
            GetVDisksResponse::PermissionDenied => StatusCode::FORBIDDEN,
        }
    }
}

impl From<GetVersionResponse> for StatusCode {
    fn from(value: GetVersionResponse) -> Self {
        match value {
            GetVersionResponse::VersionInfo(_) => StatusCode::OK,
        }
    }
}

impl From<GetConfigurationResponse> for StatusCode {
    fn from(value: GetConfigurationResponse) -> Self {
        match value {
            GetConfigurationResponse::ConfigurationObject(_) => StatusCode::OK,
            GetConfigurationResponse::PermissionDenied => StatusCode::FORBIDDEN,
        }
    }
}

pub trait AsApiError {
    fn as_invalid_status(self) -> APIError;
}

impl<T: Into<StatusCode>> AsApiError for T {
    fn as_invalid_status(self) -> APIError {
        APIError::InvalidStatusCode(self.into())
    }
}
