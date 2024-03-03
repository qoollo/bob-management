import { Box, LinearProgress } from '@mui/material';
import React from 'react';

import style from './totalNodes.module.css';

const NodeColor: Record<NodeStatusName, string> = {
    Good: '#5EB36B',
    Bad: '#7C817E',
    Offline: '#B3344D',
};

const NodeLabelColor: Record<NodeStatusName, string> = {
    Good: style.totalGoodNodesLabel,
    Bad: style.totalBadNodesLabel,
    Offline: style.totalOfflineNodesLabel,
};

const NodeState = ({ nodeCount, status }: { nodeCount: Record<NodeStatusName, number>; status: NodeStatusName }) => {
    const total = nodeCount.Good + nodeCount.Bad + nodeCount.Offline;
    const percent = Math.floor((nodeCount[status] / total) * 100) || 0;
    return (
        <Box
            sx={{
                display: 'flex',
                flexDirection: 'row',
                alignItems: 'center',
                gap: '20px',
            }}
        >
            <p className={NodeLabelColor[status]}>{nodeCount[status]}</p>
            <p className={style.titleLabel}>{status.charAt(0).toUpperCase() + status.slice(1)}</p>
            <Box sx={{ flex: 1, textAlign: 'right' }}>
                <p className={style.totalLabelPercent}>{percent}%</p>
            </Box>
            <LinearProgress
                value={percent}
                id="workingProgress"
                variant="determinate"
                style={{
                    width: '70%',
                }}
                sx={{
                    height: '20px',
                    backgroundColor: '#2E2E2E',
                    borderRadius: '35px',
                    '& .MuiLinearProgress-bar': {
                        backgroundColor: NodeColor[status],
                        borderRadius: '35px',
                    },
                }}
            />
        </Box>
    );
};

const TotalNodes = ({ nodeCount: { map: count } }: { nodeCount: NodeCount }) => {
    return (
        <Box
            sx={{
                display: 'flex',
                flexDirection: 'column',
                padding: '16px',
                backgroundColor: '#212328',
                boxShadow: '0px 4px 4px 0px rgba(0, 0, 0, 0.25)',
                borderRadius: '8px',
                gap: '4px',
            }}
        >
            <p style={{ fontSize: '16px' }}>State of the nodes in the cluster</p>
            <NodeState nodeCount={count} status="Good" />
            <NodeState nodeCount={count} status="Bad" />
            <NodeState nodeCount={count} status="Offline" />
        </Box>
    );
};

export default TotalNodes;
