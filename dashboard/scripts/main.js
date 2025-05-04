/**
 * Main JavaScript for Bitcoin Analytics Dashboard
 * Initializes the dashboard and handles user interactions
 */

document.addEventListener('DOMContentLoaded', async () => {
    // Initialize the dashboard
    await initializeDashboard();
    
    // Set up automatic refresh every 5 minutes
    setInterval(async () => {
        await refreshDashboard();
    }, 5 * 60 * 1000);
    
    // Set up search functionality
    setupSearchFunctionality();
    
    // Set up event listeners
    setupEventListeners();
});

/**
 * Initialize the dashboard with data
 */
async function initializeDashboard() {
    try {
        // Show loading state
        showLoading();
        
        // Initialize the charts
        await charts.initializeCharts();
        
        // Load summary statistics
        await loadSummaryStats();
        
        // Load recent blocks and transactions
        await Promise.all([
            loadRecentBlocks(),
            loadRecentTransactions()
        ]);
        
        // Load mempool statistics
        await loadMempoolStats();
        
        // Hide loading state
        hideLoading();
        
        console.log('Dashboard initialized successfully');
    } catch (error) {
        console.error('Error initializing dashboard:', error);
        hideLoading();
        showError('Failed to initialize dashboard. Please try again later.');
    }
}

/**
 * Refresh dashboard data
 */
async function refreshDashboard() {
    try {
        console.log('Refreshing dashboard data...');
        
        // Update the charts
        await charts.updateCharts();
        
        // Update summary statistics
        await loadSummaryStats(false);
        
        // Update recent blocks and transactions
        await Promise.all([
            loadRecentBlocks(false),
            loadRecentTransactions(false)
        ]);
        
        // Update mempool statistics
        await loadMempoolStats(false);
        
        console.log('Dashboard refreshed successfully');
    } catch (error) {
        console.error('Error refreshing dashboard:', error);
    }
}

/**
 * Load summary statistics
 */
async function loadSummaryStats(showLoadingState = true) {
    if (showLoadingState) {
        document.getElementById('latest-block').textContent = 'Loading...';
        document.getElementById('tx-count').textContent = 'Loading...';
        document.getElementById('avg-fee').textContent = 'Loading...';
        document.getElementById('segwit-adoption').textContent = 'Loading...';
    }
    
    try {
        const summaryData = await api.getSummaryStats();
        
        // Update summary stats cards
        document.getElementById('latest-block').textContent = summaryData.latest_block.toLocaleString();
        document.getElementById('tx-count').textContent = summaryData.transaction_count_24h.toLocaleString();
        document.getElementById('avg-fee').textContent = summaryData.average_fee.toFixed(2);
        document.getElementById('segwit-adoption').textContent = summaryData.segwit_adoption.toFixed(1) + '%';
    } catch (error) {
        console.error('Error loading summary stats:', error);
        if (showLoadingState) {
            document.getElementById('latest-block').textContent = 'Error';
            document.getElementById('tx-count').textContent = 'Error';
            document.getElementById('avg-fee').textContent = 'Error';
            document.getElementById('segwit-adoption').textContent = 'Error';
        }
    }
}

/**
 * Load recent blocks
 */
async function loadRecentBlocks(showLoadingState = true) {
    const recentBlocksElement = document.getElementById('recent-blocks');
    
    if (showLoadingState) {
        recentBlocksElement.innerHTML = `
            <div class="placeholder-glow">
                <div class="placeholder col-12 mb-2"></div>
                <div class="placeholder col-12 mb-2"></div>
                <div class="placeholder col-12 mb-2"></div>
                <div class="placeholder col-12 mb-2"></div>
                <div class="placeholder col-12 mb-2"></div>
            </div>
        `;
    }
    
    try {
        const blocksData = await api.getRecentBlocks(5);
        
        if (!blocksData.data || blocksData.data.length === 0) {
            recentBlocksElement.innerHTML = '<p class="text-muted">No recent blocks found.</p>';
            return;
        }
        
        let html = '';
        
        blocksData.data.forEach(block => {
            const timestamp = new Date(block.timestamp);
            const timeAgo = getTimeAgo(timestamp);
            
            html += `
                <div class="block-item" data-block-number="${block.block_number}">
                    <div class="d-flex justify-content-between align-items-center">
                        <strong>${block.block_number.toLocaleString()}</strong>
                        <small class="text-muted">${timeAgo}</small>
                    </div>
                    <div class="block-hash">${truncateMiddle(block.block_hash, 20)}</div>
                    <div class="block-info">
                        <span>${block.transaction_count} txs</span>
                        <span>${formatBytes(block.size)}</span>
                        <span>${block.miner}</span>
                    </div>
                </div>
            `;
        });
        
        recentBlocksElement.innerHTML = html;
        
        // Add click event listeners to block items
        document.querySelectorAll('.block-item').forEach(element => {
            element.addEventListener('click', () => {
                const blockNumber = element.getAttribute('data-block-number');
                showBlockDetails(blockNumber);
            });
        });
    } catch (error) {
        console.error('Error loading recent blocks:', error);
        if (showLoadingState) {
            recentBlocksElement.innerHTML = '<p class="text-danger">Failed to load recent blocks. Please try again later.</p>';
        }
    }
}

