/// RPS list per operation for RPS chart.
type RPSList = TypedMap<Operation, number[]>;

interface DashboardState {
    diskSpace: SpaceInfo;
    nodeCount: NodeCount;
    disksCount: DiskCount;
    rps: RPS;
    timeList: string[];
    rpsTotalList: number[];
    rpsBreakdownList: RPSList;
    dataLoaded: boolean;
}

interface NodeTableCols {
    id: number;
    nodename: string;
    hostname: string;
    status: NodeStatusName;
    space?: SpaceInfo;
    rps?: RPS;
    aliens?: number;
    corruptedBlobs?: number;
}

interface ReplicaCount {
    goodReplicas: number;
    totalReplicas: number;
}

interface VDiskTableCols {
    id: number;
    vdiskid: number;
    replicas: Replica[];
    availability: ReplicaCount;
    status: VDiskStatus;
}
