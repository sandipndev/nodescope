<script setup lang="ts">
import MessagesViewComponent from '../components/MessagesView.vue'
</script>

<template>
  <div class="messages-view-wrapper">
    <div class="messages-header">
      <div class="container">
        <div class="header-content">
          <div class="header-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
            </svg>
          </div>
          <div>
            <h1>Network Messages</h1>
            <p>Real-time P2P message monitoring and analysis</p>
          </div>
        </div>
      </div>
    </div>
    
    <MessagesViewComponent />
  </div>
</template>

<style scoped>
.messages-view-wrapper {
  width: 100%;
  min-height: 100vh;
  background: var(--color-background);
}

.messages-header {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  padding: 3rem 0;
  margin-bottom: 2rem;
}

.container {
  max-width: 1600px;
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
  .messages-header {
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

