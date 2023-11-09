import { cookieAuthId, eraseCookie, getCookie } from '@appTypes/common.ts';
import { Context, refreshTimes } from '@appTypes/context.ts';
import { isLocation, type NavLocation } from '@appTypes/navigation.ts';
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
import { navigate } from 'astro:transitions/client';
import BrandMark from 'public/brandmark.svg';
import React, { useEffect, useState } from 'react';

const Navbar = ({ logoutRedirectTo }: { logoutRedirectTo: string }) => {
    const context = useStore(Context);
    const [tab, setTab] = useState('/dashboard' as NavLocation);
    const [refresh, setRefresh] = useState(context.refreshTime);
    // FIXME: button's render is not on the same state on page refresh as the context (always on)
    // I hate react....
    const [switchButton, setSwitchButton] = useState(context.enabled);

    const path = window.location.pathname.replace(/\/$/, '');
    useEffect(() => {
        // setSwitchButton(Context.get().enabled);
        if (isLocation(path)) {
            setTab(path);
        }
    }, [setSwitchButton, context, switchButton, path]);

    if (getCookie('id') === '' && path !== '/login') {
        // location.replace(logoutRedirectTo);
        navigate(logoutRedirectTo);
    }

    if (path === '/login') {
        return <div></div>;
    }

    async function handleLogout() {
        await fetch('/api/v1/logout', {
            method: 'POST',
        });
        eraseCookie(cookieAuthId);
        navigate(logoutRedirectTo);
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
                            value={tab}
                            onChange={(e, val) => {
                                setTab(val);
                            }}
                        >
                            <Tab
                                label="Monitoring"
                                component={Link}
                                href="/dashboard"
                                value={'/dashboard' as NavLocation}
                                style={{ textTransform: 'none', fontSize: '16px' }}
                            />
                            <Tab
                                label="Nodes"
                                component={Link}
                                href="/nodelist"
                                value={'/nodelist' as NavLocation}
                                style={{ textTransform: 'none', fontSize: '16px' }}
                            />
                            <Tab
                                label="Virtual disks"
                                component={Link}
                                href="/vdisklist"
                                value={'/vdisklist' as NavLocation}
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
                            value={switchButton}
                            control={switchButton ? <Switch defaultChecked /> : <Switch />}
                            onChange={() => {
                                setSwitchButton(!switchButton);
                                Context.setKey('enabled', !switchButton);
                            }}
                            label={'STOP: ' + switchButton}
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
                                // value={refresh}
                                defaultValue={refresh}
                                onChange={(e) => {
                                    setRefresh(e.target.value);
                                    Context.setKey('refreshTime', e.target.value);
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