/**
 * Load recent transactions
 */
async function loadRecentTransactions(showLoadingState = true) {
    const recentTransactionsElement = document.getElementById('recent-transactions');
    
    if (showLoadingState) {
        recentTransactionsElement.innerHTML = `
            <tr>
                <td colspan="5" class="text-center">
                    <div class="placeholder-glow">
                        <div class="placeholder col-12"></div>
                    </div>
                </td>
            </tr>
        `;
    }
    
    try {
        const transactionsData = await api.getRecentTransactions(5);
        
        if (!transactionsData.data || transactionsData.data.length === 0) {
            recentTransactionsElement.innerHTML = `
                <tr>
                    <td colspan="5" class="text-center">No recent transactions found.</td>
                </tr>
            `;
            return;
        }
        
        let html = '';
        
        transactionsData.data.forEach(tx => {
            const txType = getTransactionTypeLabel(tx.transaction_type);
            const txValue = tx.total_output_value / 100000000; // Convert satoshis to BTC
            
            html += `
                <tr class="transaction-row" data-tx-hash="${tx.transaction_hash}">
                    <td>
                        <div class="d-flex align-items-center">
                            ${tx.is_coinbase ? '<i class="bi bi-award-fill text-warning me-2" title="Coinbase Transaction"></i>' : ''}
                            ${truncateMiddle(tx.transaction_hash, 14)}
                        </div>
                    </td>
                    <td>${tx.virtual_size.toLocaleString()}</td>
                    <td>${tx.fee_rate.toFixed(2)}</td>
                    <td>${txValue.toFixed(8)} BTC</td>
                    <td><span class="badge bg-dark">${txType}</span></td>
                </tr>
            `;
        });
        
        recentTransactionsElement.innerHTML = html;
        
        // Add click event listeners to transaction rows
        document.querySelectorAll('.transaction-row').forEach(element => {
            element.addEventListener('click', () => {
                const txHash = element.getAttribute('data-tx-hash');
                showTransactionDetails(txHash);
            });
        });
    } catch (error) {
        console.error('Error loading recent transactions:', error);
        if (showLoadingState) {
            recentTransactionsElement.innerHTML = `
                <tr>
                    <td colspan="5" class="text-center text-danger">Failed to load recent transactions. Please try again later.</td>
                </tr>
            `;
        }
    }
}

/**
 * Load mempool statistics
 */
async function loadMempoolStats(showLoadingState = true) {
    if (showLoadingState) {
        document.getElementById('high-fee-rate').textContent = 'Loading...';
        document.getElementById('medium-fee-rate').textContent = 'Loading...';
        document.getElementById('low-fee-rate').textContent = 'Loading...';
    }
    
    try {
        const mempoolData = await api.getMempoolStats();
        
        // Update mempool stats cards
        document.getElementById('high-fee-rate').textContent = mempoolData.high_priority_fee_rate.toFixed(2);
        document.getElementById('medium-fee-rate').textContent = mempoolData.medium_priority_fee_rate.toFixed(2);
        document.getElementById('low-fee-rate').textContent = mempoolData.low_priority_fee_rate.toFixed(2);
    } catch (error) {
        console.error('Error loading mempool stats:', error);
        if (showLoadingState) {
            document.getElementById('high-fee-rate').textContent = 'Error';
            document.getElementById('medium-fee-rate').textContent = 'Error';
            document.getElementById('low-fee-rate').textContent = 'Error';
        }
    }
}

/**
 * Show block details
 */
async function showBlockDetails(blockNumber) {
    try {
        const blockData = await api.getBlockByNumber(blockNumber);
        // In a real implementation, show a modal with block details
        alert(`Block details for block ${blockNumber} would be shown here.`);
        console.log('Block details:', blockData);
    } catch (error) {
        console.error('Error fetching block details:', error);
        showError('Failed to load block details.');
    }
}

/**
 * Show transaction details
 */
