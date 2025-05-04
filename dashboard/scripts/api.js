/**
 * API Module for Bitcoin Analytics Dashboard
 * Handles all API requests to the ClickHouse backend
 */

class BitcoinAnalyticsAPI {
    constructor() {
        // Base URL for API endpoints
        this.baseURL = '/api';
        
        // For demo purposes, we'll use mock data if API is not available
        this.useMockData = true; // Set to false when real API is available
    }

    /**
     * Generic method to fetch data from an endpoint
     */
    async fetchData(endpoint, params = {}) {
        if (this.useMockData) {
            return this.getMockData(endpoint);
        }

        try {
            const queryString = new URLSearchParams(params).toString();
            const url = `${this.baseURL}/${endpoint}${queryString ? `?${queryString}` : ''}`;
            
            const response = await fetch(url);
            
            if (!response.ok) {
                throw new Error(`API request failed: ${response.status}`);
            }
            
            return await response.json();
        } catch (error) {
            console.error(`Error fetching ${endpoint}:`, error);
            return this.getMockData(endpoint);
        }
    }

    /**
     * Get recent blocks
     */
    async getRecentBlocks(limit = 10) {
        return this.fetchData('blocks', { limit });
    }

    /**
     * Get block details by block number
     */
    async getBlockByNumber(blockNumber) {
        return this.fetchData(`blocks/${blockNumber}`);
    }

    /**
     * Get recent transactions
     */
    async getRecentTransactions(limit = 10) {
        return this.fetchData('transactions', { limit });
    }

    /**
     * Get transaction details by hash
     */
    async getTransactionByHash(txHash) {
        return this.fetchData(`transactions/${txHash}`);
    }

    /**
     * Get address information
     */
    async getAddressInfo(address) {
        return this.fetchData(`addresses/${address}`);
    }

    /**
     * Get mempool statistics
     */
    async getMempoolStats() {
        return this.fetchData('mempool');
    }

    /**
     * Get block size history (for charts)
     */
    async getBlockSizeHistory(days = 7) {
        return this.fetchData('charts/block-size', { days });
    }

    /**
     * Get transaction fee history (for charts)
     */
    async getTransactionFeeHistory(days = 7) {
        return this.fetchData('charts/tx-fees', { days });
    }

    /**
     * Get transaction volume history (for charts)
     */
    async getTransactionVolumeHistory(days = 7) {
        return this.fetchData('charts/tx-volume', { days });
    }

    /**
     * Get address activity history (for charts)
     */
    async getAddressActivityHistory(days = 7) {
        return this.fetchData('charts/address-activity', { days });
    }

    /**
     * Get balance distribution (for charts)
     */
    async getBalanceDistribution() {
        return this.fetchData('charts/balance-distribution');
    }

    /**
     * Get mempool size history (for charts)
     */
    async getMempoolSizeHistory(hours = 24) {
        return this.fetchData('charts/mempool-size', { hours });
    }

    /**
     * Get fee rates history (for charts)
     */
    async getFeeRatesHistory(hours = 24) {
        return this.fetchData('charts/fee-rates', { hours });
    }

    /**
     * Get protocol feature adoption (for charts)
     */
    async getProtocolFeatureAdoption(days = 30) {
        return this.fetchData('charts/protocol-features', { days });
    }

    /**
     * Get summary statistics
     */
    async getSummaryStats() {
        return this.fetchData('summary');
    }

