<template>
  <div class="landing-container">
    <nav class="landing-navbar">
      <div class="navbar-brand">
        <svg width="40" height="40" viewBox="0 0 60 60" fill="none">
          <circle cx="30" cy="30" r="28" stroke="#667eea" stroke-width="2"/>
          <path d="M30 15V45M15 30H45" stroke="#667eea" stroke-width="2" stroke-linecap="round"/>
        </svg>
        <span>Ruvy</span>
      </div>
      <button @click="$emit('get-started')" class="btn-get-started">
        Commencer
      </button>
    </nav>

    <section class="hero">
      <div class="hero-content">
        <h1>Bienvenue sur Ruvy</h1>
        <p class="hero-subtitle">Votre application productive tout-en-un</p>
        <p class="hero-description">
          Gérez vos compteurs, chronomètres et bien plus avec une interface intuitive et sécurisée.
        </p>
        <button @click="$emit('get-started')" class="btn-hero">
          Commencer Maintenant
        </button>
      </div>

      <div class="hero-features">
        <div class="feature-card">
          <div class="feature-icon">📊</div>
          <h3>Compteur</h3>
          <p>Suivi précis avec cloud functions</p>
        </div>
        <div class="feature-card">
          <div class="feature-icon">⏱️</div>
          <h3>Timer</h3>
          <p>Chronomètrage en temps réel</p>
        </div>
        <div class="feature-card">
          <div class="feature-icon">📱</div>
          <h3>PWA</h3>
          <p>Installez comme une vraie app</p>
        </div>
        <div class="feature-card">
          <div class="feature-icon">🔐</div>
          <h3>Sécurisé</h3>
          <p>Authentification Google</p>
        </div>
      </div>
    </section>

    <section class="features-detailed">
      <h2>Fonctionnalités</h2>
      <div class="features-grid">
        <div class="feature-item">
          <h3>✨ Interface Moderne</h3>
          <p>Design élégant avec gradient et animations fluides</p>
        </div>
        <div class="feature-item">
          <h3>☁️ Cloud Powered</h3>
          <p>Données synchronisées via Firebase</p>
        </div>
        <div class="feature-item">
          <h3>📲 Installation Simple</h3>
          <p>Un clic pour installer sur téléphone</p>
        </div>
        <div class="feature-item">
          <h3>🚀 Rapide</h3>
          <p>Chargement instantané grâce au service worker</p>
        </div>
      </div>
    </section>

    <footer class="footer">
      <p>&copy; 2026 Ruvy - Application productive</p>
    </footer>
  </div>
</template>

<script setup>
defineEmits(['get-started']);
</script>