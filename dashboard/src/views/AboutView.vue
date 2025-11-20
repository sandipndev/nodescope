<template>
  <div class="about-page">
    <div class="container">
      <!-- Header Section -->
      <section class="about-header">
        <div class="logo-section">
          <div class="logo-circle">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1-13h2v6h-2zm0 8h2v2h-2z"/>
            </svg>
          </div>
          <h1 class="about-title">
            <span class="text-gradient">NodeScope</span>
          </h1>
          <p class="tagline">Total Visibility for your Bitcoin Node</p>
        </div>
      </section>

      <!-- What is NodeScope -->
      <section class="content-section">
        <h2>What is NodeScope?</h2>
        <p class="lead-text">
          Running a Bitcoin node is powerful, but also opaque. You know it's syncing, you know it's connected, 
          but <strong>what exactly is going on under the hood?</strong>
        </p>
        <p>
          NodeScope brings <strong>full visibility</strong> to your Bitcoin nodes. You can monitor which peers your 
          node is connected to, observe real-time P2P gossip, and track how your node participates in the network.
        </p>
        <p>
          Historical data about peer connections, message flows, and data exchange helps you understand network 
          topology, peer quality, and your node's behavior over time.
        </p>
      </section>

      <!-- Features -->
      <section class="features-section">
        <h2>Key Features</h2>
        <div class="features-grid">
          <div class="feature-card">
            <div class="feature-icon icon-peers">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
                <circle cx="9" cy="7" r="4"></circle>
                <path d="M23 21v-2a4 4 0 0 0-3-3.87"></path>
                <path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
              </svg>
            </div>
            <h3>Peer Connection Tracking</h3>
            <p>
              Historical records of all peers your node connects to. Track connection duration, data transfer, 
              and peer reliability over time.
            </p>
          </div>

          <div class="feature-card">
            <div class="feature-icon icon-messages">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
              </svg>
            </div>
            <h3>Real-Time Message Monitoring</h3>
            <p>
              Observe every message sent and received from each peer with timestamps. Understand the flow of 
              transactions, blocks, and network gossip.
            </p>
          </div>

          <div class="feature-card">
            <div class="feature-icon icon-handshake">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
                <circle cx="8.5" cy="7" r="4"></circle>
                <polyline points="17 11 19 13 23 9"></polyline>
              </svg>
            </div>
            <h3>Handshake Analysis</h3>
            <p>
              Track peer versions, services, and connection handshakes. Monitor both successful connections 
              and failed attempts to understand network dynamics.
            </p>
          </div>

          <div class="feature-card">
            <div class="feature-icon icon-data">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="12" y1="1" x2="12" y2="23"></line>
                <path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"></path>
              </svg>
            </div>
            <h3>Data Exchange Analytics</h3>
            <p>
              Monitor bandwidth usage, message types, and data transfer patterns. Understand how your node 
              contributes to the Bitcoin network.
            </p>
          </div>

          <div class="feature-card">
            <div class="feature-icon icon-topology">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"></circle>
                <circle cx="12" cy="12" r="6"></circle>
                <circle cx="12" cy="12" r="2"></circle>
              </svg>
            </div>
            <h3>Network Topology Insights</h3>
            <p>
              Visualize your node's position in the Bitcoin network. Understand peer diversity, geographic 
              distribution, and connection patterns.
            </p>
          </div>

          <div class="feature-card">
            <div class="feature-icon icon-history">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"></circle>
                <polyline points="12 6 12 12 16 14"></polyline>
              </svg>
            </div>
            <h3>Historical Records</h3>
            <p>
              Store and query historical data about your node's behavior. Track performance trends and 
              identify patterns over weeks and months.
            </p>
          </div>
        </div>
      </section>

      <!-- How it Works -->
      <section class="content-section">
        <h2>How It Works</h2>
        <div class="architecture-diagram">
          <div class="arch-item">
            <div class="arch-number">1</div>
            <div class="arch-content">
              <h3>Lightweight Proxy</h3>
              <p>NodeScope runs a SOCKS5 proxy that intercepts Bitcoin P2P traffic without modifying your node.</p>
            </div>
          </div>
          <div class="arch-arrow">→</div>
          <div class="arch-item">
            <div class="arch-number">2</div>
            <div class="arch-content">
              <h3>Data Collection</h3>
              <p>Captures peer connections, message types, and network activity in real-time.</p>
            </div>
          </div>
          <div class="arch-arrow">→</div>
          <div class="arch-item">
            <div class="arch-number">3</div>
            <div class="arch-content">
              <h3>Storage & Analysis</h3>
              <p>Stores data in a local database and provides GraphQL API for querying.</p>
            </div>
          </div>
          <div class="arch-arrow">→</div>
          <div class="arch-item">
            <div class="arch-number">4</div>
            <div class="arch-content">
              <h3>Dashboard</h3>
              <p>Beautiful web interface for exploring and visualizing your node's activity.</p>
            </div>
          </div>
        </div>
      </section>

      <!-- Tech Stack -->
      <section class="tech-section">
        <h2>Built With</h2>
        <div class="tech-stack">
          <div class="tech-badge">
            <span class="tech-icon">🦀</span>
            <span>Rust</span>
          </div>
          <div class="tech-badge">
            <span class="tech-icon">📊</span>
            <span>GraphQL</span>
          </div>
          <div class="tech-badge">
            <span class="tech-icon">💚</span>
            <span>Vue.js</span>
          </div>
          <div class="tech-badge">
            <span class="tech-icon">🗄️</span>
            <span>SQLite</span>
          </div>
          <div class="tech-badge">
            <span class="tech-icon">⚡</span>
            <span>TypeScript</span>
          </div>
          <div class="tech-badge">
            <span class="tech-icon">🔒</span>
            <span>SOCKS5 Proxy</span>
          </div>
        </div>
      </section>

      <!-- Open Source -->
      <section class="open-source-section">
        <div class="oss-card">
          <div class="oss-icon">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
            </svg>
          </div>
          <h2>Open Source</h2>
          <p>
            NodeScope is open source and available under the MIT License. We welcome contributions from the community!
          </p>
          <div class="oss-actions">
            <a href="https://github.com/sandipndev/nodescope" target="_blank" class="btn-github">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
              View on GitHub
            </a>
          </div>
        </div>
      </section>

      <!-- Footer -->
      <footer class="about-footer">
        <p>Built with ❤️ for the Bitcoin community</p>
        <p class="license">MIT License • 2024</p>
      </footer>
    </div>
  </div>
