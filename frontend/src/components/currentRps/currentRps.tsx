import { Box } from '@mui/material';
import React from 'react';

import style from './currentRps.module.css';

const OperationColor: Record<Operation, string> = {
    put: style.reddot,
    get: style.orangedot,
    exist: style.graydot,
    delete: style.greendot,
};

const RpsLine = ({ type, rps }: { type: Operation; rps: number }) => {
    return (
        <Box
            sx={{
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                gap: '27px',
                width: '250px',
            }}
        >
            <span className={OperationColor[type]}></span>
            <span style={{ minWidth: '50px' }}>{type.toUpperCase()}</span>
            <span
                style={{
                    width: '90px',
                    backgroundColor: '#2F3136',
                    textAlign: 'center',
                }}
            >
                {rps}
            </span>
        </Box>
    );
};

const CurrentRps = ({ rps: { map: rps } }: { rps: RPS }) => {
    return (
        <Box
            sx={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                padding: '14px 18px 30px 18px',
                gap: '38px',
                backgroundColor: '#212329',
                borderRadius: '8px',
                width: '100%',
                height: '340px',
            }}
        >
            <span>Total RPS per operation:</span>
            <RpsLine type={'put'} rps={rps.put} />
            <RpsLine type={'get'} rps={rps.get} />
            <RpsLine type={'exist'} rps={rps.exist} />
            <RpsLine type={'delete'} rps={rps.delete} />
        </Box>
    );
};

export default CurrentRps;
