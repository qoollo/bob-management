import { cookieAuthId, eraseCookie, getCookie, refreshTimes } from '@components/common.ts';
import { Context } from '@components/Context.ts';
import defaultTheme from '@layouts/DefaultTheme.ts';
import { ExitToApp } from '@mui/icons-material';
import {
    AppBar,
    Box,
    FormControl,
    FormControlLabel,
    IconButton,
    InputLabel,
    Link,
    MenuItem,
    Select,
    Switch,
    Tab,
    Tabs,
    Toolbar,
} from '@mui/material';
import { ThemeProvider } from '@mui/material/styles';
import { useStore } from '@nanostores/react';
import BrandMark from 'public/brandmark.svg';
import React, { useEffect, useState } from 'react';
// import type { RefreshTimes } from '../../env.d.ts';

const Navbar = ({ logoutRedirectTo }: { logoutRedirectTo: string }) => {
    const context = useStore(Context);
    const [value, setValue] = useState(0);
    const [refresh, setRefresh] = useState(context.refreshTime);
    const [switchButton, setSwitchButton] = useState(context.enabled);

    useEffect(() => {
        const path = window.location.pathname;
        if (path === '/dashboard') {
            setValue(0);
        } else if (path === '/nodelist') {
            setValue(1);
        } else if (path === '/vdisklist') {
            setValue(2);
        }
    }, []);

    const updateContext = () => {
        Context.set({
            refreshTime: refresh,
            enabled: switchButton,
        });
    };

    const pathname = window.location.pathname.replace(/\/$/, '');
    if (getCookie('id') === '' && pathname !== '/login') {
        location.assign(logoutRedirectTo);
    }

    if (pathname === '/login') {
        return <div></div>;
    }

    async function handleLogout() {
        await fetch('/api/v1/logout', {
            method: 'POST',
        });
        eraseCookie(cookieAuthId);
        location.assign(logoutRedirectTo);
    }

    return (
        <ThemeProvider theme={defaultTheme}>
            <AppBar position="static" style={{ background: '#000000', padding: '16px 22px' }} sx={{ width: '100%' }}>
                <Toolbar>
                    <Box
                        sx={{
                            display: 'flex',
                            flexDirection: 'row',
                            alignItems: 'center',
                            gap: '45px',
                            width: '60%',
                        }}
                    >
                        <img
                            src={BrandMark.src}
                            width={30}
                            height={30}
                            alt="BrandMark"
                            style={{ marginRight: '25px' }}
                        />
                        <Tabs
                            indicatorColor="primary"
                            textColor="secondary"
                            value={value}
                            onChange={(e, val) => {
                                setValue(val);
                                updateContext();
                            }}
                        >
                            <Tab
                                label="Monitoring"
                                component={Link}
                                href="dashboard"
                                style={{ textTransform: 'none', fontSize: '16px' }}
                            />
                            <Tab
                                label="Nodes"
                                component={Link}
                                href="nodelist"
                                style={{ textTransform: 'none', fontSize: '16px' }}
                            />
                            <Tab
                                label="Virtual disks"
                                component={Link}
                                href="vdisklist"
                                style={{ textTransform: 'none', fontSize: '16px' }}
                            />
                        </Tabs>
                    </Box>
                    <Box
                        sx={{
                            display: 'flex',
                            flexDirection: 'row',
                            alignItems: 'center',
                            justifyContent: 'flex-end',
                            gap: '25px',
                            width: '40%',
                        }}
                    >
                        <FormControlLabel
                            value="stop"
                            control={
                                <Switch
                                    onChange={(e, c) => {
                                        setSwitchButton(c);
                                        updateContext();
                                    }}
                                />
                            }
                            label="STOP"
                            labelPlacement="start"
                            sx={{
                                '&.MuiFormControlLabel-labelPlacementStart': {
                                    color: '#FF6936',
                                },
                            }}
                        />
                        <span>Refresh time</span>
                        <FormControl
                            variant="standard"
                            sx={{
                                width: '85px',
                            }}
                        >
                            <InputLabel id="polling-label-id" style={{ marginTop: '-20px' }}>
                                minutes
                            </InputLabel>
                            <Select
                                style={{ marginTop: '-5px' }}
                                labelId="polling-select-label-id"
                                id="pollint-select-id"
                                label="min"
                                value={refresh}
                                onChange={(e) => {
                                    setRefresh(e.target.value);
                                    updateContext();
                                }}
                            >
                                {refreshTimes.map((val: string) => (
                                    <MenuItem key={val} value={val}>
                                        {val}
                                    </MenuItem>
                                ))}
                            </Select>
                        </FormControl>
                        <Box
                            sx={{
                                marginTop: '-10px',
                                cursor: 'pointer',
                                marginLeft: '+70px',
                            }}
                        >
                            <IconButton
                                onClick={() => {
                                    handleLogout();
                                    updateContext();
                                }}
                                color="inherit"
                            >
                                <ExitToApp fontSize="large" color="primary" />
                            </IconButton>
                        </Box>
                    </Box>
                </Toolbar>
            </AppBar>
        </ThemeProvider>
    );
};

export default Navbar;