</template>

<style scoped>
.about-page {
  width: 100%;
  padding: 4rem 0;
  background: var(--color-background);
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem;
}

/* Header Section */
.about-header {
  text-align: center;
  padding: 3rem 0;
  margin-bottom: 4rem;
}

.logo-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}

.logo-circle {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  box-shadow: 0 12px 32px rgba(102, 126, 234, 0.3);
  animation: fadeInUp 0.6s ease-out;
}

.logo-circle svg {
  width: 60px;
  height: 60px;
}

.about-title {
  font-size: 4rem;
  font-weight: 800;
  margin: 0;
  animation: fadeInUp 0.6s ease-out 0.1s backwards;
}

.tagline {
  font-size: 1.5rem;
  color: var(--color-text-mute);
  margin: 0;
  animation: fadeInUp 0.6s ease-out 0.2s backwards;
}

/* Content Sections */
.content-section {
  margin-bottom: 4rem;
  animation: fadeInUp 0.6s ease-out;
}

.content-section h2 {
  font-size: 2.5rem;
  font-weight: 700;
  margin-bottom: 1.5rem;
  color: var(--color-heading);
}

.lead-text {
  font-size: 1.25rem;
  line-height: 1.8;
  color: var(--color-text);
  margin-bottom: 1.5rem;
}

.content-section p {
  font-size: 1.1rem;
  line-height: 1.8;
  color: var(--color-text-mute);
  margin-bottom: 1rem;
}

/* Features Section */
.features-section {
  margin-bottom: 4rem;
}

.features-section h2 {
  font-size: 2.5rem;
  font-weight: 700;
  margin-bottom: 2rem;
  color: var(--color-heading);
  text-align: center;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
}

.feature-card {
  background: var(--color-background-soft);
  border: 1px solid var(--color-border);
  border-radius: 16px;
  padding: 2rem;
  transition: all 0.3s ease;
}

