<script setup>
import { ref, onMounted } from 'vue';
import { initializeApp } from 'firebase/app';
import { getAuth, onAuthStateChanged } from 'firebase/auth';
import Login from './components/Login.vue';
import Counter from './components/Counter.vue';

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

const currentUser = ref(null);
const loading = ref(true);

onMounted(() => {
  onAuthStateChanged(auth, (user) => {
    currentUser.value = user;
    loading.value = false;
  });
});

const handleLogout = () => {
  currentUser.value = null;
};
</script>

<template>
  <Login v-if="!currentUser && !loading" @login-success="currentUser = $event" />
  <Counter v-else-if="currentUser" :user="currentUser" @logout="handleLogout" />
</template>