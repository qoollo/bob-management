import { removeEmpty } from '@appTypes/common.ts';
import defaultTheme from '@layouts/DefaultTheme.ts';
import ThemeRegistry from '@layouts/ThemeRegistry.tsx';
import { Alert, Box, Button, Grid, Snackbar, TextField } from '@mui/material';
import BobLogo from 'public/logo.svg';
import React, { type FormEvent, useState } from 'react';

import style from './login.module.css';

const LoginPage = ({ redirectTo }: { redirectTo: string }) => {
    const [address, setAddress] = useState('');
    const [port, setPort] = useState('');
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [openSnackbar, setOpenSnackbar] = useState(false);
    const [snackbarMessage, setSnackbarMessage] = useState('');

    const snackbarStyle = {
        top: '20%',
        left: '50%',
        transform: 'translate(-50%, -50%)',
    };

    async function handleSubmit(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();
        const response = await fetch('/api/v1/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json;charset=utf-8',
            },
            body: JSON.stringify(
                removeEmpty({
                    hostname: address + ':' + port,
                    credentials: {
                        login: username,
                        password: password,
                    },
                }),
            ),
        });
        if (response.ok) {
            location.replace(redirectTo);
        } else {
            setSnackbarMessage('Wrong data');
            setOpenSnackbar(true);
            console.log(response.json());
        }
    }

    return (
        <ThemeRegistry options={{ key: 'mui' }} theme={defaultTheme}>
            <Box className={style.main}>
                <Box
                    component="form"
                    onSubmit={handleSubmit}
                    className={style.form}
                    sx={{
                        width: 400,
                        padding: '30px',
                        borderRadius: '8px',
                        boxShadow: '0px 4px 8px rgba(0, 0, 0, 0.1)',
                    }}
                >
                    <Box sx={{ display: 'flex', justifyContent: 'center', mb: 4 }}>
                        <img src={BobLogo.src} width={200} height={60} alt="BobLogo" />
                    </Box>
                    <Snackbar
                        open={openSnackbar}
                        autoHideDuration={3000}
                        onClose={() => setOpenSnackbar(false)}
                        style={snackbarStyle}
                    >
                        <Alert onClose={() => setOpenSnackbar(false)} severity="error">
                            {snackbarMessage}
                        </Alert>
                    </Snackbar>
                    <Grid container spacing={2}>
                        <Grid item xs={12} sm={8}>
                            <TextField
                                onChange={(e) => setAddress(e.target.value)}
                                required
                                fullWidth
                                label="Address"
                                name="address"
                                id="address"
                                autoFocus
                                variant="outlined"
                            />
                        </Grid>
                        <Grid item xs={12} sm={4}>
                            <TextField
                                onChange={(e) => setPort(e.target.value)}
                                required
                                fullWidth
                                label="Port"
                                name="port"
                                id="port"
                                variant="outlined"
                            />
                        </Grid>
                        <Grid item xs={12}>
                            <TextField
                                onChange={(e) => setUsername(e.target.value)}
                                fullWidth
                                label="Login"
                                variant="outlined"
                            />
                        </Grid>
                        <Grid item xs={12}>
                            <TextField
                                onChange={(e) => setPassword(e.target.value)}
                                fullWidth
                                label="Password"
                                type="password"
                                variant="outlined"
                            />
                        </Grid>
                    </Grid>
                    <Box sx={{ display: 'flex', justifyContent: 'center', mt: 4 }}>
                        <Button type="submit" variant="contained" color="primary" size="large" sx={{ width: '100%' }}>
                            Authorize
                        </Button>
                    </Box>
                </Box>
            </Box>
        </ThemeRegistry>
    );
};

export default LoginPage;
