/* This file is generated and managed by tsync */

/** Defines kind of problem on disk */
type DiskProblem =
  | "FreeSpaceRunningOut";

/**
 * Defines disk status
 * 
 * Variant - Disk Status
 * Content - List of problems on disk. 'null' if status != 'bad'
 */
type DiskStatus =
  | DiskStatus__Good
  | DiskStatus__Offline;

type DiskStatus__Good = {
  status: "Good";
};
type DiskStatus__Offline = {
  status: "Offline";
};

/** Defines disk status names */
type DiskStatusName =
  | "good" | "bad" | "offline";

/** Defines kind of problem on Node */
type NodeProblem =
  | "AliensExists" | "CorruptedExists" | "FreeSpaceRunningOut" | "VirtualMemLargerThanRAM" | "HighCPULoad";

/**
 * Defines status of node
 * 
 * Variants - Node status
 * 
 * Content - List of problems on node. 'null' if status != 'bad'
 */
type NodeStatus =
  | NodeStatus__Good
  | NodeStatus__Offline;

type NodeStatus__Good = {
  status: "Good";
};
type NodeStatus__Offline = {
  status: "Offline";
};

/** Defines node status names */
type NodeStatusName =
  | "good" | "bad" | "offline";

/** Reasons why Replica is offline */
type ReplicaProblem =
  | "NodeUnavailable" | "DiskUnavailable";

/**
 * Replica status. It's either good or offline with the reasons why it is offline
 * 
 * Variants - Replica status
 * 
 * Content - List of problems on replica. 'null' if status != 'offline'
 */
type ReplicaStatus =
  | ReplicaStatus__Good;

type ReplicaStatus__Good = {
  status: "Good";
};

/** Disk space information in bytes */
interface SpaceInfo {
  /** Total disk space amount */
  total_disk: number;
  /** The amount of free disk space */
  free_disk: number;
  /** Used disk space amount */
  used_disk: number;
  /** Disk space occupied only by BOB. occupied_disk should be lesser than used_disk */
  occupied_disk: number;
}

/**
 * Virtual disk status.
 * 
 * Variants - Virtual Disk status
 * status == 'bad' when at least one of its replicas has problems
 */
type VDiskStatus =
  | "Good" | "Bad" | "Offline";

/** Types of operations on BOB cluster */
type Operation =
  | "put" | "get" | "exist" | "delete";

type RawMetricEntry =
  | "ClusterGrinderGetCountRate" | "ClusterGrinderPutCountRate" | "ClusterGrinderExistCountRate" | "ClusterGrinderDeleteCountRate" | "PearlExistCountRate" | "PearlGetCountRate" | "PearlPutCountRate" | "PearlDeleteCountRate" | "BackendAlienCount" | "BackendCorruptedBlobCount" | "HardwareBobVirtualRam" | "HardwareTotalRam" | "HardwareUsedRam" | "HardwareBobCpuLoad" | "HardwareFreeSpace" | "HardwareTotalSpace" | "HardwareDescrAmount";

type RPS = TypedMap<Operation, number>

type TypedMetrics = TypedMap<RawMetricEntry, MetricsEntryModel>

type NodeCount = TypedMap<NodeStatusName, number>

type DiskCount = TypedMap<DiskStatusName, number>

interface TypedMap<Id, Value> {
  map: Record<Id, Value>;
}

/** Data needed to connect to a BOB cluster */
interface BobConnectionData {
  /** Address to connect to */
  hostname: Hostname;
  /** [Optional] Credentials used for BOB authentication */
  credentials?: Credentials;
}

/** Optional auth credentials for a BOB cluster */
interface Credentials {
  /** Login used during auth */
  login: string;
  /** Password used during auth */
  password: string;
}

type Hostname = string
