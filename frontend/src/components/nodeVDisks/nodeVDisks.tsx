import { Box } from '@mui/material';
import type { GridColDef, GridRenderCellParams, GridValidRowModel } from '@mui/x-data-grid';
import { DataGrid, GridToolbar } from '@mui/x-data-grid';
import React from 'react';

import style from './nodeVDisks.module.css';

const DotColor: Record<VDiskStatus, string> = {
    Good: style.greendot,
    Bad: style.graydot,
    Offline: style.reddot,
};

const columns: GridColDef[] = [
    {
        field: 'vdiskid',
        headerName: 'VDisk ID',
        align: 'center',
        headerAlign: 'center',
        flex: 1,
    },
    {
        field: 'status',
        headerName: 'Status',
        align: 'left',
        flex: 1,
        headerAlign: 'center',
        renderCell: (params: GridRenderCellParams<GridValidRowModel, VDiskStatus>) => {
            return (
                <Box
                    sx={{
                        display: 'flex',
                        flexDirection: 'row',
                        alignItems: 'center',
                        gap: '18px',
                    }}
                >
                    <span className={DotColor[params.value || 'Offline']}></span>
                    {params.value || 'Offline'}
                </Box>
            );
        },
    },
    {
        field: 'partitionNumber',
        headerName: 'Partition Number',
        align: 'center',
        flex: 1,
        headerAlign: 'center',
        renderCell: (params: GridRenderCellParams<GridValidRowModel, number>) => {
            return (
                <Box
                    sx={{
                        display: 'flex',
                        flexDirection: 'row',
                        alignItems: 'center',
                        gap: '18px',
                    }}
                >
                    {params.value || 0}
                </Box>
            );
        },
    },
];

const NodeVDisks = ({ vdisks }: { vdisks: VDisk[] }) => {
    const data = vdisks
        ? vdisks
              .map((vdisk, i) => {
                  return {
                      id: i,
                      vdiskid: vdisk.id,
                      status: vdisk.status,
                      partitionNumber: vdisk.partition_count,
                  } as NodeVDiskCol;
              })
              .sort((a, b) => (a.vdiskid < b.vdiskid ? 1 : -1))
        : [];
    return (
        <Box
            sx={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                backgroundColor: '#212329',
                gap: '15px',
                borderRadius: '8px',
                padding: '16px 28px 25px 28px',
                height: '678px',
                width: '40%',
            }}
        >
            <span>Node`s VDisk List</span>

            <div style={{ width: '100%', height: '100%' }}>
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
            </div>
        </Box>
    );
};

export default NodeVDisks;
