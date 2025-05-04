/**
 * Charts Module for Bitcoin Analytics Dashboard
 * Creates and manages all chart visualizations
 */

class BitcoinAnalyticsCharts {
    constructor() {
        // Default chart colors
        this.colors = {
            primary: '#f7931a', // Bitcoin orange
            secondary: '#3490dc',
            tertiary: '#38c172',
            neutral: '#6c757d',
            danger: '#e3342f',
            background: '#121212',
            gridLines: '#262626',
            text: '#f8f9fa'
        };
        
        // Default options for all charts
        this.defaultOptions = {
            chart: {
                fontFamily: 'Inter, sans-serif',
                background: 'transparent',
                toolbar: {
                    show: false
                },
                zoom: {
                    enabled: false
                }
            },
            grid: {
                borderColor: this.colors.gridLines,
                strokeDashArray: 4,
                xaxis: {
                    lines: {
                        show: true
                    }
                }
            },
            tooltip: {
                theme: 'dark'
            },
            theme: {
                mode: 'dark'
            },
            xaxis: {
                labels: {
                    style: {
                        colors: this.colors.text
                    }
                },
                axisBorder: {
                    color: this.colors.gridLines
                },
                axisTicks: {
                    color: this.colors.gridLines
                }
            },
            yaxis: {
                labels: {
                    style: {
                        colors: this.colors.text
                    },
                    formatter: function (value) {
                        return value >= 1000000 ? (value / 1000000).toFixed(1) + 'M' :
                               value >= 1000 ? (value / 1000).toFixed(1) + 'K' :
                               value;
                    }
                }
            }
        };
        
        // Initialize chart instances
        this.charts = {};
    }

    /**
     * Initialize all charts on the dashboard
     */
    async initializeCharts() {
        // Fetch and initialize each chart
        try {
            await Promise.all([
                this.initBlockSizeChart(),
                this.initTxFeeChart(),
                this.initTxVolumeChart(),
                this.initAddressActivityChart(),
                this.initBalanceDistributionChart(),
                this.initMempoolSizeChart(),
                this.initFeeRatesChart()
            ]);
            console.log('All charts initialized successfully');
        } catch (error) {
            console.error('Error initializing charts:', error);
        }
    }

    /**
     * Update all charts with new data
     */
    async updateCharts() {
        try {
            await Promise.all([
                this.updateBlockSizeChart(),
                this.updateTxFeeChart(),
                this.updateTxVolumeChart(),
                this.updateAddressActivityChart(),
                this.updateBalanceDistributionChart(),
                this.updateMempoolSizeChart(),
                this.updateFeeRatesChart()
            ]);
            console.log('All charts updated successfully');
        } catch (error) {
            console.error('Error updating charts:', error);
        }
    }

    /**
     * Initialize block size chart
     */
    async initBlockSizeChart() {
        const chartData = await api.getBlockSizeHistory();
        
        const options = {
            ...this.defaultOptions,
            series: [{
                name: 'Block Size',
                data: chartData.data
            }],
            chart: {
                ...this.defaultOptions.chart,
                height: 300,
                type: 'area'
            },
            dataLabels: {
                enabled: false
            },
            stroke: {
                curve: 'smooth',
                width: 2
            },
            colors: [this.colors.primary],
            fill: {
                type: 'gradient',
                gradient: {
                    shadeIntensity: 1,
                    opacityFrom: 0.7,
                    opacityTo: 0.2,
                    stops: [0, 90, 100]
                }
            },
            title: {
                text: 'Block Size (Last 7 Days)',
                align: 'left',
                style: {
                    color: this.colors.text
                }
            },
            subtitle: {
                text: 'Average size in bytes',
                align: 'left',
                style: {
                    color: this.colors.neutral
                }
            },
            xaxis: {
                ...this.defaultOptions.xaxis,
                categories: chartData.labels
            },
            yaxis: {
                ...this.defaultOptions.yaxis,
                title: {
                    text: 'Size (bytes)',
                    style: {
                        color: this.colors.neutral
                    }
                }
            },
            tooltip: {
                ...this.defaultOptions.tooltip,
                y: {
                    formatter: function(value) {
                        return value >= 1000000 ? (value / 1000000).toFixed(2) + ' MB' :
                               value >= 1000 ? (value / 1000).toFixed(2) + ' KB' :
                               value + ' bytes';
                    }
                }
            }
        };
        
        this.charts.blockSize = new ApexCharts(document.querySelector('#block-size-chart'), options);
        this.charts.blockSize.render();
    }

