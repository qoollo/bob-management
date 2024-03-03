import { Box } from '@mui/system';
import {
    DataGrid,
    type GridColDef,
    type GridRenderCellParams,
    GridToolbar,
    type GridValidRowModel,
} from '@mui/x-data-grid';
import React from 'react';

import style from './VDiskTable.module.css';

const BarLabelColor: Record<VDiskStatus, string> = {
    Good: style.greendot,
    Bad: style.graydot,
    Offline: style.reddot,
};

const columns: GridColDef[] = [
    {
        field: 'vdiskid',
        headerName: 'VDisk Number',
        flex: 1,
        align: 'center',
        headerAlign: 'center',
        headerClassName: style.greyHeader,
    },
    {
        field: 'replicas',
        headerName: 'Replicas on nodes',
        flex: 3,
        align: 'center',
        headerAlign: 'center',
        headerClassName: style.greyHeader,
        renderCell: (params: GridRenderCellParams<GridValidRowModel, Replica[]>) => {
            return (
                <Box
                    sx={{
                        display: 'flex',
                        flexDirection: 'row',
                        alignItems: 'center',
                        gap: '18px',
                    }}
                >
                    <span>{params.value?.map((replica) => replica.node).join(', ') || ''}</span>
                </Box>
            );
        },
    },
    {
        field: 'availability',
        headerName: 'Availability',
        flex: 1,
        align: 'center',
        headerAlign: 'center',
        headerClassName: style.greyHeader,
        renderCell: (params: GridRenderCellParams<GridValidRowModel, ReplicaCount>) => {
            return (
                <Box
                    sx={{
                        display: 'flex',
                        flexDirection: 'row',
                        alignItems: 'center',
                        gap: '18px',
                    }}
                >
                    <span>
                        {params.value?.goodReplicas || 0} / {params.value?.totalReplicas || 0}
                    </span>
                </Box>
            );
        },
    },
    {
        field: 'status',
        headerName: 'Status',
        flex: 1,
        align: 'left',
        headerAlign: 'center',
        headerClassName: style.greyHeader,
        renderCell: (params: GridRenderCellParams<GridValidRowModel, VDiskStatus>) => {
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
                    <span className={BarLabelColor[status]}></span>
                    <span>{params.value}</span>
                </Box>
            );
        },
    },
];
const VDiskTable = ({ vdisks }: { vdisks: VDisk[] }) => {
    const data = vdisks.sort()
        ? vdisks.map((vdisk, i) => {
              return {
                  id: i,
                  vdiskid: vdisk.id,
                  replicas: vdisk.replicas,
                  availability: {
                      goodReplicas: vdisk.replicas.filter((replica) => replica.status.status === 'Good').length,
                      totalReplicas: vdisk.replicas.length,
                  },
                  status: vdisk.status,
              } as VDiskTableCols;
          })
        : [];
    return (
        <DataGrid
            rows={data}
            // rows={[]}
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

export default VDiskTable;