.feature-card:hover {
  border-color: var(--color-border-hover);
  transform: translateY(-4px);
  box-shadow: 0 12px 24px rgba(0, 0, 0, 0.1);
}

.feature-icon {
  width: 70px;
  height: 70px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 1.5rem;
  color: white;
}

.icon-peers {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.icon-messages {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
}

.icon-handshake {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
}

.icon-data {
  background: linear-gradient(135deg, #f7931a 0%, #e07f00 100%);
}

.icon-topology {
  background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
}

.icon-history {
  background: linear-gradient(135deg, #06b6d4 0%, #0891b2 100%);
}

.feature-icon svg {
  width: 36px;
  height: 36px;
}

.feature-card h3 {
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 1rem;
  color: var(--color-heading);
}

.feature-card p {
  font-size: 1rem;
  line-height: 1.6;
  color: var(--color-text-mute);
  margin: 0;
}

/* Architecture Diagram */
.architecture-diagram {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 2rem 0;
  overflow-x: auto;
}

.arch-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  flex: 1;
  min-width: 200px;
}

.arch-number {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--accent-blue) 0%, var(--accent-purple) 100%);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  font-weight: 700;
  margin-bottom: 1rem;
}

.arch-content h3 {
  font-size: 1.25rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
  color: var(--color-heading);
}

.arch-content p {
  font-size: 0.95rem;
  line-height: 1.5;
  color: var(--color-text-mute);
  margin: 0;
}

.arch-arrow {
  font-size: 2rem;
  color: var(--color-text-mute);
  flex-shrink: 0;
}

/* Tech Stack */
.tech-section {
  margin-bottom: 4rem;
}

.tech-section h2 {
  font-size: 2.5rem;
  font-weight: 700;
  margin-bottom: 2rem;
  color: var(--color-heading);
  text-align: center;
}

.tech-stack {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  justify-content: center;
}

.tech-badge {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1.5rem;
  background: var(--color-background-soft);
  border: 1px solid var(--color-border);
  border-radius: 50px;
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--color-heading);
  transition: all 0.3s ease;
}

.tech-badge:hover {
  border-color: var(--accent-blue);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.tech-icon {
  font-size: 1.5rem;
}

/* Open Source Section */
.open-source-section {
  margin-bottom: 4rem;
}

.oss-card {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 24px;
  padding: 4rem 3rem;
  text-align: center;
  color: white;
}

.oss-icon {
  width: 80px;
  height: 80px;
  margin: 0 auto 1.5rem;
}

.oss-icon svg {
  width: 100%;
  height: 100%;
  opacity: 0.9;
}

.oss-card h2 {
  font-size: 2.5rem;
  font-weight: 700;
  margin-bottom: 1rem;
}

.oss-card p {
  font-size: 1.25rem;
  line-height: 1.6;
  opacity: 0.95;
  margin-bottom: 2rem;
  max-width: 600px;
  margin-left: auto;
  margin-right: auto;
}

.oss-actions {
  display: flex;
  justify-content: center;
}

.btn-github {
  display: inline-flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem 2rem;
  background: white;
  color: #667eea;
  border-radius: 12px;
  font-size: 1.1rem;
  font-weight: 600;
  text-decoration: none;
  transition: all 0.3s ease;
}

.btn-github:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.2);
}

.btn-github svg {
  width: 24px;
  height: 24px;
}

/* Footer */
.about-footer {
  text-align: center;
  padding: 3rem 0 2rem;
  border-top: 1px solid var(--color-border);
  margin-top: 4rem;
}

.about-footer p {
  margin: 0.5rem 0;
  color: var(--color-text-mute);
  font-size: 1rem;
}

.license {
  font-size: 0.9rem;
  color: var(--color-text-faint);
}

/* Responsive */
@media (max-width: 768px) {
  .about-title {
    font-size: 2.5rem;
  }

  .tagline {
    font-size: 1.25rem;
  }

  .content-section h2,
  .features-section h2,
  .tech-section h2 {
    font-size: 2rem;
  }

  .architecture-diagram {
    flex-direction: column;
  }

  .arch-arrow {
    transform: rotate(90deg);
    margin: 1rem 0;
  }

  .features-grid {
    grid-template-columns: 1fr;
  }

  .oss-card {
    padding: 3rem 2rem;
  }
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
