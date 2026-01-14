<script setup lang="ts">
interface Props {
  progress: number;
  message: string;
}

withDefaults(defineProps<Props>(), {
  progress: 0,
  message: "Converting...",
});
</script>

<template>
  <div class="progress-container">
    <div class="progress-message">{{ message }}</div>
    
    <div class="progress-bar-wrapper">
      <div class="progress-bar">
        <div
          class="progress-fill"
          :style="{ width: progress + '%' }"
        />
      </div>
      <span class="progress-percentage">{{ progress }}%</span>
    </div>
  </div>
</template>

<style scoped>
.progress-container {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.progress-message {
  color: rgba(255, 255, 255, 0.9);
  font-size: 0.95rem;
  font-weight: 500;
}

.progress-bar-wrapper {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.progress-bar {
  flex: 1;
  height: 0.5rem;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 0.25rem;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
  transition: width 0.3s ease;
  box-shadow: 0 0 10px rgba(102, 126, 234, 0.5);
}

.progress-percentage {
  color: rgba(255, 255, 255, 0.8);
  font-weight: 600;
  font-size: 0.85rem;
  min-width: 3rem;
  text-align: right;
}
</style>
