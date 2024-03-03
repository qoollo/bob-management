import { formatBytes } from '@appTypes/common.ts';
import { Box } from '@mui/material';
import React from 'react';

const SystemLine = ({ left, right }: { left: string; right: string }) => {
    return (
        <Box
            sx={{
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                gap: '15px',
            }}
        >
            <span style={{ width: '50%', fontSize: 14, textAlign: 'left' }}>{left}</span>
            <span
                style={{
                    width: '50%',
                    lineHeight: '25px',
                    textAlign: 'center',
                    backgroundColor: '#2F3136',
                }}
            >
                {right}
            </span>
        </Box>
    );
};

const SystemChar = ({ cpu, ram, fd }: { cpu: number; ram: { total: number; used: number }; fd: number }) => {
    return (
        <Box
            sx={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'stretch',
                gap: '42px',
                borderRadius: '8px',
                backgroundColor: '#212329',
                padding: '16px 14px 30px 14px',
                width: '100%',
                height: '320px',
            }}
        >
            <h2>System characteristics</h2>
            <SystemLine left="CPU Load" right={`${cpu}%`} />
            <SystemLine left="RAM Load" right={`${formatBytes(ram.used)}/${formatBytes(ram.total)}`} />
            <SystemLine left="File Descriptors" right={`${fd}`} />
        </Box>
    );
};

export default SystemChar;
