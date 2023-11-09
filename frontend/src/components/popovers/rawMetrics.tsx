import { Box, Button, Divider, List, ListItem, ListSubheader, Paper, Popper } from '@mui/material';
import React, { useState } from 'react';

const TableRow = ({ left, right }: { left: string; right: string }) => {
    return (
        <Box
            sx={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                gap: '20px',
                width: '100%',
            }}
        >
            <span
                style={{
                    width: '50%',
                    textAlign: 'center',
                    fontSize: '12px',
                    wordBreak: 'break-all',
                }}
            >
                {left}
            </span>
            <Divider orientation="vertical" flexItem />
            <span style={{ width: '50%', textAlign: 'center', fontSize: '12px' }}>{right}</span>
        </Box>
    );
};

const RawMetrics = ({ metrics }: { metrics: TypedMetrics }) => {
    const [anchorEl, setAnchorEl] = useState<HTMLButtonElement | null>(null);

    const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
        setAnchorEl(anchorEl ? null : event.currentTarget);
    };

    const open = Boolean(anchorEl);
    const id = open ? 'simple-popper' : undefined;

    const metricsList = Object.entries(metrics.map)
        .sort()
        .map((metric, i) => {
            return (
                <ListItem key={i} component="div">
                    <TableRow left={metric[0]} right={metric[1].value.toString()} />
                </ListItem>
            );
        });

    return (
        <div style={{ width: '50%', flexGrow: 1 }}>
            <Button
                aria-describedby={id}
                onClick={handleClick}
                variant="contained"
                sx={{
                    backgroundColor: '#282A30',
                    '&:hover': {
                        backgroundColor: '#282A2F',
                    },
                    color: '#ffffff',
                    borderRadius: '8px',
                    fontSize: '13px',
                    height: '50px',
                    width: '100%',
                }}
            >
                Show Raw Metrics
            </Button>
            <Popper
                id={id}
                open={open}
                anchorEl={anchorEl}
                style={{ zIndex: 1000 }}
                disablePortal={true}
                modifiers={[
                    {
                        name: 'flip',
                        enabled: false,
                        options: {
                            altBoundary: true,
                            rootBoundary: 'document',
                            padding: 8,
                        },
                    },
                    {
                        name: 'preventOverflow',
                        enabled: false,
                        options: {
                            altAxis: true,
                            altBoundary: true,
                            tether: true,
                            rootBoundary: 'document',
                            padding: 8,
                        },
                    },
                ]}
            >
                <Paper
                    style={{
                        maxHeight: 610,
                        overflow: 'auto',
                        width: '280px',
                    }}
                >
                    <List
                        subheader={
                            <ListSubheader>
                                <TableRow left="Metric" right="Value" />
                            </ListSubheader>
                        }
                    >
                        {metricsList}
                    </List>
                </Paper>
            </Popper>
        </div>
    );
};

export default RawMetrics;
