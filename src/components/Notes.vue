<template>
  <div class="tool-card">
    <h2>📝 Notes</h2>
    <textarea
      v-model="noteText"
      placeholder="Écrivez vos notes ici..."
      class="notes-textarea"
    ></textarea>
    <div class="notes-info">
      <span>{{ noteText.length }} caractères</span>
      <button @click="saveNote" class="btn-save">💾 Enregistrer</button>
    </div>
    <p v-if="saved" class="saved-message">✅ Enregistré !</p>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';

const noteText = ref('');
const saved = ref(false);

const saveNote = () => {
  localStorage.setItem('ruvy-notes', noteText.value);
  saved.value = true;
  setTimeout(() => {
    saved.value = false;
  }, 2000);
};

onMounted(() => {
  const saved = localStorage.getItem('ruvy-notes');
  if (saved) {
    noteText.value = saved;
  }
});
</script>