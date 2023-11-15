import { Backdrop, CircularProgress } from '@mui/material';
import { Box } from '@mui/system';
import React from 'react';

const FetchingBackdrop = () => {
    return (
        <Backdrop sx={{ color: '#fff', zIndex: (theme) => theme.zIndex.drawer + 1 }} open={true}>
            <Box
                sx={{
                    display: 'flex',
                    flexDirection: 'row',
                    alignItems: 'center',
                    gap: '20px',
                }}
            >
                <p>Data is fetching. Please, wait...</p>
                <CircularProgress color="inherit" />
            </Box>
        </Backdrop>
    );
};

export default FetchingBackdrop;