async function showTransactionDetails(txHash) {
    try {
        const txData = await api.getTransactionByHash(txHash);
        // In a real implementation, show a modal with transaction details
        alert(`Transaction details for ${txHash} would be shown here.`);
        console.log('Transaction details:', txData);
    } catch (error) {
        console.error('Error fetching transaction details:', error);
        showError('Failed to load transaction details.');
    }
}

/**
 * Set up search functionality
 */
function setupSearchFunctionality() {
    const searchInput = document.getElementById('address-search');
    const searchButton = document.getElementById('address-search-btn');
    
    // Search on button click
    searchButton.addEventListener('click', () => {
        const address = searchInput.value.trim();
        if (address) {
            searchAddress(address);
        }
    });
    
    // Search on Enter key
    searchInput.addEventListener('keyup', event => {
        if (event.key === 'Enter') {
            const address = searchInput.value.trim();
            if (address) {
                searchAddress(address);
            }
        }
    });
}

/**
 * Search for an address
 */
async function searchAddress(address) {
    const addressDetails = document.getElementById('address-details');
    
    try {
        // Show loading state
        addressDetails.classList.remove('d-none');
        document.getElementById('detail-address').textContent = 'Loading...';
        document.getElementById('detail-balance').textContent = 'Loading...';
        document.getElementById('detail-transactions').textContent = 'Loading...';
        document.getElementById('detail-first-seen').textContent = 'Loading...';
        
        const addressData = await api.getAddressInfo(address);
        
        // Update address details
        document.getElementById('detail-address').textContent = addressData.address;
        document.getElementById('detail-balance').textContent = (addressData.balance / 100000000).toFixed(8) + ' BTC';
        document.getElementById('detail-transactions').textContent = addressData.transaction_count.toLocaleString();
        document.getElementById('detail-first-seen').textContent = new Date(addressData.first_seen).toLocaleString();
    } catch (error) {
        console.error('Error searching address:', error);
        addressDetails.classList.remove('d-none');
        document.getElementById('detail-address').textContent = address;
        document.getElementById('detail-balance').textContent = 'Error loading data';
        document.getElementById('detail-transactions').textContent = 'Error loading data';
        document.getElementById('detail-first-seen').textContent = 'Error loading data';
    }
}

/**
 * Set up event listeners
 */
function setupEventListeners() {
    // Toggle address details section
    document.getElementById('address-search-btn').addEventListener('click', () => {
        const address = document.getElementById('address-search').value.trim();
        if (address) {
            document.getElementById('address-details').classList.remove('d-none');
        }
    });
}

/**
 * Helper function to show loading state
 */
function showLoading() {
    // In a real implementation, show a loading spinner
    console.log('Loading dashboard data...');
}

/**
 * Helper function to hide loading state
 */
function hideLoading() {
    // In a real implementation, hide the loading spinner
    console.log('Dashboard data loaded');
}

/**
 * Helper function to show error message
 */
function showError(message) {
    // In a real implementation, show an error toast or alert
    console.error(message);
    alert(message);
}

/**
 * Helper function to format bytes
 */
function formatBytes(bytes, decimals = 2) {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

/**
 * Helper function to get time ago string
 */
function getTimeAgo(timestamp) {
    const now = new Date();
    const secondsAgo = Math.floor((now - timestamp) / 1000);
    
    if (secondsAgo < 60) {
        return `${secondsAgo} sec${secondsAgo !== 1 ? 's' : ''} ago`;
    }
    
    const minutesAgo = Math.floor(secondsAgo / 60);
    if (minutesAgo < 60) {
        return `${minutesAgo} min${minutesAgo !== 1 ? 's' : ''} ago`;
    }
    
    const hoursAgo = Math.floor(minutesAgo / 60);
    if (hoursAgo < 24) {
        return `${hoursAgo} hour${hoursAgo !== 1 ? 's' : ''} ago`;
    }
    
    const daysAgo = Math.floor(hoursAgo / 24);
    return `${daysAgo} day${daysAgo !== 1 ? 's' : ''} ago`;
}

/**
 * Helper function to truncate string in the middle
 */
function truncateMiddle(text, maxLength) {
    if (text.length <= maxLength) {
        return text;
    }
    
    const ellipsis = '...';
    const charsToShow = maxLength - ellipsis.length;
    const frontChars = Math.ceil(charsToShow / 2);
    const backChars = Math.floor(charsToShow / 2);
    
    return text.substring(0, frontChars) + ellipsis + text.substring(text.length - backChars);
}

/**
 * Helper function to get transaction type label
 */
function getTransactionTypeLabel(type) {
    const types = {
        0: 'Unknown',
        1: 'Standard',
        2: 'SegWit',
        3: 'Taproot',
        4: 'Multisig',
        5: 'Lightning'
    };
    
    return types[type] || 'Unknown';
}
