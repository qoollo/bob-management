import { Box, LinearProgress } from '@mui/material';
import React from 'react';

import style from './diskState.module.css';

const BarColor: Record<DiskStatusName, string> = {
    good: '#5EB36B',
    bad: '#7C817E',
    offline: '#B3344D',
};

const BarLabelColor: Record<DiskStatusName, string> = {
    good: style.totalGoodDisksLabel,
    bad: style.totalBadDisksLabel,
    offline: style.totalOfflineDisksLabel,
};

const DiskState = ({ diskCount, status }: { diskCount: Record<DiskStatusName, number>; status: DiskStatusName }) => {
    const total = diskCount.good + diskCount.bad + diskCount.offline;
    const percent = Math.floor((diskCount[status] / total) * 100) || 0;
    return (
        <Box
            sx={{
                display: 'flex',
                flexDirection: 'row',
                alignItems: 'center',
                gap: '20px',
            }}
        >
            <p className={BarLabelColor[status]}>{diskCount[status]}</p>
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
                        backgroundColor: BarColor[status],
                        borderRadius: '35px',
                    },
                }}
            />
        </Box>
    );
};

const DiskBreakdown = ({ diskCount: { map: count } }: { diskCount: DiskCount }) => {
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
            <p style={{ fontSize: '16px' }}>State of the physical disks in the cluster</p>
            <DiskState diskCount={count} status="good" />
            <DiskState diskCount={count} status="bad" />
            <DiskState diskCount={count} status="offline" />
        </Box>
    );
};

export default DiskBreakdown;
