import { Context } from '@appTypes/context.ts';
import defaultTheme from '@layouts/DefaultTheme.ts';
import { Box, ThemeProvider } from '@mui/system';
import { useStore } from '@nanostores/react';
import axios from 'axios';
import React, { useCallback, useEffect, useMemo, useState } from 'react';

import FetchingBackdrop from '../backdrop/backdrop.tsx';
import VDiskTable from '../VDiskTable/VDiskTable.tsx';

const stubVDisk: VDisk = {
    id: 0,
    status: 'Offline',
    partition_count: 0,
    replicas: [],
};

axios.defaults.withCredentials = true;

const VDiskPage = () => {
    const [vdisks, setVdisks] = useState<VDisk[]>([]);
    const [vdiskList, setVdiskList] = useState<DTOVDisk[]>([]);
    const [isPageLoaded, setIsPageLoaded] = useState(false);
    const context = useStore(Context);

    const fetchVdiskList = useMemo(
        () => async () => {
            try {
                const [res] = await Promise.all([axios.get<DTOVDisk[]>('/api/v1/vdisks/list')]);
                setVdisks(
                    res.data
                        .map((dtoVdisk: DTOVDisk) => {
                            return {
                                ...stubVDisk,
                                id: dtoVdisk.id,
                            } as VDisk;
                        })
                        .sort((a, b) => (a.id < b.id ? -1 : 1)),
                );
                setVdiskList(res.data);
            } catch (err) {
                console.log(err);
            }
        },
        [],
    );

    const fetchVdisk = useCallback(
        (vdisk: number) => async () => {
            try {
                const [res] = await Promise.all([axios.get<VDisk>('/api/v1/vdisks/' + vdisk)]);
                return res.data;
            } catch (err) {
                console.log(err);
            }
        },
        [],
    );
    useEffect(() => {
        fetchVdiskList();
    }, [fetchVdiskList]);

    useEffect(() => {
        const fetchNodes = async () => {
            const res = (
                await Promise.all(
                    vdiskList.map(async (vdisk) => {
                        return fetchVdisk(vdisk.id)()
                            .catch(console.error)
                            .then((resultVdisk) => resultVdisk);
                    }),
                )
            ).filter((vdisk): vdisk is VDisk => {
                return typeof vdisk !== undefined;
            });
            setVdisks(res.concat(vdisks.filter((item) => !res.find((n) => (n?.id || '') == item.id))));
        };
        if (!isPageLoaded && vdiskList.length !== 0) {
            fetchNodes();
            setIsPageLoaded(true);
        }
        const interval = setInterval(() => {
            fetchNodes();
        }, context.refreshTime * 1000);

        return () => clearInterval(interval);
    }, [isPageLoaded, fetchVdisk, context.enabled, context.refreshTime, vdiskList, vdisks]);

    if (!isPageLoaded) {
        return <FetchingBackdrop />;
    }
    return (
        <ThemeProvider theme={defaultTheme}>
            <Box
                sx={{
                    marginLeft: '52px',
                    marginRight: '52px',
                    marginTop: '38px',
                    '&:hover': {
                        color: '#282A2F',
                    },
                    height: '820px',
                    backgroundColor: '#1F2125',
                    borderColor: '#2E2E33',
                    border: '1',
                }}
            >
                <VDiskTable vdisks={vdisks} />
            </Box>
        </ThemeProvider>
    );
};

export default VDiskPage;
