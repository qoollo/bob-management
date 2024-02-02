import { Context } from '@appTypes/context.ts';
import defaultTheme from '@layouts/DefaultTheme.ts';
import { Box, Grid, ThemeProvider } from '@mui/material';
import { useStore } from '@nanostores/react';
import axios from 'axios';
import React, { useEffect, useMemo, useState } from 'react';

import FetchingBackdrop from '../backdrop/backdrop.tsx';
import ClusterRpsChart from '../clusterRpsChart/clusterRpsChart.tsx';
import CrudChart from '../crudChart/crudChart.tsx';
import DiskPie from '../diskPie/diskPie.tsx';
import DiskBreakdown from '../diskState/diskState.tsx';
import TotalNodes from '../totalNodes/totalNodes.tsx';

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

const initialDashboard: DashboardState = {
    diskSpace: {} as SpaceInfo,
    nodeCount: {} as NodeCount,
    disksCount: {} as DiskCount,
    rps: {} as RPS,
    timeList: [] as string[],
    rpsTotalList: [] as number[],
    rpsBreakdownList: {
        map: {
            get: [],
            put: [],
            exist: [],
            delete: [],
        },
    } as RPSList,
    dataLoaded: false,
};

const Dashboard = () => {
    const [isPageLoaded, setIsPageLoaded] = useState(false);
    const context = useStore(Context);

    const [dashboard, setDashboard] = useState(initialDashboard);

    window.onload = () => {
        const loadedDashboard = window.sessionStorage.getItem('dashboard');
        if (loadedDashboard) {
            const parsedDasboard: DashboardState = JSON.parse(loadedDashboard);
            parsedDasboard.dataLoaded = true;
            setDashboard(parsedDasboard);
        }
    };
    window.onbeforeunload = () => {
        window.sessionStorage.setItem('dashboard', JSON.stringify(dashboard));
    };

    // let time = 0;

    const fetchData = useMemo(
        () => async () => {
            try {
                const [space, disksCount, nodesCount, rps] = await Promise.all([
                    axios.get<SpaceInfo>('/api/v1/nodes/space'),
                    axios.get<DiskCount>('/api/v1/disks/count'),
                    axios.get<NodeCount>('/api/v1/nodes/count'),
                    axios.get<RPS>('/api/v1/nodes/rps'),
                ]);

                /// Don't think we need to preserve time state?..

                // time += Context.get().refreshTime * 60;

                dashboard.timeList.push(new Date().toLocaleTimeString());
                dashboard.rpsTotalList.push(
                    rps.data.map.put + rps.data.map.get + rps.data.map.exist + rps.data.map.delete,
                );
                dashboard.rpsBreakdownList.map.get.push(rps.data.map.get);
                dashboard.rpsBreakdownList.map.put.push(rps.data.map.put);
                dashboard.rpsBreakdownList.map.exist.push(rps.data.map.exist);
                dashboard.rpsBreakdownList.map.delete.push(rps.data.map.delete);

                setDashboard({
                    ...dashboard,
                    diskSpace: space.data,
                    nodeCount: nodesCount.data,
                    disksCount: disksCount.data,
                    rps: rps.data,
                    dataLoaded: true,
                });
            } catch (err) {
                setDashboard({
                    ...dashboard,
                    dataLoaded: false,
                });
                // location.assign('/login');
            }
        },
        [dashboard],
    );

    useEffect(() => {
        if (!isPageLoaded) {
            fetchData();
        }

        setIsPageLoaded(true);
    }, [fetchData, isPageLoaded]);

    useEffect(() => {
        const interval = setInterval(() => {
            if (context.enabled) {
                fetchData();
            }
        }, context.refreshTime * 1000);

        return () => clearInterval(interval);
    }, [context.enabled, context.refreshTime, fetchData]);

    if (!isPageLoaded) {
        return null;
    }

    if (!dashboard.dataLoaded) {
        return <FetchingBackdrop />;
    }

    return (
        <ThemeProvider theme={defaultTheme}>
            <Box
                sx={{
                    margin: '24px',
                }}
            >
                <Grid container spacing={2}>
                    <Grid item xs={4}>
                        <Box
                            sx={{
                                display: 'flex',
                                flexDirection: 'column',
                                gap: '16px',
                            }}
                        >
                            <TotalNodes nodeCount={dashboard.nodeCount} />
                            <DiskBreakdown diskCount={dashboard.disksCount} />
                        </Box>
                    </Grid>

                    <Grid item xs={8}>
                        <Box
                            sx={{
                                backgroundColor: '#212328',
                                padding: '14px 24px 16px 24px',
                                borderRadius: '8px',
                                height: '100%',
                            }}
                        >
                            <ClusterRpsChart timex={dashboard.timeList} rpsy={dashboard.rpsTotalList} />
                        </Box>
                    </Grid>

                    <Grid item xs={4}>
                        <Box
                            sx={{
                                backgroundColor: '#212328',
                                padding: '14px 24px 16px 24px',
                                borderRadius: '8px',
                                height: '400px',
                                width: '100%',
                            }}
                        >
                            <DiskPie spaceInfo={dashboard.diskSpace} />
                        </Box>
                    </Grid>

                    <Grid item xs={8}>
                        <Box
                            sx={{
                                backgroundColor: '#212328',
                                padding: '14px 24px 16px 24px',
                                borderRadius: '8px',
                                height: '400px',
                            }}
                        >
                            <CrudChart
                                time={dashboard.timeList}
                                get={dashboard.rpsBreakdownList.map.get}
                                put={dashboard.rpsBreakdownList.map.put}
                                exist={dashboard.rpsBreakdownList.map.exist}
                                del={dashboard.rpsBreakdownList.map.delete}
                            />
                        </Box>
                    </Grid>
                </Grid>
            </Box>
        </ThemeProvider>
    );
};

export default Dashboard;
