import { Box, LinearProgress } from '@mui/material';
import React from 'react';

import style from './diskState.module.css';

const BarColor: Record<DiskStatusName, string> = {
    Good: '#5EB36B',
    Bad: '#7C817E',
    Offline: '#B3344D',
};

const BarLabelColor: Record<DiskStatusName, string> = {
    Good: style.totalGoodDisksLabel,
    Bad: style.totalBadDisksLabel,
    Offline: style.totalOfflineDisksLabel,
};

const DiskState = ({ diskCount, status }: { diskCount: Record<DiskStatusName, number>; status: DiskStatusName }) => {
    const total = diskCount.Good + diskCount.Bad + diskCount.Offline;
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
            <DiskState diskCount={count} status="Good" />
            <DiskState diskCount={count} status="Bad" />
            <DiskState diskCount={count} status="Offline" />
        </Box>
    );
};

export default DiskBreakdown;
