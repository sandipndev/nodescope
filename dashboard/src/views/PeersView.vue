<script setup lang="ts">
import PeerConnectionsList from '../components/PeerConnectionsList.vue'
</script>

<template>
  <div class="peers-view">
    <div class="peers-header">
      <div class="container">
        <div class="header-content">
          <div class="header-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
              <circle cx="9" cy="7" r="4"></circle>
              <path d="M23 21v-2a4 4 0 0 0-3-3.87"></path>
              <path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
            </svg>
          </div>
          <div>
            <h1>Peer Connections</h1>
            <p>Monitor and track all connections to your Bitcoin node</p>
          </div>
        </div>
      </div>
    </div>
    
    <PeerConnectionsList />
  </div>
</template>

<style scoped>
.peers-view {
  width: 100%;
  min-height: 100vh;
  background: var(--color-background);
}

.peers-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  padding: 3rem 0;
  margin-bottom: 2rem;
}

.container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 2rem;
}

.header-content {
  display: flex;
  align-items: center;
  gap: 1.5rem;
}

.header-icon {
  width: 80px;
  height: 80px;
  border-radius: 20px;
  background: rgba(255, 255, 255, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  backdrop-filter: blur(10px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
}

.header-icon svg {
  width: 40px;
  height: 40px;
}

.header-content h1 {
  margin: 0;
  font-size: 2.5rem;
  font-weight: 800;
  color: white;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.header-content p {
  margin: 0.5rem 0 0;
  font-size: 1.1rem;
  color: rgba(255, 255, 255, 0.9);
}

@media (max-width: 768px) {
  .peers-header {
    padding: 2rem 0;
  }

  .header-icon {
    width: 60px;
    height: 60px;
  }

  .header-icon svg {
    width: 30px;
    height: 30px;
  }

  .header-content h1 {
    font-size: 1.75rem;
  }

  .header-content p {
    font-size: 1rem;
  }
}
</style>

