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
            text: 'Общий RPS кластера',
            align: 'start' as 'end' | 'start' | 'center' | undefined,
            color: '#efefef',
            font: {
                size: 16,
            },
        },

        scales: {
            yAxes: [
                {
                    min: 0,
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
                    ticks: {
                        source: 'labels',
                    },
                    gridLines: {
                        color: '#ffffff',
                    },
                },
            ],
        },
    },
};

const ClusterRpsChart = ({ timex, rpsy }: { timex: string[]; rpsy: number[] }) => {
    const data = {
        labels: timex,
        datasets: [
            {
                label: 'RPS',
                data: rpsy,
                borderColor: '#ED7246',
                backgroundColor: '#ED7246',
            },
        ],
    };

    return <Line options={options} data={data} style={{ backgroundColor: '#212328' }} />;
};

export default ClusterRpsChart;
