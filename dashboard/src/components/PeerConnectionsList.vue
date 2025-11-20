<template>
  <div class="peer-connections">
    <div class="header">
      <h2>Peer Connections</h2>
      <button @click="refetch" :disabled="loading" class="refresh-btn">
        {{ loading ? 'Refreshing...' : 'Refresh' }}
      </button>
    </div>

    <!-- Connection Statistics -->
    <div v-if="stats" class="stats-grid">
      <div class="stat-card">
        <div class="stat-label">Total Connections</div>
        <div class="stat-value">{{ stats.totalConnections }}</div>
      </div>
      <div class="stat-card active">
        <div class="stat-label">Active Connections</div>
        <div class="stat-value">{{ stats.activeConnections }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">Total Bytes In</div>
        <div class="stat-value">{{ formatBytes(stats.totalBytesInbound) }}</div>
      </div>
      <div class="stat-card">
        <div class="stat-label">Total Bytes Out</div>
        <div class="stat-value">{{ formatBytes(stats.totalBytesOutbound) }}</div>
      </div>
    </div>

    <!-- Error Message -->
    <div v-if="error" class="error">
      ⚠️ {{ error }}
    </div>

    <!-- Loading State -->
    <div v-if="loading && connections.length === 0" class="loading">
      Loading peer connections...
    </div>

    <!-- Empty State -->
    <div v-else-if="!loading && connections.length === 0" class="empty-state">
      <p>No active peer connections found.</p>
      <p class="hint">Connections will appear here when peers connect to your node.</p>
    </div>

    <!-- Connections Table -->
    <div v-else class="table-container">
      <table class="connections-table">
        <thead>
          <tr>
            <th>Connection ID</th>
            <th>Client Address</th>
            <th>Target Address</th>
            <th>Connected At</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="connection in connections" :key="connection.id" class="connection-row">
            <td class="mono">{{ connection.connectionId }}</td>
            <td class="mono">{{ connection.clientAddr }}</td>
            <td class="mono">{{ connection.targetAddr }}</td>
            <td>{{ formatDate(connection.connectedAt) }}</td>
            <td>
              <span class="status-badge active">
                Active
              </span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="connections.length > 0" class="footer">
      Showing {{ connections.length }} active connection{{ connections.length !== 1 ? 's' : '' }}
      • Auto-refreshes every 5 seconds
    </div>
  </div>
</template>

<script setup lang="ts">
import { usePeerConnections } from '../composables/usePeerConnections'

const { connections, stats, loading, error, refetch } = usePeerConnections(true, 5000)

const formatBytes = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`
}

const formatDate = (dateString: string): string => {
  const date = new Date(dateString)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  
  if (diffMins < 1) return 'Just now'
  if (diffMins < 60) return `${diffMins}m ago`
  if (diffMins < 1440) return `${Math.floor(diffMins / 60)}h ago`
  
  return date.toLocaleString()
}
</script>

<style scoped>
.peer-connections {
  max-width: 1400px;
  margin: 0 auto;
  padding: 2rem;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.header h2 {
  margin: 0;
  font-size: 2rem;
  color: var(--color-heading);
}

.refresh-btn {
  padding: 0.5rem 1rem;
  background: var(--color-background-mute);
  border: 1px solid var(--color-border);
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.2s;
}

.refresh-btn:hover:not(:disabled) {
  background: var(--color-background-soft);
  border-color: var(--color-border-hover);
}

.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.stat-card {
  background: var(--color-background-soft);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  padding: 1.5rem;
  text-align: center;
}

.stat-card.active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
}

.stat-label {
  font-size: 0.85rem;
  opacity: 0.8;
  margin-bottom: 0.5rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.stat-value {
  font-size: 2rem;
  font-weight: bold;
}

.error {
  background: #fee;
  border: 1px solid #fcc;
  color: #c33;
  padding: 1rem;
  border-radius: 6px;
  margin-bottom: 1rem;
}

.loading {
  text-align: center;
  padding: 3rem;
  color: var(--color-text-mute);
  font-size: 1.1rem;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  background: var(--color-background-soft);
  border: 2px dashed var(--color-border);
  border-radius: 8px;
}

.empty-state p {
  margin: 0.5rem 0;
  color: var(--color-text);
}

.empty-state .hint {
  font-size: 0.9rem;
  color: var(--color-text-mute);
}

.table-container {
  overflow-x: auto;
  background: var(--color-background-soft);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  margin-bottom: 1rem;
}

.connections-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9rem;
}

.connections-table thead {
  background: var(--color-background-mute);
  border-bottom: 2px solid var(--color-border);
}

.connections-table th {
  padding: 1rem;
  text-align: left;
  font-weight: 600;
  color: var(--color-heading);
  white-space: nowrap;
}

.connections-table tbody tr {
  border-bottom: 1px solid var(--color-border);
  transition: background 0.2s;
}

.connections-table tbody tr:hover {
  background: var(--color-background-mute);
}

.connections-table tbody tr:last-child {
  border-bottom: none;
}

.connections-table td {
  padding: 1rem;
}

.mono {
  font-family: 'Courier New', monospace;
  font-size: 0.85rem;
}

.status-badge {
  display: inline-block;
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.status-badge.active {
  background: #d4edda;
  color: #155724;
}

.footer {
  text-align: center;
  padding: 1rem;
  color: var(--color-text-mute);
  font-size: 0.85rem;
}

@media (max-width: 768px) {
  .peer-connections {
    padding: 1rem;
  }

  .header h2 {
    font-size: 1.5rem;
  }

  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  .table-container {
    font-size: 0.8rem;
  }

  .connections-table th,
  .connections-table td {
    padding: 0.75rem 0.5rem;
  }
}
</style>

