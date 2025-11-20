<template>
  <div class="messages-view">
    <div class="header">
      <h2>Recent Messages</h2>
      <button @click="refetch" :disabled="loading" class="refresh-btn">
        {{ loading ? 'Refreshing...' : 'Refresh' }}
      </button>
    </div>

    <!-- Error Message -->
    <div v-if="error" class="error">
      ⚠️ {{ error }}
    </div>

    <!-- Loading State -->
    <div v-if="loading && messages.length === 0" class="loading">
      Loading messages...
    </div>

    <!-- Empty State -->
    <div v-else-if="!loading && messages.length === 0" class="empty-state">
      <p>No messages recorded yet.</p>
      <p class="hint">Messages will appear here as peers communicate with your node.</p>
    </div>

    <!-- Messages Table -->
    <div v-else class="table-container">
      <table class="messages-table">
        <thead>
          <tr>
            <th>Time</th>
            <th>Direction</th>
            <th>Type</th>
            <th>From</th>
            <th>To</th>
            <th>Size</th>
            <th>Description</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="message in messages" :key="message.id" class="message-row">
            <td class="timestamp">{{ formatTimestamp(message.timestamp) }}</td>
            <td>
              <span :class="['direction-badge', message.direction]">
                {{ message.direction === 'inbound' ? '→' : '←' }}
                {{ message.direction }}
              </span>
            </td>
            <td>
              <span class="message-type" :class="getMessageTypeClass(message.messageType)">
                {{ message.messageType }}
              </span>
            </td>
            <td class="mono peer-addr">{{ formatPeerAddr(message.sourcePeer) }}</td>
            <td class="mono peer-addr">{{ formatPeerAddr(message.destinationPeer) }}</td>
            <td class="size">{{ formatBytes(message.payloadSize) }}</td>
            <td class="description">{{ message.description }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="messages.length > 0" class="footer">
      Showing {{ messages.length }} recent message{{ messages.length !== 1 ? 's' : '' }}
      • Auto-refreshes every 5 seconds
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRecentMessages } from '../composables/useMessages'

const { messages, loading, error, refetch } = useRecentMessages(200, true, 5000)

const formatTimestamp = (timestamp: string): string => {
  const date = new Date(timestamp)
  return date.toLocaleTimeString()
}

const formatPeerAddr = (addr: string): string => {
  // Shorten long addresses for display
  if (addr.length > 25) {
    return addr.substring(0, 22) + '...'
  }
  return addr
}

const formatBytes = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`
}

const getMessageTypeClass = (type: string): string => {
  // Color-code different message types
  const handshakeTypes = ['version', 'verack']
  const dataTypes = ['tx', 'block', 'inv', 'getdata']
  const addressTypes = ['addr', 'addrv2', 'getaddr']
  const syncTypes = ['headers', 'getheaders', 'getblocks']

  if (handshakeTypes.includes(type)) return 'type-handshake'
  if (dataTypes.includes(type)) return 'type-data'
  if (addressTypes.includes(type)) return 'type-address'
  if (syncTypes.includes(type)) return 'type-sync'
  return 'type-other'
}
</script>

<style scoped>
.messages-view {
  max-width: 1600px;
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
  border-radius: 12px;
  margin-bottom: 1rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.messages-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.85rem;
}

.messages-table thead {
  background: var(--color-background-mute);
  border-bottom: 2px solid var(--color-border);
}

.messages-table th {
  padding: 0.75rem;
  text-align: left;
  font-weight: 600;
  color: var(--color-heading);
  white-space: nowrap;
}

.messages-table tbody tr {
  border-bottom: 1px solid var(--color-border);
  transition: background 0.2s;
}

.messages-table tbody tr:hover {
  background: var(--color-background-mute);
}

.messages-table tbody tr:last-child {
  border-bottom: none;
}

.messages-table td {
  padding: 0.75rem;
}

.timestamp {
  color: var(--color-text-mute);
  font-size: 0.8rem;
}

.direction-badge {
  display: inline-block;
  padding: 0.2rem 0.5rem;
  border-radius: 4px;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
}

.direction-badge.inbound {
  background: #d4edda;
  color: #155724;
}

.direction-badge.outbound {
  background: #cce5ff;
  color: #004085;
}

.message-type {
  display: inline-block;
  padding: 0.2rem 0.5rem;
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 0.75rem;
  font-weight: 600;
}

.type-handshake {
  background: #fff3cd;
  color: #856404;
}

.type-data {
  background: #f8d7da;
  color: #721c24;
}

.type-address {
  background: #d1ecf1;
  color: #0c5460;
}

.type-sync {
  background: #e2e3e5;
  color: #383d41;
}

.type-other {
  background: #e7e7e7;
  color: #666;
}

.mono {
  font-family: 'Courier New', monospace;
  font-size: 0.8rem;
}

.peer-addr {
  max-width: 150px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.size {
  color: var(--color-text-mute);
  font-size: 0.8rem;
}

.description {
  color: var(--color-text-mute);
  font-size: 0.85rem;
}

.footer {
  text-align: center;
  padding: 1rem;
  color: var(--color-text-mute);
  font-size: 0.85rem;
}

@media (max-width: 768px) {
  .messages-view {
    padding: 1rem;
  }

  .header h2 {
    font-size: 1.5rem;
  }

  .messages-table {
    font-size: 0.75rem;
  }

  .messages-table th,
  .messages-table td {
    padding: 0.5rem 0.25rem;
  }
}
</style>

