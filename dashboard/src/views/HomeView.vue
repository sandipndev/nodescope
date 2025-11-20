<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { usePeerConnections } from '../composables/usePeerConnections'
import { useRecentMessages } from '../composables/useMessages'

const { connections, stats } = usePeerConnections(true, 5000)
const { messages } = useRecentMessages(50, true, 5000)

const heroVisible = ref(false)

onMounted(() => {
  setTimeout(() => {
    heroVisible.value = true
  }, 100)
})

const recentMessageTypes = computed(() => {
  const types: Record<string, number> = {}
  messages.value.slice(0, 20).forEach(msg => {
    types[msg.messageType] = (types[msg.messageType] || 0) + 1
  })
  return Object.entries(types).slice(0, 5).map(([type, count]) => ({ type, count }))
})

const formatBytes = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`
}

const getQualityClass = (): string => {
  const active = stats.value?.activeConnections || 0
  if (active >= 5) return 'quality-excellent'
  if (active >= 2) return 'quality-good'
  return 'quality-poor'
}

const getQualityText = (): string => {
  const active = stats.value?.activeConnections || 0
  if (active >= 5) return 'Excellent Connection'
  if (active >= 2) return 'Good Connection'
  return 'Limited Connectivity'
}
</script>

<template>
  <div class="home-view">
    <!-- Hero Section -->
    <section class="hero" :class="{ visible: heroVisible }">
      <div class="hero-content">
        <div class="hero-text">
          <h1 class="hero-title">
            <span class="text-gradient">Total Visibility</span>
            <br />for your Bitcoin Node
          </h1>
          <p class="hero-subtitle">
            Monitor peer connections, track real-time P2P gossip, and understand how your node participates in the Bitcoin network.
          </p>
          <div class="hero-actions">
            <a href="#dashboard" class="btn btn-primary">
              <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M3 3h7v7H3zM14 3h7v7h-7zM14 14h7v7h-7zM3 14h7v7H3z"/>
              </svg>
              View Dashboard
            </a>
            <a href="#about" class="btn btn-secondary">
              Learn More
            </a>
          </div>
        </div>
        <div class="hero-visual">
          <div class="network-animation">
            <div class="node central-node">
              <div class="node-pulse"></div>
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
              </svg>
            </div>
            <div class="peer-nodes">
              <div class="peer-node" v-for="i in 8" :key="i" :style="{ '--delay': i * 0.2 + 's', '--angle': i * 45 + 'deg' }">
                <div class="connection-line"></div>
                <div class="node-dot"></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Dashboard Section -->
    <section id="dashboard" class="dashboard-section">
      <div class="container">
        <h2 class="section-title">Live Network Overview</h2>
        
        <!-- Key Stats -->
        <div class="stats-showcase">
          <div class="stat-box stat-primary">
            <div class="stat-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
                <circle cx="9" cy="7" r="4"></circle>
                <path d="M23 21v-2a4 4 0 0 0-3-3.87"></path>
                <path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
              </svg>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ stats?.activeConnections || 0 }}</div>
              <div class="stat-label">Active Peers</div>
            </div>
          </div>

          <div class="stat-box stat-success">
            <div class="stat-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline>
              </svg>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ messages.length }}</div>
              <div class="stat-label">Recent Messages</div>
            </div>
          </div>

          <div class="stat-box stat-info">
            <div class="stat-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                <polyline points="7 10 12 15 17 10"></polyline>
                <line x1="12" y1="15" x2="12" y2="3"></line>
              </svg>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ formatBytes(stats?.totalBytesInbound || 0) }}</div>
              <div class="stat-label">Data Received</div>
            </div>
          </div>

          <div class="stat-box stat-warning">
            <div class="stat-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                <polyline points="17 8 12 3 7 8"></polyline>
                <line x1="12" y1="3" x2="12" y2="15"></line>
              </svg>
            </div>
            <div class="stat-content">
              <div class="stat-value">{{ formatBytes(stats?.totalBytesOutbound || 0) }}</div>
              <div class="stat-label">Data Sent</div>
            </div>
          </div>
        </div>

        <!-- Activity Cards -->
        <div class="activity-grid">
          <div class="activity-card">
            <h3>Recent Message Types</h3>
            <div class="message-types">
              <div v-for="{ type, count } in recentMessageTypes" :key="type" class="message-type-item">
                <span class="type-badge">{{ type }}</span>
                <span class="type-count">{{ count }}</span>
              </div>
              <div v-if="recentMessageTypes.length === 0" class="empty-message">
                No recent messages
              </div>
            </div>
          </div>

          <div class="activity-card">
            <h3>Connection Quality</h3>
            <div class="quality-indicator">
              <div class="quality-circle" :class="getQualityClass()">
                <div class="quality-inner">
                  <svg viewBox="0 0 24 24" fill="currentColor">
                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
                  </svg>
                </div>
              </div>
              <p class="quality-text">{{ getQualityText() }}</p>
            </div>
          </div>

          <div class="activity-card">
            <h3>Network Status</h3>
            <div class="status-list">
              <div class="status-item">
                <span class="status-dot active"></span>
                <span>Proxy Active</span>
              </div>
              <div class="status-item">
                <span class="status-dot active"></span>
                <span>GraphQL Server</span>
              </div>
              <div class="status-item">
                <span class="status-dot active"></span>
                <span>Database Connected</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Quick Actions -->
        <div class="quick-actions">
          <router-link to="/peers" class="action-card">
            <div class="action-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
                <circle cx="9" cy="7" r="4"></circle>
                <path d="M23 21v-2a4 4 0 0 0-3-3.87"></path>
                <path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
              </svg>
            </div>
            <h3>View All Peers</h3>
            <p>Explore detailed peer connections and history</p>
          </router-link>

          <router-link to="/messages" class="action-card">
            <div class="action-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
              </svg>
            </div>
            <h3>View Messages</h3>
            <p>Monitor real-time P2P message flows</p>
          </router-link>

          <router-link to="/about" class="action-card">
            <div class="action-icon">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="12" y1="16" x2="12" y2="12"></line>
                <line x1="12" y1="8" x2="12.01" y2="8"></line>
              </svg>
            </div>
            <h3>About NodeScope</h3>
            <p>Learn more about how it works</p>
          </router-link>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped>
.home-view {
  width: 100%;
}

/* Hero Section */
.hero {
  min-height: 90vh;
  display: flex;
  align-items: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 50%, #f7931a 100%);
  position: relative;
  overflow: hidden;
  opacity: 0;
  transition: opacity 0.8s ease;
}

.hero.visible {
  opacity: 1;
}

.hero::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: 
    radial-gradient(circle at 20% 50%, rgba(255, 255, 255, 0.1) 0%, transparent 50%),
    radial-gradient(circle at 80% 80%, rgba(255, 255, 255, 0.1) 0%, transparent 50%);
  pointer-events: none;
}

.hero-content {
  max-width: 1400px;
  margin: 0 auto;
  padding: 4rem 2rem;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4rem;
  align-items: center;
  position: relative;
  z-index: 1;
}

.hero-text {
  animation: fadeInUp 0.8s ease-out;
}

.hero-title {
  font-size: 4rem;
  font-weight: 800;
  line-height: 1.1;
  margin-bottom: 1.5rem;
  color: white;
}

.hero-subtitle {
  font-size: 1.25rem;
  line-height: 1.6;
  color: rgba(255, 255, 255, 0.9);
  margin-bottom: 2rem;
}

.hero-actions {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem 2rem;
  border-radius: 12px;
  font-size: 1.1rem;
  font-weight: 600;
  text-decoration: none;
  transition: all 0.3s ease;
  cursor: pointer;
}

.btn-primary {
  background: white;
  color: #667eea;
  box-shadow: 0 4px 14px rgba(0, 0, 0, 0.2);
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.2);
  color: white;
  border: 2px solid white;
  backdrop-filter: blur(10px);
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.3);
  transform: translateY(-2px);
}

.btn .icon {
  width: 20px;
  height: 20px;
}

/* Network Animation */
.hero-visual {
  display: flex;
  justify-content: center;
  align-items: center;
  animation: fadeInUp 0.8s ease-out 0.2s backwards;
}

.network-animation {
  position: relative;
  width: 400px;
  height: 400px;
}

.central-node {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 100px;
  height: 100px;
  background: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  z-index: 10;
}

.central-node svg {
  width: 50px;
  height: 50px;
  color: #667eea;
}

.node-pulse {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 100%;
  height: 100%;
  border-radius: 50%;
  border: 3px solid rgba(255, 255, 255, 0.6);
  animation: pulse-ring 2s infinite;
}

@keyframes pulse-ring {
  0% {
    transform: translate(-50%, -50%) scale(1);
    opacity: 1;
  }
  100% {
    transform: translate(-50%, -50%) scale(2.5);
    opacity: 0;
  }
}

.peer-nodes {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

.peer-node {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 100%;
  height: 100%;
  transform-origin: center;
  transform: rotate(var(--angle));
  animation: float 3s ease-in-out infinite;
  animation-delay: var(--delay);
}

@keyframes float {
  0%, 100% {
    transform: rotate(var(--angle)) translateY(0);
  }
  50% {
    transform: rotate(var(--angle)) translateY(-10px);
  }
}

.connection-line {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 150px;
  height: 2px;
  background: linear-gradient(90deg, rgba(255, 255, 255, 0.6) 0%, rgba(255, 255, 255, 0.2) 100%);
  transform-origin: left center;
  animation: pulse 2s ease-in-out infinite;
  animation-delay: var(--delay);
}

.node-dot {
  position: absolute;
  top: 50%;
  left: calc(50% + 150px);
  width: 40px;
  height: 40px;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 50%;
  transform: translate(-50%, -50%);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
  animation: pulse 2s ease-in-out infinite;
  animation-delay: var(--delay);
}

/* Dashboard Section */
.dashboard-section {
  padding: 4rem 0;
  background: var(--color-background);
}

.section-title {
  font-size: 2.5rem;
  font-weight: 700;
  text-align: center;
  margin-bottom: 3rem;
  color: var(--color-heading);
}

.stats-showcase {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
  margin-bottom: 3rem;
}

.stat-box {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  padding: 2rem;
  border-radius: 16px;
  background: var(--color-background-soft);
  border: 1px solid var(--color-border);
  transition: all 0.3s ease;
}

.stat-box:hover {
  transform: translateY(-4px);
  box-shadow: 0 12px 24px rgba(0, 0, 0, 0.1);
}

.stat-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  color: white;
}

.stat-success {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  border: none;
  color: white;
}

.stat-info {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  border: none;
  color: white;
}

.stat-warning {
  background: linear-gradient(135deg, #f7931a 0%, #e07f00 100%);
  border: none;
  color: white;
}

.stat-icon {
  width: 60px;
  height: 60px;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(10px);
}

.stat-icon svg {
  width: 32px;
  height: 32px;
}

.stat-content {
  flex: 1;
}

.stat-value {
  font-size: 2.5rem;
  font-weight: 700;
  line-height: 1;
  margin-bottom: 0.25rem;
}

.stat-label {
  font-size: 0.9rem;
  opacity: 0.9;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

/* Activity Grid */
.activity-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
  margin-bottom: 3rem;
}

.activity-card {
  background: var(--color-background-soft);
  border: 1px solid var(--color-border);
  border-radius: 16px;
  padding: 2rem;
  transition: all 0.3s ease;
}

.activity-card:hover {
  border-color: var(--color-border-hover);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
}

.activity-card h3 {
  font-size: 1.25rem;
  font-weight: 600;
  margin-bottom: 1.5rem;
  color: var(--color-heading);
}

.message-types {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.message-type-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: var(--color-background-mute);
  border-radius: 8px;
}

.type-badge {
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--color-heading);
}

.type-count {
  background: var(--accent-blue);
  color: white;
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.85rem;
  font-weight: 600;
}

.empty-message {
  text-align: center;
  padding: 2rem;
  color: var(--color-text-mute);
}

.quality-indicator {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  padding: 1rem 0;
}

.quality-circle {
  width: 100px;
  height: 100px;
  border-radius: 50%;
  padding: 8px;
  animation: spin 20s linear infinite;
}

.quality-excellent {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
}

.quality-good {
  background: linear-gradient(135deg, #f7931a 0%, #e07f00 100%);
}

.quality-poor {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
}

.quality-inner {
  width: 100%;
  height: 100%;
  background: var(--color-background);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.quality-inner svg {
  width: 40px;
  height: 40px;
}

.quality-excellent .quality-inner svg {
  color: #10b981;
}

.quality-good .quality-inner svg {
  color: #f7931a;
}

.quality-poor .quality-inner svg {
  color: #ef4444;
}

.quality-text {
  font-weight: 600;
  font-size: 1.1rem;
  color: var(--color-heading);
}

.status-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem;
  background: var(--color-background-mute);
  border-radius: 8px;
}

.status-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #6b7280;
}

.status-dot.active {
  background: #10b981;
  box-shadow: 0 0 0 4px rgba(16, 185, 129, 0.2);
  animation: pulse 2s ease-in-out infinite;
}

/* Quick Actions */
.quick-actions {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
}

.action-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 2.5rem;
  background: var(--color-background-soft);
  border: 2px solid var(--color-border);
  border-radius: 16px;
  text-decoration: none;
  transition: all 0.3s ease;
}

.action-card:hover {
  border-color: var(--accent-blue);
  transform: translateY(-4px);
  box-shadow: 0 12px 24px rgba(0, 0, 0, 0.1);
}

.action-icon {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--accent-blue) 0%, var(--accent-purple) 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 1.5rem;
  color: white;
}

.action-icon svg {
  width: 40px;
  height: 40px;
}

.action-card h3 {
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 0.75rem;
  color: var(--color-heading);
}

.action-card p {
  color: var(--color-text-mute);
  font-size: 1rem;
  line-height: 1.6;
}

/* Responsive */
@media (max-width: 1024px) {
  .hero-content {
    grid-template-columns: 1fr;
    gap: 2rem;
  }

  .hero-title {
    font-size: 3rem;
  }

  .network-animation {
    width: 300px;
    height: 300px;
  }
}

@media (max-width: 768px) {
  .hero {
    min-height: auto;
    padding: 3rem 0;
  }

  .hero-title {
    font-size: 2.5rem;
  }

  .hero-subtitle {
    font-size: 1.1rem;
  }

  .network-animation {
    width: 250px;
    height: 250px;
  }

  .central-node {
    width: 80px;
    height: 80px;
  }

  .section-title {
    font-size: 2rem;
  }

  .stats-showcase {
    grid-template-columns: 1fr;
  }
}
</style>
