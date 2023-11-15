#![allow(unused_qualifications)]

use super::prelude::*;

pub const DEFAULT_MAX_CPU: u64 = 90;
pub const DEFAULT_MIN_FREE_SPACE: f64 = 0.1;

/// Connection Data
pub use crate::models::shared::{BobConnectionData, Credentials};

/// Defines kind of problem on disk
#[derive(ToSchema, Debug, Clone, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub enum DiskProblem {
    #[serde(rename = "freeSpaceRunningOut")]
    FreeSpaceRunningOut,
}

/// Defines disk status
///
/// Variant - Disk Status
/// Content - List of problems on disk. 'null' if status != 'bad'
#[derive(ToSchema, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(tag = "status", content = "problems")]
pub enum DiskStatus {
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "bad")]
    Bad(Vec<DiskProblem>),
    #[serde(rename = "offline")]
    Offline,
}

/// Defines disk status names
#[derive(ToSchema, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, EnumIter)]
#[serde(rename_all = "camelCase")]
pub enum DiskStatusName {
    Good,
    Bad,
    Offline,
}

/// Defines kind of problem on Node
#[derive(ToSchema, Debug, Clone, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub enum NodeProblem {
    #[serde(rename = "aliensExists")]
    AliensExists,
    #[serde(rename = "corruptedExists")]
    CorruptedExists,
    #[serde(rename = "freeSpaceRunningOut")]
    FreeSpaceRunningOut,
    #[serde(rename = "virtualMemLargerThanRAM")]
    VirtualMemLargerThanRAM,
    #[serde(rename = "highCPULoad")]
    HighCPULoad,
}

impl NodeProblem {
    #[must_use]
    pub fn default_from_metrics(node_metrics: &TypedMetrics) -> Vec<Self> {
        Self::from_metrics(node_metrics, DEFAULT_MAX_CPU, DEFAULT_MIN_FREE_SPACE)
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn from_metrics(
        node_metrics: &TypedMetrics,
        max_cpu: u64,
        min_free_space: f64,
    ) -> Vec<Self> {
        let mut res = vec![];
        if node_metrics[RawMetricEntry::BackendAlienCount].value != 0 {
            res.push(Self::AliensExists);
        }
        if node_metrics[RawMetricEntry::BackendCorruptedBlobCount].value != 0 {
            res.push(Self::CorruptedExists);
        }
        if node_metrics[RawMetricEntry::HardwareBobCpuLoad].value >= max_cpu {
            res.push(Self::HighCPULoad);
        }
        if (1.
            - (node_metrics[RawMetricEntry::HardwareTotalSpace].value
                - node_metrics[RawMetricEntry::HardwareFreeSpace].value) as f64
                / node_metrics[RawMetricEntry::HardwareTotalSpace].value as f64)
            < min_free_space
        {
            res.push(Self::FreeSpaceRunningOut);
        }
        if node_metrics[RawMetricEntry::HardwareBobVirtualRam]
            > node_metrics[RawMetricEntry::HardwareTotalRam]
        {
            res.push(Self::VirtualMemLargerThanRAM);
        }

        res
    }
}

/// Defines status of node
///
/// Variants - Node status
///
/// Content - List of problems on node. 'null' if status != 'bad'
#[derive(ToSchema, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(tag = "status", content = "problems")]
pub enum NodeStatus {
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "bad")]
    Bad(Vec<NodeProblem>),
    #[serde(rename = "offline")]
    Offline,
}

impl NodeStatus {
    #[must_use]
    pub fn from_problems(problems: Vec<NodeProblem>) -> Self {
        if problems.is_empty() {
            Self::Good
        } else {
            Self::Bad(problems)
        }
    }
}

/// Defines node status names
#[derive(ToSchema, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, EnumIter)]
#[serde(rename_all = "camelCase")]
pub enum NodeStatusName {
    Good,
    Bad,
    Offline,
}

/// Reasons why Replica is offline
#[derive(ToSchema, Debug, Clone, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ReplicaProblem {
    #[serde(rename = "nodeUnavailable")]
    NodeUnavailable,
    #[serde(rename = "diskUnavailable")]
    DiskUnavailable,
}

/// Replica status. It's either good or offline with the reasons why it is offline
///
/// Variants - Replica status
///
/// Content - List of problems on replica. 'null' if status != 'offline'
#[derive(ToSchema, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", content = "problems")]
pub enum ReplicaStatus {
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "offline")]
    Offline(Vec<ReplicaProblem>),
}

/// Disk space information in bytes
#[derive(ToSchema, Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SpaceInfo {
    /// Total disk space amount
    pub total_disk: u64,

    /// The amount of free disk space
    pub free_disk: u64,

    /// Used disk space amount
    pub used_disk: u64,

    /// Disk space occupied only by BOB. occupied_disk should be lesser than used_disk
    pub occupied_disk: u64,
}

/// Virtual disk status.
///
/// Variants - Virtual Disk status
/// status == 'bad' when at least one of its replicas has problems
#[derive(ToSchema, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum VDiskStatus {
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "bad")]
    Bad,
    #[serde(rename = "offline")]
    Offline,
}

#[derive(
    ToSchema, Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord, EnumIter,
)]
#[serde(rename_all = "camelCase")]
pub enum Operation {
    Put,
    Get,
    Exist,
    Delete,
}