    /**
     * Update block size chart with new data
     */
    async updateBlockSizeChart() {
        const chartData = await api.getBlockSizeHistory();
        
        this.charts.blockSize.updateOptions({
            xaxis: {
                categories: chartData.labels
            },
            series: [{
                name: 'Block Size',
                data: chartData.data
            }]
        });
    }

    /**
     * Initialize transaction fee chart
     */
    async initTxFeeChart() {
        const chartData = await api.getTransactionFeeHistory();
        
        const options = {
            ...this.defaultOptions,
            series: [{
                name: 'Average Fee Rate',
                data: chartData.data
            }],
            chart: {
                ...this.defaultOptions.chart,
                height: 300,
                type: 'line'
            },
            dataLabels: {
                enabled: false
            },
            stroke: {
                curve: 'smooth',
                width: 3
            },
            colors: [this.colors.secondary],
            title: {
                text: 'Transaction Fees (Last 7 Days)',
                align: 'left',
                style: {
                    color: this.colors.text
                }
            },
            subtitle: {
                text: 'Average fee rate in sat/vB',
                align: 'left',
                style: {
                    color: this.colors.neutral
                }
            },
            markers: {
                size: 4,
                strokeWidth: 0,
                hover: {
                    size: 6
                }
            },
            xaxis: {
                ...this.defaultOptions.xaxis,
                categories: chartData.labels
            },
            yaxis: {
                ...this.defaultOptions.yaxis,
                title: {
                    text: 'Fee Rate (sat/vB)',
                    style: {
                        color: this.colors.neutral
                    }
                },
                min: 0
            },
            tooltip: {
                ...this.defaultOptions.tooltip,
                y: {
                    formatter: function(value) {
                        return value.toFixed(2) + ' sat/vB';
                    }
                }
            }
        };
        
        this.charts.txFee = new ApexCharts(document.querySelector('#tx-fee-chart'), options);
        this.charts.txFee.render();
    }

    /**
     * Update transaction fee chart with new data
     */
    async updateTxFeeChart() {
        const chartData = await api.getTransactionFeeHistory();
        
        this.charts.txFee.updateOptions({
            xaxis: {
                categories: chartData.labels
            },
            series: [{
                name: 'Average Fee Rate',
                data: chartData.data
            }]
        });
    }

    /**
     * Initialize transaction volume chart
     */
    async initTxVolumeChart() {
        const chartData = await api.getTransactionVolumeHistory();
        
        const options = {
            ...this.defaultOptions,
            series: [{
                name: 'Transaction Count',
                data: chartData.data
            }],
            chart: {
                ...this.defaultOptions.chart,
                height: 300,
                type: 'bar'
            },
            plotOptions: {
                bar: {
                    borderRadius: 4,
                    columnWidth: '70%'
                }
            },
            dataLabels: {
                enabled: false
            },
            colors: [this.colors.tertiary],
            title: {
                text: 'Transaction Volume (Last 7 Days)',
                align: 'left',
                style: {
                    color: this.colors.text
                }
            },
            subtitle: {
                text: 'Number of transactions per day',
                align: 'left',
                style: {
                    color: this.colors.neutral
                }
            },
            xaxis: {
                ...this.defaultOptions.xaxis,
                categories: chartData.labels
            },
            yaxis: {
                ...this.defaultOptions.yaxis,
                title: {
                    text: 'Transaction Count',
                    style: {
                        color: this.colors.neutral
                    }
                },
                min: 0
            },
            tooltip: {
                ...this.defaultOptions.tooltip,
                y: {
                    formatter: function(value) {
                        return value.toLocaleString();
                    }
                }
            }
        };
        
        this.charts.txVolume = new ApexCharts(document.querySelector('#tx-volume-chart'), options);
        this.charts.txVolume.render();
    }

    /**
     * Update transaction volume chart with new data
     */
    async updateTxVolumeChart() {
        const chartData = await api.getTransactionVolumeHistory();
        
        this.charts.txVolume.updateOptions({
            xaxis: {
                categories: chartData.labels
            },
            series: [{
                name: 'Transaction Count',
                data: chartData.data
            }]
        });
    }

