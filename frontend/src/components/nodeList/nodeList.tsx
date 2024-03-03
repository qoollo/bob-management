import { Context } from '@appTypes/context.ts';
import defaultTheme from '@layouts/DefaultTheme.ts';
import { Box, ThemeProvider } from '@mui/system';
import { useStore } from '@nanostores/react';
import axios from 'axios';
import React, { useCallback, useEffect, useMemo, useState } from 'react';

import FetchingBackdrop from '../backdrop/backdrop.tsx';
import NodeTable from '../nodeTable/nodeTable.tsx';

const stubNode: NodeInfo = {
    name: 'loading...',
    hostname: 'loading...',
    vdisks: [],
    status: {
        status: 'Offline',
    },
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
    space: {
        total_disk: 0,
        free_disk: 0,
        used_disk: 0,
        occupied_disk: 0,
    },
};

const NodeListPage = () => {
    const [nodes, setNodes] = useState<NodeInfo[]>([]);
    const [nodeList, setNodeList] = useState<DTONode[]>([]);
    const [isPageLoaded, setIsPageLoaded] = useState(false);
    const context = useStore(Context);

    const fetchNodeList = useMemo(
        () => async () => {
            try {
                const [res] = await Promise.all([axios.get<DTONode[]>('/api/v1/nodes/list')]);
                setNodes(
                    res.data
                        .map((dtoNode: DTONode) => {
                            return {
                                ...stubNode,
                                name: dtoNode.name,
                                hostname: dtoNode.address,
                            } as NodeInfo;
                        })
                        .sort((a, b) => (a.name < b.name ? -1 : 1)),
                );
                setNodeList(res.data);
            } catch (err) {
                console.log(err);
            }
        },
        [],
    );

    const fetchNode = useCallback(
        (nodeName: string) => async () => {
            try {
                const [res] = await Promise.all([axios.get<NodeInfo>('/api/v1/nodes/' + nodeName)]);
                return res.data;
            } catch (err) {
                console.log(err);
            }
        },
        [],
    );

    useEffect(() => {
        const fetchNodes = async () => {
            const res = (
                await Promise.all(
                    nodeList.map(async (node) => {
                        return fetchNode(node.name)()
                            .catch(console.error)
                            .then((resultNode) => resultNode);
                    }),
                )
            ).filter((node): node is NodeInfo => {
                return typeof node !== undefined;
            });
            setNodes(res.concat(nodes.filter((item) => !res.find((n) => (n?.name || '') == item.name))));
        };
        if (!isPageLoaded && nodeList.length !== 0) {
            fetchNodes();
            setIsPageLoaded(true);
        }
        const interval = setInterval(() => {
            fetchNodes();
        }, context.refreshTime * 1000);

        return () => clearInterval(interval);
    }, [fetchNode, context.enabled, context.refreshTime, nodeList, nodes, isPageLoaded]);

    useEffect(() => {
        fetchNodeList();
    }, [fetchNodeList]);
    if (!isPageLoaded) {
        return <FetchingBackdrop />;
    }
    return (
        <ThemeProvider theme={defaultTheme}>
            <Box
                sx={{
                    marginLeft: '52px',
                    marginRight: '52px',
                    marginTop: '38px',
                    '&:hover': {
                        color: '#282A2F',
                    },
                    height: '820px',
                    backgroundColor: '#1F2125',
                    borderColor: '#2E2E33',
                    border: '1',
                }}
            >
                <NodeTable nodes={nodes} />
            </Box>
        </ThemeProvider>
    );
};

export default NodeListPage;
