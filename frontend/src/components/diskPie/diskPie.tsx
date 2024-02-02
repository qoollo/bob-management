import { ArcElement, Chart as ChartJS, Legend, Tooltip } from 'chart.js';
import React from 'react';
import { Doughnut } from 'react-chartjs-2';

ChartJS.register(ArcElement, Tooltip, Legend);

const config = {
    maintainAspectRatio: false,
    responsive: true,
    cutout: 170,
    plugins: {
        title: {
            display: true,
            text: 'Total occupied space on disks',
            align: 'center' as 'end' | 'start' | 'center' | undefined,
            color: '#efefef',
            font: {
                size: 16,
            },
        },
    },
};

function formatBytes(bytes: number, decimals = 0) {
    if (!+bytes) return '0 Bytes';

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))}${sizes[i]}`;
}

const DiskPie = ({ spaceInfo: { used_disk: usedSpace, total_disk: totalSpace } }: { spaceInfo: SpaceInfo }) => {
    const textCenter = {
        id: 'textCenter',
        afterDraw(chart: ChartJS<'doughnut', number[], unknown>) {
            const { ctx } = chart;
            ctx.save();
            ctx.font = 'bolder 50px  sans-serif';
            ctx.fillStyle = 'rgb(255, 104, 54)';
            ctx.textAlign = 'center';
            const numb = usedSpace / totalSpace;
            ctx.fillText(
                (+numb.toFixed(3) * 100).toFixed(0) + '%',
                chart.getDatasetMeta(0).data[0].x,
                chart.getDatasetMeta(0).data[0].y - 20,
            );
            ctx.font = 'bolder 30px sans-serif';
            ctx.fillStyle = 'white';
            ctx.fillText(
                formatBytes(usedSpace) + '/' + formatBytes(totalSpace),
                chart.getDatasetMeta(0).data[0].x,
                chart.getDatasetMeta(0).data[0].y + 25,
            );
        },
    };

    const f = (cur: number, total: number): number[] => {
        return [cur, total - cur];
    };

    const data = {
        datasets: [
            {
                label: 'Occupied space',
                data: f(usedSpace, totalSpace),
                backgroundColor: ['#FF6936', '#282A2F'],
                borderColor: ['#FF6936', '#282A2F'],
                borderWidth: 20,
            },
        ],
    };

    return <Doughnut data={data} options={config} plugins={[textCenter]} />;
};

export default DiskPie;
