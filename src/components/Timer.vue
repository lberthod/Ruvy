<template>
  <div class="tool-card">
    <h2>⏱️ Timer</h2>
    <div class="timer-display">{{ formatTime }}</div>

    <div class="input-group">
      <input v-model.number="minutes" type="number" placeholder="Minutes" min="0" max="59" :disabled="running">
      <span>:</span>
      <input v-model.number="seconds" type="number" placeholder="Secondes" min="0" max="59" :disabled="running">
    </div>

    <div class="timer-controls">
      <button v-if="!running" @click="startTimer" class="btn-start">Démarrer</button>
      <button v-else @click="pauseTimer" class="btn-pause">Pause</button>
      <button @click="resetTimer" class="btn-reset">Réinitialiser</button>
    </div>

    <div v-if="finished" class="timer-finished">
      ⏰ Temps écoulé !
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onUnmounted } from 'vue';

const minutes = ref(1);
const seconds = ref(0);
const running = ref(false);
const finished = ref(false);
let interval = null;

const totalSeconds = computed(() => minutes.value * 60 + seconds.value);

const formatTime = computed(() => {
  const mins = String(Math.floor(totalSeconds.value / 60)).padStart(2, '0');
  const secs = String(totalSeconds.value % 60).padStart(2, '0');
  return `${mins}:${secs}`;
});

const startTimer = () => {
  running.value = true;
  finished.value = false;

  interval = setInterval(() => {
    let total = minutes.value * 60 + seconds.value;

    if (total <= 0) {
      running.value = false;
      finished.value = true;
      clearInterval(interval);
      return;
    }

    total--;
    minutes.value = Math.floor(total / 60);
    seconds.value = total % 60;
  }, 1000);
};

const pauseTimer = () => {
  running.value = false;
  clearInterval(interval);
};

const resetTimer = () => {
  running.value = false;
  finished.value = false;
  clearInterval(interval);
  minutes.value = 1;
  seconds.value = 0;
};

onUnmounted(() => {
  if (interval) clearInterval(interval);
});
</script>