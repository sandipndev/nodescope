<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <header>
    <div class="wrapper">
      <h1 class="app-title">NodeScope</h1>

      <nav>
        <RouterLink to="/">Peers</RouterLink>
        <RouterLink to="/about">About</RouterLink>
      </nav>
    </div>
  </header>

  <RouterView />
</template>

<style scoped>
header {
  line-height: 1.5;
  background: var(--color-background-soft);
  border-bottom: 1px solid var(--color-border);
  padding: 1rem 2rem;
}

.wrapper {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.app-title {
  margin: 0;
  font-size: 1.5rem;
  color: var(--color-heading);
  font-weight: 700;
}

nav {
  display: flex;
  gap: 1rem;
}

nav a {
  padding: 0.5rem 1rem;
  border-radius: 6px;
  text-decoration: none;
  color: var(--color-text);
  transition: all 0.2s;
  font-weight: 500;
}

nav a:hover {
  background: var(--color-background-mute);
}

nav a.router-link-exact-active {
  background: var(--color-background);
  color: var(--color-heading);
  border: 1px solid var(--color-border);
}

@media (max-width: 768px) {
  header {
    padding: 1rem;
  }

  .app-title {
    font-size: 1.2rem;
  }

  nav {
    gap: 0.5rem;
  }

  nav a {
    padding: 0.4rem 0.8rem;
    font-size: 0.9rem;
  }
}
</style>