    /**
     * Initialize address activity chart
     */
    async initAddressActivityChart() {
        const chartData = await api.getAddressActivityHistory();
        
        const options = {
            ...this.defaultOptions,
            series: [{
                name: 'Active Addresses',
                data: chartData.data
            }],
            chart: {
                ...this.defaultOptions.chart,
                height: 300,
                type: 'area'
            },
            dataLabels: {
                enabled: false
            },
            stroke: {
                curve: 'smooth',
                width: 2
            },
            colors: [this.colors.secondary],
            fill: {
                type: 'gradient',
                gradient: {
                    shadeIntensity: 1,
                    opacityFrom: 0.7,
                    opacityTo: 0.2,
                    stops: [0, 90, 100]
                }
            },
            title: {
                text: 'Address Activity (Last 7 Days)',
                align: 'left',
                style: {
                    color: this.colors.text
                }
            },
            subtitle: {
                text: 'Number of active addresses per day',
                align: 'left',
                style: {
                    color: this.colors.neutral
                }
            },
            xaxis: {
                ...this.defaultOptions.xaxis,
                categories: chartData.labels
            },
            yaxis: {
                ...this.defaultOptions.yaxis,
                title: {
                    text: 'Address Count',
                    style: {
                        color: this.colors.neutral
                    }
                },
                min: 0
            },
            tooltip: {
                ...this.defaultOptions.tooltip,
                y: {
                    formatter: function(value) {
                        return value.toLocaleString();
                    }
                }
            }
        };
        
        this.charts.addressActivity = new ApexCharts(document.querySelector('#address-activity-chart'), options);
        this.charts.addressActivity.render();
    }

    /**
     * Update address activity chart with new data
     */
    async updateAddressActivityChart() {
        const chartData = await api.getAddressActivityHistory();
        
        this.charts.addressActivity.updateOptions({
            xaxis: {
                categories: chartData.labels
            },
            series: [{
                name: 'Active Addresses',
                data: chartData.data
            }]
        });
    }

    /**
     * Initialize balance distribution chart
     */
    async initBalanceDistributionChart() {
        const chartData = await api.getBalanceDistribution();
        
        const options = {
            ...this.defaultOptions,
            series: chartData.data,
            chart: {
                ...this.defaultOptions.chart,
                height: 300,
                type: 'pie'
            },
            labels: chartData.labels,
            colors: [
                '#f7931a', // Bitcoin orange
                '#e67e22',
                '#d35400',
                '#e74c3c',
                '#c0392b',
                '#9b59b6',
                '#8e44ad'
            ],
            title: {
                text: 'Balance Distribution',
                align: 'left',
                style: {
                    color: this.colors.text
                }
            },
            subtitle: {
                text: 'Percentage of addresses by balance',
                align: 'left',
                style: {
                    color: this.colors.neutral
                }
            },
            legend: {
                position: 'bottom',
                horizontalAlign: 'center',
                labels: {
                    colors: this.colors.text
                }
            },
            dataLabels: {
                enabled: true,
                formatter: function(val) {
                    return val.toFixed(1) + '%';
                },
                style: {
                    colors: ['#fff']
                },
                dropShadow: {
                    enabled: false
                }
            },
            responsive: [{
                breakpoint: 480,
                options: {
                    chart: {
                        height: 300
                    },
                    legend: {
                        position: 'bottom'
                    }
                }
            }]
        };
        
        this.charts.balanceDistribution = new ApexCharts(document.querySelector('#balance-distribution-chart'), options);
        this.charts.balanceDistribution.render();
    }

    /**
     * Update balance distribution chart with new data
     */
    async updateBalanceDistributionChart() {
        const chartData = await api.getBalanceDistribution();
        
        this.charts.balanceDistribution.updateOptions({
            labels: chartData.labels,
            series: chartData.data
        });
    }

