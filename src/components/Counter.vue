<template>
  <div class="counter-container">
    <div class="counter-card">
      <h1>Compteur de Clics</h1>
      <div class="click-count">{{ count }}</div>
      <button @click="incrementCount" class="btn-click">
        Cliquer (+1)
      </button>
      <button @click="resetCount" class="btn-reset">
        Réinitialiser
      </button>
      <p v-if="loading" class="loading">Chargement...</p>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { database, ref as dbRef, get, set } from '../firebase';
import { child } from 'firebase/database';

const count = ref(0);
const loading = ref(true);

const loadCount = async () => {
  try {
    const snapshot = await get(dbRef(database, 'counter'));
    if (snapshot.exists()) {
      count.value = snapshot.val();
    } else {
      count.value = 0;
      await set(dbRef(database, 'counter'), 0);
    }
  } catch (error) {
    console.error('Erreur lors du chargement:', error);
  } finally {
    loading.value = false;
  }
};

const incrementCount = async () => {
  count.value++;
  try {
    await set(dbRef(database, 'counter'), count.value);
  } catch (error) {
    console.error('Erreur lors de la mise à jour:', error);
    count.value--;
  }
};

const resetCount = async () => {
  count.value = 0;
  try {
    await set(dbRef(database, 'counter'), 0);
  } catch (error) {
    console.error('Erreur lors de la réinitialisation:', error);
    count.value = 0;
  }
};

onMounted(() => {
  loadCount();
});
</script>

<style scoped>
.counter-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.counter-card {
  background: white;
  padding: 3rem;
  border-radius: 20px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  text-align: center;
  max-width: 400px;
  width: 100%;
}

h1 {
  color: #333;
  font-size: 2rem;
  margin-bottom: 2rem;
  font-weight: 700;
}

.click-count {
  font-size: 5rem;
  font-weight: 800;
  color: #667eea;
  margin: 2rem 0;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.1);
}

.btn-click {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 1rem 2rem;
  font-size: 1.1rem;
  border-radius: 10px;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
  margin-right: 0.5rem;
  font-weight: 600;
}

.btn-click:hover {
  transform: translateY(-2px);
  box-shadow: 0 10px 20px rgba(102, 126, 234, 0.4);
}

.btn-click:active {
  transform: translateY(0);
}

.btn-reset {
  background: #f0f0f0;
  color: #333;
  border: 2px solid #667eea;
  padding: 1rem 2rem;
  font-size: 1.1rem;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s;
  font-weight: 600;
}

.btn-reset:hover {
  background: #667eea;
  color: white;
}

.loading {
  color: #999;
  margin-top: 1rem;
  font-style: italic;
}
</style>
