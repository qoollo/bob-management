#![allow(unused_qualifications)]

use super::prelude::*;

pub const DEFAULT_MAX_CPU: u64 = 90;
pub const DEFAULT_MIN_FREE_SPACE_PERCENTAGE: f64 = 0.1;

/// Connection Data
pub use crate::models::shared::{BobConnectionData, Credentials};

/// Defines kind of problem on disk
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
pub enum DiskProblem {
    #[serde(rename = "freeSpaceRunningOut")]
    FreeSpaceRunningOut,
}

/// Defines disk status
///
/// Variant - Disk Status
/// Content - List of problems on disk. 'null' if status != 'bad'
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(tag = "status", content = "problems")]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
pub enum DiskStatus {
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "bad")]
    Bad(Vec<DiskProblem>),
    #[serde(rename = "offline")]
    Offline,
}

impl DiskStatus {
    #[must_use]
    pub fn from_space_info(space: &dto::SpaceInfo, disk_name: &str) -> Self {
        if let Some(&occupied_space) = space.occupied_disk_space_by_disk.get(disk_name) {
            #[allow(clippy::cast_precision_loss)]
            if ((space.total_disk_space_bytes - occupied_space) as f64
                / space.total_disk_space_bytes as f64)
                < DEFAULT_MIN_FREE_SPACE_PERCENTAGE
            {
                Self::Bad(vec![DiskProblem::FreeSpaceRunningOut])
            } else {
                Self::Good
            }
        } else {
            Self::Offline
        }
    }
}

/// Defines disk status names
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, EnumIter)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
pub enum DiskStatusName {
    Good,
    Bad,
    Offline,
}

/// Defines kind of problem on Node
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
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
        Self::from_metrics(
            node_metrics,
            DEFAULT_MAX_CPU,
            DEFAULT_MIN_FREE_SPACE_PERCENTAGE,
        )
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn from_metrics(
        node_metrics: &TypedMetrics,
        max_cpu: u64,
        min_free_space_perc: f64,
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
            < min_free_space_perc
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
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(tag = "status", content = "problems")]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
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

impl TypedMetrics {
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn is_bad_node(&self) -> bool {
        self[RawMetricEntry::BackendAlienCount].value != 0
            || self[RawMetricEntry::BackendCorruptedBlobCount].value != 0
            || self[RawMetricEntry::HardwareBobCpuLoad].value >= DEFAULT_MAX_CPU
            || (1.
                - (self[RawMetricEntry::HardwareTotalSpace].value
                    - self[RawMetricEntry::HardwareFreeSpace].value) as f64
                    / self[RawMetricEntry::HardwareTotalSpace].value as f64)
                < DEFAULT_MIN_FREE_SPACE_PERCENTAGE
            || self[RawMetricEntry::HardwareBobVirtualRam] > self[RawMetricEntry::HardwareTotalRam]
    }
}

/// Defines node status names
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, EnumIter)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
pub enum NodeStatusName {
    Good,
    Bad,
    Offline,
}

/// Reasons why Replica is offline
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
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
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", content = "problems")]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
pub enum ReplicaStatus {
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "offline")]
    Offline(Vec<ReplicaProblem>),
}

/// Disk space information in bytes
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
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
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status")]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
#[cfg_attr(all(feature = "swagger", debug_assertions),
    schema(example = json!({"status": "good"})))]
pub enum VDiskStatus {
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "bad")]
    Bad,
    #[serde(rename = "offline")]
    Offline,
}

/// Types of operations on BOB cluster
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord, EnumIter)]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub enum Operation {
    Put,
    Get,
    Exist,
    Delete,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord, EnumIter)]
#[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
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
#[cfg(all(feature = "swagger", debug_assertions))]
fn get_map_schema<Id: IntoEnumIterator + Serialize, V: PartialSchema + Default + Serialize>(
) -> Object {
    let mut res = ObjectBuilder::new();
    let mut example = serde_json::Map::new();
    for key in Id::iter() {
        let key = serde_json::to_string(&key).expect("infallible");
        let key = key.trim_matches('"');
        res = res.required(key).property(key, V::schema());
        example.insert(
            key.to_string(),
            serde_json::to_value(V::default()).expect("infallible"),
        );
    }
    res.example(serde_json::to_value(example).ok()).build()
}

// #[cfg(not(all(feature = "swagger", debug_assertions)))]
pub type RPS = TypedMap<Operation, u64>;
// #[cfg(not(all(feature = "swagger", debug_assertions)))]
pub type TypedMetrics = TypedMap<RawMetricEntry, dto::MetricsEntryModel>;
// #[cfg(not(all(feature = "swagger", debug_assertions)))]
pub type NodeCount = TypedMap<NodeStatusName, u64>;
// #[cfg(not(all(feature = "swagger", debug_assertions)))]
pub type DiskCount = TypedMap<DiskStatusName, u64>;

#[derive(Debug, Serialize, Clone)]
// #[cfg_attr(all(feature = "swagger", debug_assertions), derive(ToSchema))]
// #[cfg_attr(all(feature = "swagger", debug_assertions),
//     aliases(
//         RPS = TypedMap<Operation, u64>,
//         TypedMetrics = TypedMap<RawMetricEntry, dto::MetricsEntryModel>,
//         NodeCount = TypedMap<NodeStatusName, u64>,
//         DiskCount = TypedMap<DiskStatusName, u64>
//     )
// )]
// #[cfg_attr(all(feature = "swagger", debug_assertions),
//     schema(example = json!({"put": 7, "get": 8, "delete": 2, "exist": 3})))]
pub struct TypedMap<Id: IntoEnumIterator + Eq + Hash, Value: PartialSchema> {
    // FIXME: Bugged; Remove manual impl's of `ToSchema` and uncomment when fixed
    // See -> https://github.com/juhaku/utoipa/issues/644
    // #[schema(schema_with = get_map_schema::<Id, Value>)]
    #[serde(flatten)]
    map: HashMap<Id, Value>,
}

// FIXME: Remove this when utoipa's bug fixed
impl<
        'a,
        Id: IntoEnumIterator + Eq + Hash + Serialize,
        Value: PartialSchema + Default + Serialize,
    > utoipa::ToSchema<'a> for TypedMap<Id, Value>
{
    fn schema() -> (
        &'a str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            std::any::type_name::<Self>(),
            get_map_schema::<Id, Value>().into(),
        )
    }

    fn aliases() -> Vec<(&'a str, utoipa::openapi::schema::Schema)> {
        vec![
            ("RPS", {
                let mut schema = get_map_schema::<Operation, u64>();
                let _ = schema
                    .description
                    .insert("Requests per second by operation".to_string());
                schema.into()
            }),
            ("TypedMetrics", {
                let mut schema = get_map_schema::<RawMetricEntry, dto::MetricsEntryModel>();
                let _ = schema
                    .description
                    .insert("Raw metrics information".to_string());
                schema.into()
            }),
            ("NodeCount", {
                let mut schema = get_map_schema::<NodeStatusName, u64>();
                let _ = schema
                    .description
                    .insert("Node count by their status".to_string());
                schema.into()
            }),
            ("DiskCount", {
                let mut schema = get_map_schema::<DiskStatusName, u64>();
                let _ = schema
                    .description
                    .insert("Disk count by their status".to_string());
                schema.into()
            }),
        ]
    }
}

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
