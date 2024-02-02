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
