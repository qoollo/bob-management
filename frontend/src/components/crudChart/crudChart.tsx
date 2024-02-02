import {
    CategoryScale,
    Chart as ChartJS,
    Legend,
    LinearScale,
    LineElement,
    PointElement,
    Title,
    Tooltip,
} from 'chart.js';
import React from 'react';
import { Line } from 'react-chartjs-2';

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend);

const options = {
    maintainAspectRatio: false,
    responsive: true,

    plugins: {
        legend: {
            position: 'top' as const,
            align: 'end' as 'end' | 'start' | 'center' | undefined,
            labels: {
                color: '#efefef',
                font: {
                    size: 14,
                },
                usePointStyle: true,
            },
        },
        title: {
            display: true,
            text: 'RPS breakdown',
            align: 'start' as 'end' | 'start' | 'center' | undefined,
            color: '#efefef',
            font: {
                size: 16,
            },
        },

        scales: {
            yAxes: [
                {
                    ticks: {
                        beginAtZero: true,
                    },
                    gridLines: {
                        color: '#ffffff',
                    },
                },
            ],
            xAxes: [
                {
                    type: 'time',
                    gridLines: {
                        color: '#ffffff',
                    },
                },
            ],
        },
    },
};

const CrudChart = ({
    time,
    get,
    put,
    exist,
    del,
}: {
    time: string[];
    get: number[];
    put: number[];
    exist: number[];
    del: number[];
}) => {
    const data = {
        labels: time,
        datasets: [
            {
                label: 'Get',
                data: get,
                borderColor: '#EC7146',
                backgroundColor: '#EC7146',
            },
            {
                label: 'Put',
                data: put,
                borderColor: '#A12F45',
                backgroundColor: '#A12F45',
            },
            {
                label: 'Exist',
                data: exist,
                borderColor: '#7C817E',
                backgroundColor: '#7C817E',
            },
            {
                label: 'Delete',
                data: del,
                borderColor: '#5EB46B',
                backgroundColor: '#5EB46B',
            },
        ],
    };
    return <Line options={options} data={data} style={{ backgroundColor: '#212328' }} />;
};

export default CrudChart;
