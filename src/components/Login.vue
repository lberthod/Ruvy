<template>
  <div class="login-container">
    <div class="login-card">
      <div class="logo">
        <svg width="60" height="60" viewBox="0 0 60 60" fill="none">
          <circle cx="30" cy="30" r="28" stroke="#667eea" stroke-width="2"/>
          <path d="M30 15V45M15 30H45" stroke="#667eea" stroke-width="2" stroke-linecap="round"/>
        </svg>
      </div>
      <h1>Compteur de Clics</h1>
      <p class="subtitle">Connectez-vous pour continuer</p>

      <button @click="signInWithGoogle" class="btn-google" :disabled="loading">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
          <path d="M23.745 12.27c0-.79-.1-1.54-.257-2.26H12v4.26h6.52c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.08z" fill="#4285F4"/>
          <path d="M12 24c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 22.29 7.75 24 12 24z" fill="#34A853"/>
          <path d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" fill="#FBBC05"/>
          <path d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.75 1 3.99 2.71 2.18 5.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" fill="#EA4335"/>
        </svg>
        <span v-if="!loading">Se connecter avec Google</span>
        <span v-else>Connexion en cours...</span>
      </button>

      <p v-if="error" class="error">{{ error }}</p>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { getAuth, signInWithPopup, GoogleAuthProvider } from 'firebase/auth';
import { initializeApp } from 'firebase/app';

const firebaseConfig = {
  apiKey: "AIzaSyANSoSusZ1eMjcZi9gXfWDbGtrM_feHw2s",
  authDomain: "ruvy-da318.firebaseapp.com",
  databaseURL: "https://ruvy-da318-default-rtdb.europe-west1.firebasedatabase.app",
  projectId: "ruvy-da318",
  storageBucket: "ruvy-da318.firebasestorage.app",
  messagingSenderId: "389926682081",
  appId: "1:389926682081:web:db7f94a240bde4ec07e6a0",
  measurementId: "G-83RER47ML8"
};

const app = initializeApp(firebaseConfig);
const auth = getAuth(app);
const loading = ref(false);
const error = ref('');

const emit = defineEmits(['login-success']);

const signInWithGoogle = async () => {
  try {
    loading.value = true;
    error.value = '';
    const provider = new GoogleAuthProvider();
    const result = await signInWithPopup(auth, provider);
    emit('login-success', result.user);
  } catch (err) {
    console.error('Erreur de connexion:', err);
    error.value = 'Erreur lors de la connexion. Veuillez réessayer.';
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-card {
  background: white;
  padding: 3rem;
  border-radius: 20px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  text-align: center;
  max-width: 400px;
  width: 100%;
}

.logo {
  margin-bottom: 2rem;
  display: flex;
  justify-content: center;
}

h1 {
  color: #333;
  font-size: 2rem;
  margin-bottom: 0.5rem;
  font-weight: 700;
}

.subtitle {
  color: #999;
  font-size: 1rem;
  margin-bottom: 2rem;
}

.btn-google {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  width: 100%;
  padding: 1rem;
  background: white;
  border: 2px solid #e0e0e0;
  border-radius: 10px;
  font-size: 1rem;
  font-weight: 600;
  color: #333;
  cursor: pointer;
  transition: all 0.3s;
}

.btn-google:hover:not(:disabled) {
  border-color: #667eea;
  box-shadow: 0 5px 15px rgba(102, 126, 234, 0.3);
  background: #f8f9ff;
}

.btn-google:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.error {
  color: #e74c3c;
  margin-top: 1rem;
  font-weight: 600;
}
</style>