    /**
     * Initialize mempool size chart
     */
    async initMempoolSizeChart() {
        const chartData = await api.getMempoolSizeHistory();
        
        const options = {
            ...this.defaultOptions,
            series: [{
                name: 'Mempool Size',
                data: chartData.data
            }],
            chart: {
                ...this.defaultOptions.chart,
                height: 300,
                type: 'area'
            },
            dataLabels: {
                enabled: false
            },
            stroke: {
                curve: 'smooth',
                width: 2
            },
            colors: [this.colors.danger],
            fill: {
                type: 'gradient',
                gradient: {
                    shadeIntensity: 1,
                    opacityFrom: 0.7,
                    opacityTo: 0.2,
                    stops: [0, 90, 100]
                }
            },
            title: {
                text: 'Mempool Size (Last 24 Hours)',
                align: 'left',
                style: {
                    color: this.colors.text
                }
            },
            subtitle: {
                text: 'Size in bytes',
                align: 'left',
                style: {
                    color: this.colors.neutral
                }
            },
            xaxis: {
                ...this.defaultOptions.xaxis,
                categories: chartData.labels
            },
            yaxis: {
                ...this.defaultOptions.yaxis,
                title: {
                    text: 'Size (bytes)',
                    style: {
                        color: this.colors.neutral
                    }
                }
            },
            tooltip: {
                ...this.defaultOptions.tooltip,
                y: {
                    formatter: function(value) {
                        return value >= 1000000 ? (value / 1000000).toFixed(2) + ' MB' :
                               value >= 1000 ? (value / 1000).toFixed(2) + ' KB' :
                               value + ' bytes';
                    }
                }
            }
        };
        
        this.charts.mempoolSize = new ApexCharts(document.querySelector('#mempool-size-chart'), options);
        this.charts.mempoolSize.render();
    }

    /**
     * Update mempool size chart with new data
     */
    async updateMempoolSizeChart() {
        const chartData = await api.getMempoolSizeHistory();
        
        this.charts.mempoolSize.updateOptions({
            xaxis: {
                categories: chartData.labels
            },
            series: [{
                name: 'Mempool Size',
                data: chartData.data
            }]
        });
    }

    /**
     * Initialize fee rates chart
     */
    async initFeeRatesChart() {
        const chartData = await api.getFeeRatesHistory();
        
        const options = {
            ...this.defaultOptions,
            series: [
                {
                    name: 'High Priority',
                    data: chartData.high
                },
                {
                    name: 'Medium Priority',
                    data: chartData.medium
                },
                {
                    name: 'Low Priority',
                    data: chartData.low
                }
            ],
            chart: {
                ...this.defaultOptions.chart,
                height: 300,
                type: 'line',
                zoom: {
                    enabled: false
                }
            },
            dataLabels: {
                enabled: false
            },
            stroke: {
                width: [4, 3, 2],
                curve: 'smooth',
                dashArray: [0, 0, 0]
            },
            colors: [this.colors.danger, this.colors.primary, this.colors.tertiary],
            title: {
                text: 'Fee Rates (Last 24 Hours)',
                align: 'left',
                style: {
                    color: this.colors.text
                }
            },
            subtitle: {
                text: 'Fee rates by priority in sat/vB',
                align: 'left',
                style: {
                    color: this.colors.neutral
                }
            },
            legend: {
                tooltipHoverFormatter: function(val, opts) {
                    return val + ' - ' + opts.w.globals.series[opts.seriesIndex][opts.dataPointIndex] + ' sat/vB';
                },
                position: 'top',
                horizontalAlign: 'right',
                labels: {
                    colors: this.colors.text
                }
            },
            markers: {
                size: 0,
                hover: {
                    size: 5
                }
            },
            xaxis: {
                ...this.defaultOptions.xaxis,
                categories: chartData.labels
            },
            yaxis: {
                ...this.defaultOptions.yaxis,
                title: {
                    text: 'Fee Rate (sat/vB)',
                    style: {
                        color: this.colors.neutral
                    }
                },
                min: 0
            },
            tooltip: {
                ...this.defaultOptions.tooltip,
                y: {
                    formatter: function(value) {
                        return value.toFixed(2) + ' sat/vB';
                    }
                }
            }
        };
        
        this.charts.feeRates = new ApexCharts(document.querySelector('#fee-rates-chart'), options);
        this.charts.feeRates.render();
    }

    /**
     * Update fee rates chart with new data
     */
    async updateFeeRatesChart() {
        const chartData = await api.getFeeRatesHistory();
        
        this.charts.feeRates.updateOptions({
            xaxis: {
                categories: chartData.labels
            },
            series: [
                {
                    name: 'High Priority',
                    data: chartData.high
                },
                {
                    name: 'Medium Priority',
                    data: chartData.medium
                },
                {
                    name: 'Low Priority',
                    data: chartData.low
                }
            ]
        });
    }
}

// Create charts instance
const charts = new BitcoinAnalyticsCharts();
