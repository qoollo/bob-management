/* This file is generated and managed by tsync */

interface MetricsEntryModel {
  value: number;
  timestamp: number;
}

/** Physical disk definition */
interface Disk {
  /** Disk name */
  name: string;
  /** Disk path */
  path: string;
  /** Disk status */
  status: DiskStatus;
  totalSpace: number;
  usedSpace: number;
  iops: number;
}

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
  | DiskStatus__Bad
  | DiskStatus__Offline;

type DiskStatus__Good = {
  status: "Good";
};
type DiskStatus__Bad = {
  status: "Bad";
  problems: Array<DiskProblem>;
};
type DiskStatus__Offline = {
  status: "Offline";
};

/** Defines disk status names */
type DiskStatusName =
  | "Good" | "Bad" | "Offline";

interface NodeInfo {
  name: string;
  hostname: string;
  vdisks: Array<VDisk>;
  status: NodeStatus;
  rps?: RPS;
  alienCount?: number;
  corruptedCount?: number;
  space?: SpaceInfo;
}

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
  | NodeStatus__Bad
  | NodeStatus__Offline;

type NodeStatus__Good = {
  status: "Good";
};
type NodeStatus__Bad = {
  status: "Bad";
  problems: Array<NodeProblem>;
};
type NodeStatus__Offline = {
  status: "Offline";
};

/** Defines node status names */
type NodeStatusName =
  | "Good" | "Bad" | "Offline";

/** [`VDisk`]'s replicas */
interface Replica {
  node: string;
  disk: string;
  path: string;
  status: ReplicaStatus;
}

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
  | ReplicaStatus__Good
  | ReplicaStatus__Offline;

type ReplicaStatus__Good = {
  status: "Good";
};
type ReplicaStatus__Offline = {
  status: "Offline";
  problems: Array<ReplicaProblem>;
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

/** Virtual disk Component */
interface VDisk {
  id: number;
  status: VDiskStatus;
  partition_count: number;
  replicas: Array<Replica>;
}

/**
 * Virtual disk status.
 * 
 * Variants - Virtual Disk status
 * status == 'bad' when at least one of its replicas has problems
 */
type VDiskStatus =
  | "Good" | "Bad" | "Offline";

interface DetailedNode {
  name: string;
  hostname: string;
  vdisks: Array<VDisk>;
  status: NodeStatus;
  metrics: DetailedNodeMetrics;
  disks: Array<Disk>;
}

interface DetailedNodeMetrics {
  rps: RPS;
  alienCount: number;
  corruptedCount: number;
  space: SpaceInfo;
  cpuLoad: number;
  totalRam: number;
  usedRam: number;
  descrAmount: number;
}

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

/** BOB's Node interface */
interface DTONode {
  name: string;
  address: string;
  vdisks?: Array<DTOVDisk>;
}

/** BOB's Node Configuration interface */
interface DTONodeConfiguration {
  blob_file_name_prefix?: string;
  root_dir_name?: string;
}

/** BOB's VDisk interface */
interface DTOVDisk {
  id: number;
  replicas?: Array<DTOReplica>;
}

/** BOB's Replica interface */
interface DTOReplica {
  node: string;
  disk: string;
  path: string;
}
