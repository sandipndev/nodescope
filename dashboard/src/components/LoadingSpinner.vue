<template>
  <div class="loading-spinner">
    <div class="spinner">
      <div class="spinner-ring"></div>
      <div class="spinner-ring"></div>
      <div class="spinner-ring"></div>
      <svg class="spinner-icon" viewBox="0 0 24 24" fill="currentColor">
        <circle cx="12" cy="12" r="3"/>
        <circle cx="12" cy="12" r="1"/>
      </svg>
    </div>
    <p v-if="message" class="loading-message">{{ message }}</p>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  message?: string
}>()
</script>

<style scoped>
.loading-spinner {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  min-height: 300px;
}

.spinner {
  width: 80px;
  height: 80px;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.spinner-ring {
  position: absolute;
  width: 100%;
  height: 100%;
  border-radius: 50%;
  border: 3px solid transparent;
  border-top-color: var(--accent-blue);
  animation: spin 1.5s linear infinite;
}

.spinner-ring:nth-child(1) {
  animation-duration: 1.5s;
  border-top-color: #667eea;
}

.spinner-ring:nth-child(2) {
  animation-duration: 2s;
  border-top-color: #764ba2;
  animation-delay: 0.2s;
}

.spinner-ring:nth-child(3) {
  animation-duration: 2.5s;
  border-top-color: #f7931a;
  animation-delay: 0.4s;
}

.spinner-icon {
  width: 32px;
  height: 32px;
  color: var(--accent-blue);
  animation: pulse 2s ease-in-out infinite;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.5;
    transform: scale(0.8);
  }
}

.loading-message {
  margin-top: 1.5rem;
  font-size: 1.1rem;
  color: var(--color-text-mute);
  text-align: center;
  animation: fadeIn 0.5s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}
</style>

