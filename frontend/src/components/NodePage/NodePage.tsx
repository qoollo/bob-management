import { Context } from '@appTypes/context.ts';
import defaultTheme from '@layouts/DefaultTheme.ts';
import { Box, ThemeProvider } from '@mui/system';
import { useStore } from '@nanostores/react';
import axios from 'axios';
import React, { useEffect, useMemo, useState } from 'react';

import CurrentRps from '../currentRps/currentRps.tsx';
import DiskTable from '../diskTable/diskTable.tsx';
import NodeTable from '../nodeTable/nodeTable.tsx';
import NodeVDisks from '../nodeVDisks/nodeVDisks.tsx';
import NodeSettings from '../popovers/nodeSettings.tsx';
import RawMetrics from '../popovers/rawMetrics.tsx';
import SystemChar from '../systemChar/systemChar.tsx';

axios.defaults.withCredentials = true;

const stubEntry: MetricsEntryModel = {
    value: -1,
    timestamp: -1,
};

const transform = (node: DetailedNode): NodeInfo => {
    return {
        name: node.name,
        hostname: node.hostname,
        vdisks: node.vdisks,
        status: node.status,
        rps: node.metrics.rps,
        alienCount: node.metrics.alienCount,
        corruptedCount: node.metrics.corruptedCount,
        space: node.metrics.space,
    };
};

const stubMetrics: TypedMetrics = {
    map: {
        ClusterGrinderGetCountRate: stubEntry,
        ClusterGrinderPutCountRate: stubEntry,
        ClusterGrinderExistCountRate: stubEntry,
        ClusterGrinderDeleteCountRate: stubEntry,
        PearlGetCountRate: stubEntry,
        PearlPutCountRate: stubEntry,
        PearlExistCountRate: stubEntry,
        PearlDeleteCountRate: stubEntry,
        BackendAlienCount: stubEntry,
        BackendCorruptedBlobCount: stubEntry,
        HardwareUsedRam: stubEntry,
        HardwareTotalRam: stubEntry,
        HardwareFreeSpace: stubEntry,
        HardwareBobCpuLoad: stubEntry,
        HardwareTotalSpace: stubEntry,
        HardwareDescrAmount: stubEntry,
        HardwareBobVirtualRam: stubEntry,
    },
};

const stubNodeState: DetailedNode = {
    name: 'Loading...',
    hostname: 'Loading...',
    status: {
        status: 'Offline',
    },
    vdisks: [],
    disks: [],
    metrics: {
        rps: {
            map: {
                put: 0,
                get: 0,
                exist: 0,
                delete: 0,
            },
        },
        alienCount: 0,
        corruptedCount: 0,
        cpuLoad: 0,
        space: {
            occupied_disk: 0,
            total_disk: 0,
            used_disk: 0,
            free_disk: 0,
        },
        usedRam: 0,
        totalRam: 0,
        descrAmount: 0,
    },
};

const NodePage = () => {
    const urlSearchParams = new URLSearchParams(window.location.search);
    const nodename = Object.fromEntries(urlSearchParams.entries()).node;

    if (!nodename) {
        window.location.replace('/dashboard');
    }

    const context = useStore(Context);
    const [config, setConfig] = useState<DTONodeConfiguration>({});
    const [rawMetrics, setRawMetrics] = useState<TypedMetrics>(stubMetrics);
    const [nodeDetails, setNodeDetails] = useState<DetailedNode>({
        ...stubNodeState,
        name: nodename,
    });

    // config usually doesn't change
    const fetchConfig = useMemo(
        () => async () => {
            try {
                const [res] = await Promise.all([
                    axios.get<DTONodeConfiguration>(`/api/v1/nodes/${nodename}/configuration`),
                ]);
                setConfig(res.data);
            } catch (err) {
                console.log(err);
            }
        },
        [nodename],
    );

    useEffect(() => {
        fetchConfig();
    }, [fetchConfig]);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const [req, reqMetrics] = await Promise.all([
                    axios.get<DetailedNode>(`/api/v1/nodes/${nodename}/detailed`),
                    axios.get<TypedMetrics>(`/api/v1/nodes/${nodename}/metrics`),
                ]);

                setNodeDetails(req.data);
                setRawMetrics(reqMetrics.data);
            } catch (err) {
                console.error(err);
            }
        };
        fetchData();

        const interval = setInterval(() => {
            if (context.enabled) {
                fetchData();
            }
        }, context.refreshTime * 1000);

        return () => clearInterval(interval);
    }, [context.enabled, context.refreshTime, nodename]);
    return (
        <ThemeProvider theme={defaultTheme}>
            <Box
                sx={{
                    m: '10px 70px 40px 70px',
                }}
            >
                <h2>Detailed Node Information</h2>

                <Box
                    sx={{
                        marginTop: '24px',
                        marginBottom: '24px',
                    }}
                >
                    <NodeTable nodes={[transform(nodeDetails)]} />
                </Box>
                <Box
                    sx={{
                        display: 'flex',
                        alignItems: 'flex-start',
                        gap: '15px',
                        height: '700px',
                    }}
                >
                    <Box
                        sx={{
                            display: 'flex',
                            flexDirection: 'column',
                            alignItems: 'flex-start',
                            gap: '16px',
                            width: '20%',
                        }}
                    >
                        <CurrentRps rps={nodeDetails.metrics.rps} />
                        <SystemChar
                            cpu={nodeDetails.metrics.cpuLoad}
                            ram={{
                                total: nodeDetails.metrics.totalRam,
                                used: nodeDetails.metrics.usedRam,
                            }}
                            fd={nodeDetails.metrics.descrAmount}
                        />
                    </Box>

                    <NodeVDisks vdisks={nodeDetails.vdisks} />

                    <Box
                        sx={{
                            display: 'flex',
                            flexDirection: 'column',
                            alignItems: 'flex-start',
                            gap: '16px',
                            width: '40%',
                        }}
                    >
                        <Box
                            sx={{
                                display: 'flex',
                                flexDirection: 'row',
                                alignItems: 'center',
                                justifyContent: 'center',
                                gap: '16px',
                                padding: '16px 28px',
                                backgroundColor: '#212329',
                                borderRadius: '8px',
                                width: '100%',
                            }}
                        >
                            <RawMetrics metrics={rawMetrics} />
                            <NodeSettings configs={config} />
                        </Box>
                        <DiskTable disks={nodeDetails.disks} />
                    </Box>
                </Box>
            </Box>
        </ThemeProvider>
    );
};

export default NodePage;
