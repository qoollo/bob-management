/* eslint-disable @typescript-eslint/no-shadow */
import createCache from '@emotion/cache';
import { CacheProvider } from '@emotion/react';
import defaultTheme from '@layouts/DefaultTheme.ts';
import { CssBaseline, ThemeProvider, Typography } from '@mui/material';
import * as React from 'react';

// This implementation is from emotion-js
// https://github.com/emotion-js/emotion/issues/2928#issuecomment-1319747902
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export default function ThemeRegistry(props: any) {
    const { options, children } = props;

    // eslint-disable-next-line unused-imports/no-unused-vars
    const [{ cache, flush }] = React.useState(() => {
        const cache = createCache(options);
        cache.compat = true;
        const prevInsert = cache.insert;
        let inserted: string[] = [];
        cache.insert = (...args) => {
            const serialized = args[1];
            if (cache.inserted[serialized.name] === undefined) {
                inserted.push(serialized.name);
            }
            return prevInsert(...args);
        };
        const flush = () => {
            const prevInserted = inserted;
            inserted = [];
            return prevInserted;
        };
        return { cache, flush };
    });
    return (
        <CacheProvider value={cache}>
            <ThemeProvider theme={defaultTheme}>
                <Typography component={'span'} variant={'body2'}>
                    <CssBaseline />
                    {children}
                </Typography>
            </ThemeProvider>
        </CacheProvider>
    );
}
