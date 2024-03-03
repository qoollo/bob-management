import { formatBytes } from '@appTypes/common.ts';
import { Box } from '@mui/material';
import type { GridColDef, GridRenderCellParams, GridValidRowModel } from '@mui/x-data-grid';
import { DataGrid, GridToolbar } from '@mui/x-data-grid';
import React from 'react';

import style from './diskTable.module.css';

const DotColor: Record<DiskStatusName, string> = {
    Good: style.greendot,
    Bad: style.graydot,
    Offline: style.reddot,
};

interface Space {
    totalSpace: number;
    usedSpace: number;
}

const columns: GridColDef[] = [
    {
        field: 'name',
        flex: 4,
        headerName: 'Disk Name',
        align: 'center',
        headerAlign: 'center',
    },
    {
        field: 'status',
        flex: 5,
        headerName: 'Status',
        align: 'left',
        headerAlign: 'center',
        renderCell: (params: GridRenderCellParams<GridValidRowModel, DiskStatus>) => {
            const status = params.value?.status || 'Offline';
            return (
                <Box
                    sx={{
                        display: 'flex',
                        flexDirection: 'row',
                        alignItems: 'center',
                        gap: '18px',
                    }}
                >
                    <span className={DotColor[status]}></span>
                    <span style={{ width: '15px' }}>{status}</span>
                </Box>
            );
        },
    },
    {
        field: 'usedspace',
        headerName: 'Used Space',
        align: 'center',
        flex: 4,
        headerAlign: 'center',
        renderCell: (params: GridRenderCellParams<GridValidRowModel, Space>) => {
            return (
                <div>
                    {formatBytes(params.value?.usedSpace || 0)} /{' '}
                    <span className={style.totalspace}>{formatBytes(params.value?.totalSpace || 0)}</span>
                </div>
            );
        },
    },
    {
        field: 'ops',
        flex: 4,
        headerName: 'Operations / sec.',
        align: 'center',
        headerAlign: 'center',
    },
];

const DiskTable = ({ disks }: { disks: Disk[] }) => {
    const data = disks
        ? disks
            .map((disk, i) => {
                return {
                    id: i,
                    name: disk.name,
                    ops: disk.iops,
                    status: disk.status,
                    usedspace: { totalSpace: disk.totalSpace, usedSpace: disk.usedSpace } as Space,
                } as DiskTableCols;
            })
            .sort((a, b) => (a.name < b.name ? 1 : -1))
        : [];
    return (
        <Box
            sx={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                justifyContent: 'center',
                padding: '16px 28px',
                gap: '14px',
                height: '580px',
                backgroundColor: '#212329',
                width: '100%',
                borderRadius: '8px',
            }}
        >
            <DataGrid
                rows={data}
                columns={columns}
                sx={{
                    width: '100%',
                }}
                initialState={{
                    filter: {
                        filterModel: {
                            items: [],
                            quickFilterValues: [],
                            quickFilterExcludeHiddenColumns: true,
                        },
                    },
                }}
                disableColumnFilter
                disableColumnSelector
                disableDensitySelector
                slots={{ toolbar: GridToolbar }}
                slotProps={{
                    toolbar: {
                        showQuickFilter: true,
                        quickFilterProps: {
                            debounceMs: 500,
                            quickFilterParser: (searchInput) => searchInput.split(',').map((value) => value.trim()),
                        },
                    },
                }}
            />
        </Box>
    );
};

export default DiskTable;