    /**
     * Mock data for development and demonstration
     */
    getMockData(endpoint) {
        // Current timestamp for mock data
        const now = Date.now();
        const day = 24 * 60 * 60 * 1000;
        const hour = 60 * 60 * 1000;
        
        // Mock data based on endpoint
        const mockData = {
            'blocks': {
                data: Array(10).fill().map((_, i) => ({
                    block_number: 800000 - i,
                    block_hash: `000000000000000000${Math.random().toString(16).substring(2, 10)}`,
                    timestamp: new Date(now - (i * 10 * 60 * 1000)).toISOString(),
                    size: Math.floor(Math.random() * 1000000) + 500000,
                    weight: Math.floor(Math.random() * 4000000) + 2000000,
                    transaction_count: Math.floor(Math.random() * 2000) + 1000,
                    miner: ['Antpool', 'F2Pool', 'Foundry USA', 'Binance Pool', 'ViaBTC'][Math.floor(Math.random() * 5)],
                    version: 536870912,
                    difficulty: '49.11 T',
                    protocol_features: JSON.stringify([
                        { name: 'SegWit', count: Math.floor(Math.random() * 1500) + 500, percentage: Math.random() * 100 },
                        { name: 'Taproot', count: Math.floor(Math.random() * 500) + 100, percentage: Math.random() * 30 }
                    ])
                }))
            },
            'transactions': {
                data: Array(10).fill().map((_, i) => ({
                    transaction_hash: `${Math.random().toString(16).substring(2, 34)}`,
                    block_number: 800000 - Math.floor(i / 3),
                    timestamp: new Date(now - (i * 2 * 60 * 1000)).toISOString(),
                    size: Math.floor(Math.random() * 1000) + 200,
                    weight: Math.floor(Math.random() * 4000) + 800,
                    virtual_size: Math.floor(Math.random() * 1000) + 200,
                    fee: Math.floor(Math.random() * 50000) + 1000,
                    fee_rate: Math.random() * 100 + 5,
                    input_count: Math.floor(Math.random() * 10) + 1,
                    output_count: Math.floor(Math.random() * 5) + 1,
                    total_input_value: Math.floor(Math.random() * 10000000000),
                    total_output_value: Math.floor(Math.random() * 10000000000),
                    is_coinbase: i === 0 ? 1 : 0,
                    transaction_type: Math.floor(Math.random() * 3) + 1
                }))
            },
            'mempool': {
                timestamp: new Date(now).toISOString(),
                transaction_count: Math.floor(Math.random() * 5000) + 10000,
                total_fee: Math.floor(Math.random() * 100000000) + 50000000,
                median_fee_rate: Math.random() * 50 + 10,
                high_priority_fee_rate: Math.random() * 100 + 50,
                medium_priority_fee_rate: Math.random() * 50 + 10,
                low_priority_fee_rate: Math.random() * 10 + 1,
                total_mempool_size: Math.floor(Math.random() * 100000000) + 50000000
            },
            'addresses/bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh': {
                address: 'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh',
                balance: Math.floor(Math.random() * 10000000000),
                transaction_count: Math.floor(Math.random() * 100) + 10,
                first_seen: new Date(now - (Math.random() * 365 * day)).toISOString(),
                utxo_count: Math.floor(Math.random() * 20) + 1,
                recent_activity: Array(5).fill().map((_, i) => ({
                    transaction_hash: `${Math.random().toString(16).substring(2, 34)}`,
                    timestamp: new Date(now - (i * day)).toISOString(),
                    value: Math.floor(Math.random() * 100000000),
                    type: Math.random() > 0.5 ? 'send' : 'receive'
                }))
            },
            'summary': {
                latest_block: 800000,
                transaction_count_24h: Math.floor(Math.random() * 300000) + 200000,
                average_fee: Math.random() * 50 + 10,
                segwit_adoption: Math.random() * 100 + 70
            },
            'charts/block-size': {
                labels: Array(7).fill().map((_, i) => new Date(now - ((6 - i) * day)).toISOString().split('T')[0]),
                data: Array(7).fill().map(() => Math.floor(Math.random() * 1000000) + 500000)
            },
            'charts/tx-fees': {
                labels: Array(7).fill().map((_, i) => new Date(now - ((6 - i) * day)).toISOString().split('T')[0]),
                data: Array(7).fill().map(() => Math.random() * 100 + 5)
            },
            'charts/tx-volume': {
                labels: Array(7).fill().map((_, i) => new Date(now - ((6 - i) * day)).toISOString().split('T')[0]),
                data: Array(7).fill().map(() => Math.floor(Math.random() * 300000) + 200000)
            },
            'charts/address-activity': {
                labels: Array(7).fill().map((_, i) => new Date(now - ((6 - i) * day)).toISOString().split('T')[0]),
                data: Array(7).fill().map(() => Math.floor(Math.random() * 1000000) + 500000)
            },
            'charts/balance-distribution': {
                labels: ['0-0.01 BTC', '0.01-0.1 BTC', '0.1-1 BTC', '1-10 BTC', '10-100 BTC', '100-1000 BTC', '1000+ BTC'],
                data: [65, 25, 7, 2, 0.7, 0.25, 0.05]
            },
            'charts/mempool-size': {
                labels: Array(24).fill().map((_, i) => new Date(now - ((23 - i) * hour)).toISOString().split('T')[1].split(':')[0] + ':00'),
                data: Array(24).fill().map(() => Math.floor(Math.random() * 50000000) + 10000000)
            },
            'charts/fee-rates': {
                labels: Array(24).fill().map((_, i) => new Date(now - ((23 - i) * hour)).toISOString().split('T')[1].split(':')[0] + ':00'),
                high: Array(24).fill().map(() => Math.random() * 100 + 50),
                medium: Array(24).fill().map(() => Math.random() * 50 + 10),
                low: Array(24).fill().map(() => Math.random() * 10 + 1)
            },
            'charts/protocol-features': {
                labels: ['SegWit', 'Taproot'],
                data: [Math.random() * 30 + 70, Math.random() * 20 + 10]
            }
        };
        
        // Return mock data for the specified endpoint or fallback
        return Promise.resolve(mockData[endpoint] || { error: 'No mock data available for this endpoint' });
    }
}

// Create and export API instance
const api = new BitcoinAnalyticsAPI();
