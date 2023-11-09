import { formatBytes } from '@appTypes/common.ts';
import { Link } from '@mui/material';
import { Box } from '@mui/system';
import type { GridColDef, GridRenderCellParams, GridValidRowModel } from '@mui/x-data-grid';
import { DataGrid, GridToolbar } from '@mui/x-data-grid';
import axios from 'axios';
import React from 'react';

import style from './nodeTable.module.css';

axios.defaults.withCredentials = true;

const DotMap: Record<NodeStatusName, string> = {
    Good: style.greendot,
    Bad: style.graydot,
    Offline: style.reddot,
};

const defaultRps: RPS = {
    map: {
        put: 0,
        exist: 0,
        get: 0,
        delete: 0,
    },
};

const defaultSpace: SpaceInfo = {
    total_disk: 0,
    used_disk: 0,
    occupied_disk: 0,
    free_disk: 0,
};

const columns: GridColDef[] = [
    {
        field: 'nodename',
        headerName: 'Node Name',
        flex: 1,
        width: 200,
        align: 'center',
        headerAlign: 'center',
        headerClassName: style.greyHeader,
        sortable: false,
        renderCell: (params: GridRenderCellParams<GridValidRowModel, string>) => {
            return (
                <Link href={`/?node=${params.value}`}>
                    <b>{params.value}</b>
                </Link>
            );
        },
    },
    {
        field: 'hostname',
        headerName: 'Hostname',
        flex: 1,
        width: 200,
        align: 'center',
        headerAlign: 'center',
        headerClassName: style.greyHeader,
    },
    {
        field: 'status',
        headerName: 'Status',
        flex: 1,
        width: 200,
        align: 'left',
        headerAlign: 'center',
        headerClassName: style.greyHeader,
        renderCell: (params: GridRenderCellParams<GridValidRowModel, NodeStatusName>) => {
            const status = params.value || 'Offline';
            return (
                <Box
                    sx={{
                        display: 'flex',
                        flexDirection: 'row',
                        alignItems: 'center',
                        gap: '18px',
                    }}
                >
                    <span className={DotMap[status]}></span>
                    {status.charAt(0).toUpperCase() + status.slice(1)}
                </Box>
            );
        },
    },
    {
        field: 'space',
        headerName: 'Occupied Space',
        flex: 1,
        width: 150,
        align: 'center',
        headerAlign: 'center',
        headerClassName: style.greyHeader,
        renderCell: (params: GridRenderCellParams<GridValidRowModel, SpaceInfo>) => {
            const space = params.value || defaultSpace;
            return (
                <div>
                    {formatBytes(space.used_disk)} /{' '}
                    <span className={style.totalspace}>{formatBytes(space.total_disk)}</span>
                </div>
            );
        },
    },
    {
        field: 'rps',
        headerName: 'RPS',
        flex: 1,
        width: 150,
        align: 'center',
        headerAlign: 'center',
        headerClassName: style.greyHeader,
        renderCell: (params: GridRenderCellParams<GridValidRowModel, RPS>) => {
            const rps = (params.value || defaultRps).map;
            return <div>{rps.get + rps.put + rps.exist + rps.delete}</div>;
        },
    },
    {
        field: 'aliens',
        headerName: 'Aliens',
        flex: 1,
        width: 200,
        align: 'center',
        headerAlign: 'center',
        headerClassName: style.greyHeader,
    },
    {
        field: 'corruptedBlobs',
        headerName: 'Corrupted BLOBs',
        flex: 1,
        width: 200,
        align: 'center',
        headerAlign: 'center',
        headerClassName: style.greyHeader,
    },
];

const NodeTable = ({ nodes }: { nodes: NodeInfo[] }) => {
    const data = nodes.sort()
        ? nodes.map((node, i) => {
              return {
                  id: i,
                  nodename: node.name,
                  hostname: node.hostname,
                  status: node.status.status,
                  space: node.space,
                  rps: node.rps,
                  aliens: node.alienCount || 0,
                  corruptedBlobs: node.corruptedCount || 0,
              } as NodeTableCols;
          })
        : [];
    return (
        <DataGrid
            rows={data}
            columns={columns}
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
    );
};

export default NodeTable;