#[derive(
    ToSchema, Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord, EnumIter,
)]
pub enum RawMetricEntry {
    #[serde(rename = "cluster_grinder.get_count_rate")]
    ClusterGrinderGetCountRate,
    #[serde(rename = "cluster_grinder.put_count_rate")]
    ClusterGrinderPutCountRate,
    #[serde(rename = "cluster_grinder.exist_count_rate")]
    ClusterGrinderExistCountRate,
    #[serde(rename = "cluster_grinder.delete_count_rate")]
    ClusterGrinderDeleteCountRate,
    #[serde(rename = "pearl.exist_count_rate")]
    PearlExistCountRate,
    #[serde(rename = "pearl.get_count_rate")]
    PearlGetCountRate,
    #[serde(rename = "pearl.put_count_rate")]
    PearlPutCountRate,
    #[serde(rename = "pearl.delete_count_rate")]
    PearlDeleteCountRate,
    #[serde(rename = "backend.alien_count")]
    BackendAlienCount,
    #[serde(rename = "backend.corrupted_blob_count")]
    BackendCorruptedBlobCount,
    #[serde(rename = "hardware.bob_virtual_ram")]
    HardwareBobVirtualRam,
    #[serde(rename = "hardware.total_ram")]
    HardwareTotalRam,
    #[serde(rename = "hardware.used_ram")]
    HardwareUsedRam,
    #[serde(rename = "hardware.bob_cpu_load")]
    HardwareBobCpuLoad,
    #[serde(rename = "hardware.free_space")]
    HardwareFreeSpace,
    #[serde(rename = "hardware.total_space")]
    HardwareTotalSpace,
    #[serde(rename = "hardware.descr_amount")]
    HardwareDescrAmount,
}

#[allow(dead_code, clippy::expect_used)]
fn get_map_schema<Id: IntoEnumIterator + Serialize, V: PartialSchema>() -> Object {
    let mut res = ObjectBuilder::new();
    for key in Id::iter() {
        let key = serde_json::to_string(&key).expect("infallible");
        let key = key.trim_matches('"');
        res = res.required(key).property(key, V::schema());
    }
    res.build()
}

#[derive(ToSchema, Debug, Serialize, Clone)]
#[aliases(RPS = TypedMap<Operation, u64>, TypedMetrics = TypedMap<RawMetricEntry, dto::MetricsEntryModel>, NodeCount = TypedMap<NodeStatusName, u64>, DiskCount = TypedMap<DiskStatusName, u64>)]
pub struct TypedMap<Id: IntoEnumIterator + Eq + Hash, Value: PartialSchema> {
    // FIXME: Bugged
    // See -> https://github.com/juhaku/utoipa/issues/644
    // #[schema(schema_with = get_map_schema::<Id, Value>)]
    #[serde(flatten)]
    map: HashMap<Id, Value>,
}

// pub type TypedMetrics = TypedMap<RawMetricEntry, MetricsEntryModel>;

impl<Id: IntoEnumIterator + Eq + Hash, V: PartialSchema> std::ops::Index<Id> for TypedMap<Id, V> {
    type Output = V;

    fn index(&self, index: Id) -> &Self::Output {
        self.map.index(&index)
    }
}

#[allow(clippy::expect_used)]
impl<Id: IntoEnumIterator + Eq + Hash, V: PartialSchema> std::ops::IndexMut<Id>
    for TypedMap<Id, V>
{
    fn index_mut(&mut self, index: Id) -> &mut Self::Output {
        self.map.get_mut(&index).expect("infallible")
    }
}

impl<Id: IntoEnumIterator + Hash + Eq, V: Default + PartialSchema> Default for TypedMap<Id, V> {
    fn default() -> Self {
        let mut map = HashMap::new();
        for key in Id::iter() {
            map.insert(key, V::default());
        }

        Self { map }
    }
}

impl<Id: IntoEnumIterator + Hash + Eq, V: Default + PartialSchema> TypedMap<Id, V> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

pub trait Util<Id: IntoEnumIterator> {
    fn key_iter() -> Id::Iterator;
}

impl<Id: IntoEnumIterator + Hash + Eq, V: Default + PartialSchema> Util<Id> for TypedMap<Id, V> {
    fn key_iter() -> Id::Iterator {
        Id::iter()
    }
}

#[allow(clippy::expect_used)]
impl From<dto::MetricsSnapshotModel> for TypedMetrics {
    fn from(value: dto::MetricsSnapshotModel) -> Self {
        let mut map = HashMap::new();
        let mut value = value.metrics;
        for key in RawMetricEntry::iter() {
            let value = value
                .remove(&serde_json::to_string(&key).expect("infallible"))
                .unwrap_or_default();
            map.insert(key, value);
        }

        Self { map }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DiskCount, DiskStatusName, NodeCount, NodeStatusName, Operation, RawMetricEntry,
        TypedMetrics, RPS,
    };
    use crate::connector::dto::MetricsEntryModel;
    use strum::IntoEnumIterator;

    #[test]
    fn raw_metrics_entry_iter() {
        for key in RawMetricEntry::iter() {
            assert!(serde_json::to_string(&key).is_ok());
        }
    }

    #[test]
    fn disk_status_iter() {
        for key in DiskStatusName::iter() {
            assert!(serde_json::to_string(&key).is_ok());
        }
    }

    #[test]
    fn node_status_iter() {
        for key in NodeStatusName::iter() {
            assert!(serde_json::to_string(&key).is_ok());
        }
    }

    #[test]
    fn metrics_index() {
        let metrics = TypedMetrics::default();
        for key in RawMetricEntry::iter() {
            assert_eq!(metrics[key], MetricsEntryModel::default());
        }
    }

    #[test]
    fn node_count_index() {
        let node_count = NodeCount::default();
        for key in NodeStatusName::iter() {
            assert_eq!(node_count[key], 0);
        }
    }

    #[test]
    fn disk_count_index() {
        let disk_count = DiskCount::default();
        for key in DiskStatusName::iter() {
            assert_eq!(disk_count[key], 0);
        }
    }

    #[test]
    fn rps_index() {
        let rps = RPS::default();
        for key in Operation::iter() {
            assert_eq!(rps[key], 0);
        }
    }
}
