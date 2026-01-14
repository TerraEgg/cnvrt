<script setup lang="ts">
import { computed } from "vue";
import { useConversionStore } from "../stores/conversion";
import type { FileItem } from "../stores/conversion";

interface Props {
  files: FileItem[];
}

withDefaults(defineProps<Props>(), {});

const store = useConversionStore();

const singleFile = computed(() => {
  return store.files.length === 1 ? store.files[0] : null;
});

const supportedFormats = computed(() => {
  return ["png", "jpg", "jpeg", "webp", "bmp", "gif", "tiff", "ico"];
});

const updateTargetFormat = (fileId: string, format: string) => {
  store.updateFileFormat(fileId, format);
};

const removeFile = (fileId: string) => {
  store.removeFile(fileId);
};

const clearAll = () => {
  store.clearFiles();
};
</script>

<template>
  <div class="preview-container">
    <div class="preview-header">
      <h3 class="preview-title">
        {{ store.files.length === 1 ? "Single File" : "Multiple Files" }}
      </h3>
      <button
        v-if="store.files.length > 0"
        class="clear-btn"
        @click="clearAll"
      >
        Clear All
      </button>
    </div>

    <div v-if="singleFile" class="single-file-preview">
      <div class="file-card">
        <div class="file-icon">📄</div>
        <div class="file-info">
          <p class="file-name">{{ singleFile.name }}</p>
          <p class="file-size">{{ formatFileSize(singleFile.size) }}</p>
        </div>
        <button
          class="remove-btn"
          @click="removeFile(singleFile.id)"
        >
          ✕
        </button>
      </div>

      <div class="format-selector">
        <label class="format-label">Convert to:</label>
        <select
          :value="singleFile.targetFormat"
          @change="(e) => singleFile && updateTargetFormat(singleFile.id, (e.target as HTMLSelectElement).value)"
          class="format-select"
        >
          <option value="">Choose format...</option>
          <option
            v-for="format in supportedFormats"
            :key="format"
            :value="format"
          >
            {{ format.toUpperCase() }}
          </option>
        </select>
      </div>
    </div>

    <div v-else-if="store.files.length > 0" class="multi-file-preview">
      <div
        v-for="file in store.files"
        :key="file.id"
        class="file-row"
      >
        <div class="file-info-compact">
          <span class="file-name-compact">{{ file.name }}</span>
        </div>

        <div class="format-conversion">
          <span class="from-format">{{ file.extension.toUpperCase() }}</span>
          <span class="arrow">→</span>
          <select
            :value="file.targetFormat"
            @change="(e) => updateTargetFormat(file.id, (e.target as HTMLSelectElement).value)"
            class="format-select-compact"
          >
            <option value="">Choose...</option>
            <option
              v-for="format in supportedFormats"
              :key="format"
              :value="format"
            >
              {{ format.toUpperCase() }}
            </option>
          </select>
        </div>

        <button
          class="remove-btn-compact"
          @click="removeFile(file.id)"
        >
          ✕
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
function formatFileSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
}
</script>

<style scoped>
.preview-container {
  background: rgba(255, 255, 255, 0.1);
  padding: 1.5rem;
  border-radius: 1rem;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.preview-title {
  margin: 0;
  color: white;
  font-size: 1.1rem;
  font-weight: 600;
}

.clear-btn {
  padding: 0.5rem 1rem;
  background: rgba(244, 67, 54, 0.3);
  color: white;
  border: 1px solid rgba(244, 67, 54, 0.5);
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 0.85rem;
  font-weight: 500;
}

.clear-btn:hover {
  background: rgba(244, 67, 54, 0.5);
}

.single-file-preview {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.file-card {
  background: rgba(255, 255, 255, 0.05);
  padding: 1.5rem;
  border-radius: 0.75rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.file-icon {
  font-size: 2rem;
  min-width: 3rem;
  text-align: center;
}

.file-info {
  flex: 1;
}

.file-name {
  margin: 0;
  color: white;
  font-weight: 600;
  font-size: 1rem;
  word-break: break-all;
}

.file-size {
  margin: 0.25rem 0 0;
  color: rgba(255, 255, 255, 0.6);
  font-size: 0.85rem;
}

.remove-btn {
  background: rgba(244, 67, 54, 0.3);
  color: white;
  border: none;
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 0.5rem;
  cursor: pointer;
  font-size: 1.2rem;
  transition: all 0.3s ease;
}

.remove-btn:hover {
  background: rgba(244, 67, 54, 0.6);
}

.format-selector {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.format-label {
  color: white;
  font-weight: 600;
  font-size: 0.95rem;
}

.format-select {
  padding: 0.75rem;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 0.5rem;
  color: white;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.3s ease;
}

.format-select:hover,
.format-select:focus {
  background: rgba(255, 255, 255, 0.15);
  border-color: rgba(255, 255, 255, 0.4);
  outline: none;
}

.format-select option {
  background: #333;
  color: white;
}

.multi-file-preview {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  max-height: 50vh;
  overflow-y: auto;
}

.file-row {
  display: grid;
  grid-template-columns: 1fr auto auto;
  gap: 1rem;
  align-items: center;
  background: rgba(255, 255, 255, 0.05);
  padding: 1rem;
  border-radius: 0.5rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.file-info-compact {
  overflow: hidden;
}

.file-name-compact {
  color: white;
  font-size: 0.9rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: block;
}

.format-conversion {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.from-format {
  background: rgba(76, 175, 80, 0.3);
  color: #a5d6a7;
  padding: 0.4rem 0.8rem;
  border-radius: 0.3rem;
  font-size: 0.8rem;
  font-weight: 600;
}

.arrow {
  color: rgba(255, 255, 255, 0.6);
  font-weight: bold;
}

.format-select-compact {
  padding: 0.4rem 0.6rem;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 0.3rem;
  color: white;
  font-size: 0.8rem;
  cursor: pointer;
}

.format-select-compact:hover,
.format-select-compact:focus {
  background: rgba(255, 255, 255, 0.15);
  border-color: rgba(255, 255, 255, 0.4);
  outline: none;
}

.format-select-compact option {
  background: #333;
  color: white;
}

.remove-btn-compact {
  background: rgba(244, 67, 54, 0.3);
  color: white;
  border: none;
  width: 2rem;
  height: 2rem;
  border-radius: 0.3rem;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.3s ease;
}

.remove-btn-compact:hover {
  background: rgba(244, 67, 54, 0.6);
}
</style>
