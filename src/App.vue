<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useConversionStore } from "./stores/conversion";
import NotificationCenter from "./components/NotificationCenter.vue";

const store = useConversionStore();

const droppedFiles = computed(() => store.files);
const isProcessing = computed(() => store.isProcessing);
const currentProgress = computed(() => store.currentProgress);

const destinationFolder = ref('');
const keepTransparency = ref(true);
const isImporting = ref(false);

onMounted(async () => {
  try {
    const downloads = await invoke<string | null>('get_downloads_folder');
    if (downloads) {
      destinationFolder.value = downloads;
    }
  } catch (error) {
    console.error('Error getting downloads folder:', error);
    destinationFolder.value = '';
  }
});

const selectOutputFolder = async () => {
  try {
    const selected = await invoke<string | null>('select_folder');
    
    if (selected) {
      destinationFolder.value = selected;
    }
  } catch (error) {
    console.error('Error selecting folder:', error);
  }
};

const pickFiles = async () => {
  if (isImporting.value) {
    return;
  }
  
  try {
    const filePaths = await invoke<string[] | null>('pick_files');
    
    if (filePaths && filePaths.length > 0) {
      isImporting.value = true;
      try {
        for (const filePath of filePaths) {
          const fileName = filePath.split(/[\\\/]/).pop() || filePath;
          const ext = fileName.split('.').pop()?.toLowerCase() || '';
          
          if (!isSupportedFormat(ext)) {
            continue;
          }
          
          store.addFile({
            id: Math.random().toString(36).substring(2),
            name: fileName,
            path: filePath,
            size: 0,
            type: '',
            extension: ext,
            targetFormat: '',
            file: undefined,
            preview: '',
          });
        }
      } finally {
        isImporting.value = false;
      }
    }
  } catch (error) {
    console.error('Error picking files:', error);
  }
};

const handleDrop = (e: DragEvent) => {
  const files = e.dataTransfer?.files;
  
  if (files && files.length > 0) {
    isImporting.value = true;
    try {
      for (let i = 0; i < files.length; i++) {
        const file = files[i];
        if (!isSupportedFile(file)) {
          continue;
        }
        const ext = file.name.split('.').pop()?.toLowerCase() || '';
        store.addFile({
          id: Math.random().toString(36).substring(2),
          name: file.name,
          path: file.name,
          size: file.size,
          type: file.type,
          extension: ext,
          targetFormat: '',
          file: file,
          preview: URL.createObjectURL(file),
        });
      }
    } finally {
      isImporting.value = false;
    }
  }
};

const removeFile = (fileId: string) => {
  store.removeFile(fileId);
};

const updateFormat = (ext: string, format: string) => {
  for (const file of droppedFiles.value) {
    if (file.extension === ext) {
      store.updateFileFormat(file.id, format);
    }
  }
};

const supportedFormats = ['png', 'jpg', 'jpeg', 'jfif', 'webp', 'bmp', 'gif', 'tiff', 'tif', 'ico', 'avif', 'heic', 'heif', 'ppm', 'pgm', 'pbm', 'tga', 'dds', 'apng', 'cur', 'exr', 'svg', 'pdf', 'psd', 'psb', 'fits', 'dcm', 'pcx', 'mp4', 'm4v', 'mkv', 'mov', 'webm', 'avi', 'flv', 'mpg', 'mpeg', 'ts', 'm2ts', 'mts', 'ogv', 'ogg'];

const allImageOutputFormats = ['png', 'jpg', 'jpeg', 'bmp', 'webp', 'gif', 'tiff', 'ico', 'ppm', 'pgm', 'pbm'];

const allVideoOutputFormats = ['mp4', 'mkv', 'mov', 'webm', 'avi', 'flv', 'mpg', 'ts', 'ogv', 'gif'];

const formatCompatibility: Record<string, string[]> = {
  png: allImageOutputFormats,
  jpg: allImageOutputFormats,
  jpeg: allImageOutputFormats,
  jfif: allImageOutputFormats,
  bmp: allImageOutputFormats,
  gif: allImageOutputFormats,
  tiff: allImageOutputFormats,
  tif: allImageOutputFormats,
  webp: allImageOutputFormats,
  ico: allImageOutputFormats,
  cur: allImageOutputFormats,
  
  ppm: allImageOutputFormats,
  pgm: allImageOutputFormats,
  pbm: allImageOutputFormats,
  
  avif: allImageOutputFormats,
  heic: allImageOutputFormats,
  heif: allImageOutputFormats,
  tga: allImageOutputFormats,
  dds: allImageOutputFormats,
  apng: allImageOutputFormats,
  exr: allImageOutputFormats,
  
  svg: allImageOutputFormats,
  pdf: allImageOutputFormats,
  psd: allImageOutputFormats,
  psb: allImageOutputFormats,
  fits: allImageOutputFormats,
  dcm: allImageOutputFormats,
  pcx: allImageOutputFormats,

  mp4: allVideoOutputFormats,
  m4v: allVideoOutputFormats,
  mkv: allVideoOutputFormats,
  mov: allVideoOutputFormats,
  webm: allVideoOutputFormats,
  avi: allVideoOutputFormats,
  flv: allVideoOutputFormats,
  mpg: allVideoOutputFormats,
  mpeg: allVideoOutputFormats,
  ts: allVideoOutputFormats,
  m2ts: allVideoOutputFormats,
  mts: allVideoOutputFormats,
  ogv: allVideoOutputFormats,
  ogg: allVideoOutputFormats,
};

const getCompatibleFormats = (format: string): string[] => {
  const compatible = formatCompatibility[format.toLowerCase()] || [];
  return compatible.filter(fmt => fmt.toLowerCase() !== format.toLowerCase());
};

const isSupportedFile = (file: File): boolean => {
  const ext = file.name.split('.').pop()?.toLowerCase() || '';
  return supportedFormats.includes(ext) || file.type.startsWith('image/') || file.type.startsWith('video/') || file.type.includes('pdf') || file.type.includes('svg');
};

const isSupportedFormat = (ext: string): boolean => {
  return supportedFormats.includes(ext.toLowerCase());
};

const filesByType = computed(() => {
  const types: Record<string, any> = {};
  for (const file of droppedFiles.value) {
    if (!types[file.extension]) {
      types[file.extension] = {
        extension: file.extension,
        count: 0,
        targetFormat: file.targetFormat,
      };
    }
    types[file.extension].count++;
  }
  return Object.values(types);
});

const destinationDisplayName = computed(() => {
  if (!destinationFolder.value) return 'Downloads';
  const parts = destinationFolder.value.split(/[/\\]/);
  return parts[parts.length - 1] || 'Downloads';
});

const singleFile = computed(() => droppedFiles.value.length === 1 ? droppedFiles.value[0] : null);
</script>

<template>
  <div class="app-container">
    <NotificationCenter />

    <main class="app-main" @dragover.prevent @dragenter.prevent @dragleave.prevent @drop.prevent="handleDrop">
      <div class="converter-container">
        <div class="converter-left">
          <div v-if="droppedFiles.length === 0" class="empty-settings">
            <p>Add files to get started</p>
          </div>

          <div v-else class="settings-panel">
            <div v-if="singleFile" class="single-conversion">
              <h3 class="settings-title">Convert</h3>
              <div class="conversion-item">
                <div class="from-to">
                  <span class="file-ext">{{ singleFile.extension.toUpperCase() }}</span>
                  <span class="arrow">→</span>
                  <select :value="singleFile.targetFormat" @change="(e) => singleFile && updateFormat(singleFile.extension, (e.target as HTMLSelectElement).value)" class="format-select">
                    <option value="">Select format</option>
                    <option v-for="fmt in getCompatibleFormats(singleFile.extension)" :key="fmt" :value="fmt">{{ fmt.toUpperCase() }}</option>
                  </select>
                </div>
              </div>
            </div>

            <div v-else class="multi-conversion">
              <h3 class="settings-title">Convert</h3>
              <div v-for="fileType in filesByType" :key="fileType.extension" class="conversion-item">
                <div class="from-to">
                  <span class="file-ext">{{ fileType.extension.toUpperCase() }}</span>
                  <span class="arrow">→</span>
                  <select :value="fileType.targetFormat" @change="(e) => updateFormat(fileType.extension, (e.target as HTMLSelectElement).value)" class="format-select">
                    <option value="">Select format</option>
                    <option v-for="fmt in getCompatibleFormats(fileType.extension)" :key="fmt" :value="fmt">{{ fmt.toUpperCase() }}</option>
                  </select>
                </div>
              </div>
            </div>

            <div class="destination-section">
              <label class="dest-label">Destination:</label>
              <div class="dest-input-row">
                <input :value="destinationDisplayName" type="text" class="dest-input" readonly />
                <button type="button" class="browse-folder-btn" @click.prevent="selectOutputFolder">Browse</button>
              </div>
            </div>

            <div class="transparency-section">
              <label class="checkbox-label">
                <input v-model="keepTransparency" type="checkbox" class="checkbox-input" />
                <span>Keep transparency</span>
              </label>
            </div>

            <button 
              class="convert-btn"
              @click="store.startConversion(destinationFolder, keepTransparency)"
              :disabled="isProcessing || droppedFiles.length === 0 || droppedFiles.some(f => !f.targetFormat)"
            >
              {{ isProcessing ? `${currentProgress.percentage}%` : 'Convert Now' }}
            </button>

            <div v-if="isProcessing" class="progress-section">
              <div class="progress-bar">
                <div class="progress-fill" :style="{ width: currentProgress.percentage + '%' }"></div>
              </div>
              <p class="progress-message">{{ currentProgress.message }}</p>
            </div>
          </div>
        </div>

        <div class="converter-right">
          <div class="file-management">
            <div v-if="droppedFiles.length > 0" class="file-list-section">
              <div class="file-count">{{ droppedFiles.length }} file{{ droppedFiles.length !== 1 ? 's' : '' }} selected</div>
              <div class="file-list">
                <div v-for="file in droppedFiles" :key="file.id" class="file-list-item">
                  <div class="list-file-info">
                    <span class="list-file-name">{{ file.name }}</span>
                  </div>
                  <button class="list-remove-btn" @click="removeFile(file.id)">✕</button>
                </div>
              </div>
            </div>

            <div class="drop-zone" @click="pickFiles">
              <div v-if="isImporting" class="loading-overlay">
                <div class="loading-spinner"></div>
                <p>Importing files...</p>
              </div>
              <div class="drop-zone-content">
                <div class="browse-btn" :class="{ disabled: isImporting }">Pick Files</div>
                <p style="font-size: 12px; margin-top: 8px; color: #999;">no, you can't drop your files here :(</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.app-container {
  width: 100%;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: #2a2a2a;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
}

.app-main {
  flex: 1;
  overflow: hidden;
  padding: 1rem;
  display: flex;
  flex-direction: column;
}

.converter-container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  flex: 1;
  height: 100%;
}

.converter-left {
  background: rgba(255, 255, 255, 0.08);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 1rem;
  padding: 1.5rem;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.empty-settings {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: rgba(255, 255, 255, 0.5);
  text-align: center;
}

.settings-panel {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  height: 100%;
}

.settings-title {
  margin: 0;
  color: white;
  font-size: 1.1rem;
  font-weight: 600;
}

.single-conversion,
.multi-conversion {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  max-height: 240px;
  overflow-y: auto;
  padding-right: 0.5rem;
}

.single-conversion::-webkit-scrollbar,
.multi-conversion::-webkit-scrollbar {
  width: 6px;
}

.single-conversion::-webkit-scrollbar-track,
.multi-conversion::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
}

.single-conversion::-webkit-scrollbar-thumb,
.multi-conversion::-webkit-scrollbar-thumb {
  background: rgba(46, 204, 113, 0.5);
  border-radius: 10px;
  transition: background 0.2s ease;
}

.single-conversion::-webkit-scrollbar-thumb:hover,
.multi-conversion::-webkit-scrollbar-thumb:hover {
  background: rgba(46, 204, 113, 0.8);
}

.conversion-item {
  background: rgba(255, 255, 255, 0.05);
  padding: 1rem;
  border-radius: 0.75rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.from-to {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  flex-wrap: wrap;
}

.file-ext {
  color: white;
  font-weight: 600;
  font-size: 1rem;
  min-width: 60px;
  text-align: center;
}

.count {
  color: rgba(255, 255, 255, 0.6);
  font-size: 0.85rem;
}

.arrow {
  color: rgba(255, 255, 255, 0.5);
  font-size: 1.2rem;
}

.format-select {
  background: #3a3a3a;
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 0.5rem;
  padding: 0.6rem 0.8rem;
  padding-right: calc(0.8rem + 3px);
  cursor: pointer;
  font-size: 0.95rem;
  flex: 1;
  min-width: 100px;
}

.format-select::-webkit-scrollbar {
  width: 6px;
}

.format-select::-webkit-scrollbar-track {
  background: transparent;
}

.format-select::-webkit-scrollbar-thumb {
  background: #2ecc71;
  border-radius: 3px;
}

.format-select::-webkit-scrollbar-thumb:hover {
  background: #27ae60;
}

.format-select option {
  background: #2a2a2a;
  color: white;
}

.format-select:hover {
  border-color: rgba(255, 255, 255, 0.4);
  background: #444444;
}

.format-select:focus {
  outline: none;
  border-color: rgba(255, 255, 255, 0.3);
}

.destination-section {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.dest-label {
  color: rgba(255, 255, 255, 0.8);
  font-size: 0.9rem;
  font-weight: 600;
}

.dest-input-row {
  display: flex;
  gap: 0.5rem;
}

.dest-input {
  background: #3a3a3a;
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 0.5rem;
  padding: 0.6rem;
  font-size: 0.9rem;
  flex: 1;
}

.browse-folder-btn {
  padding: 0.6rem 1rem;
  background: rgba(255, 255, 255, 0.2);
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 0.5rem;
  cursor: pointer;
  font-size: 0.9rem;
  font-weight: 600;
  transition: all 0.2s ease;
  white-space: nowrap;
  pointer-events: auto;
  flex-shrink: 0;
}

.browse-folder-btn:hover {
  background: rgba(255, 255, 255, 0.3);
  border-color: rgba(255, 255, 255, 0.4);
}

.browse-folder-btn:active {
  transform: translateY(1px);
}

.transparency-section {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  cursor: pointer;
  user-select: none;
  color: rgba(255, 255, 255, 0.8);
  font-size: 0.9rem;
  font-weight: 500;
}

.checkbox-input {
  width: 18px;
  height: 18px;
  cursor: pointer;
  accent-color: #2ecc71;
  transition: all 0.2s ease;
}

.checkbox-input:hover {
  opacity: 0.8;
  transform: scale(1.1);
}

.checkbox-input:checked {
  animation: checkboxPulse 0.3s ease;
}

@keyframes checkboxPulse {
  0% {
    transform: scale(0.95);
  }
  50% {
    transform: scale(1.15);
  }
  100% {
    transform: scale(1);
  }
}

.converter-right {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.file-management {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  flex: 1;
  overflow: hidden;
}

.drop-zone {
  flex: 1;
  border: 2px dashed rgba(255, 255, 255, 0.3);
  border-radius: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.05);
  cursor: pointer;
  transition: all 0.3s ease;
  backdrop-filter: blur(10px);
  min-height: 140px;
  position: relative;
  pointer-events: auto;
  appearance: none;
  border: none;
  padding: 0;
}

.drop-zone:hover {
  border-color: rgba(255, 255, 255, 0.5);
  background: rgba(255, 255, 255, 0.08);
}

.drop-zone-content {
  text-align: center;
  padding: 1rem;
}

.drop-zone-icon {
  font-size: 2.5rem;
  margin-bottom: 0.75rem;
}

.drop-zone-title {
  margin: 0;
  font-size: 1.2rem;
  color: white;
  font-weight: 600;
}


.browse-btn {
  padding: 0.5rem 1rem;
  background: rgba(255, 255, 255, 0.2);
  color: white;
  border-radius: 0.5rem;
  font-weight: 600;
  transition: all 0.3s ease;
  display: inline-block;
  font-size: 0.85rem;
}

.browse-btn.disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

.drop-zone:hover .browse-btn {
  background: rgba(255, 255, 255, 0.3);
  transform: translateY(-1px);
}

.file-list-section {
  max-height: 200px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 1rem;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  overflow: hidden;
}

.file-list-section::-webkit-scrollbar {
  width: 6px;
}

.file-list-section::-webkit-scrollbar-track {
  background: transparent;
}

.file-list-section::-webkit-scrollbar-thumb {
  background: #2ecc71;
  border-radius: 3px;
}

.file-list-section::-webkit-scrollbar-thumb:hover {
  background: #27ae60;
}

.file-count {
  color: rgba(255, 255, 255, 0.7);
  font-size: 0.85rem;
  font-weight: 600;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.file-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  overflow-y: auto;
  flex: 1;
  padding-right: 3px;
}

.file-list::-webkit-scrollbar {
  width: 6px;
}

.file-list::-webkit-scrollbar-track {
  background: transparent;
}

.file-list::-webkit-scrollbar-thumb {
  background: #2ecc71;
  border-radius: 3px;
}

.file-list::-webkit-scrollbar-thumb:hover {
  background: #27ae60;
}

.file-list-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: rgba(255, 255, 255, 0.08);
  padding: 0.7rem;
  border-radius: 0.5rem;
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: all 0.2s ease;
}

.file-list-item:hover {
  background: rgba(255, 255, 255, 0.12);
  border-color: rgba(255, 255, 255, 0.15);
}

.list-file-info {
  display: flex;
  align-items: center;
  flex: 1;
  overflow: hidden;
}

.list-file-name {
  color: white;
  font-size: 0.85rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.list-remove-btn {
  width: 26px;
  height: 26px;
  padding: 0;
  background: rgba(255, 0, 0, 0.2);
  color: rgba(255, 100, 100, 1);
  border: 1px solid rgba(255, 100, 100, 0.3);
  border-radius: 0.4rem;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1rem;
  flex-shrink: 0;
  margin-left: 0.75rem;
  line-height: 1;
}

.list-remove-btn:hover {
  background: rgba(255, 0, 0, 0.3);
  border-color: rgba(255, 100, 100, 0.5);
}

.convert-btn {
  padding: 0.75rem 1.5rem;
  background: #2ecc71;
  color: white;
  border: none;
  border-radius: 0.5rem;
  font-size: 0.95rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  width: 100%;
  margin-top: -3px;
  -webkit-tap-highlight-color: transparent;
}

.convert-btn:hover:not(:disabled) {
  background: #27ae60;
}

.convert-btn:active:not(:disabled) {
  transform: translateY(1px);
}

.convert-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  background: #95a5a6;
}

.convert-btn:focus,
.convert-btn:focus-visible {
  outline: none;
}

.progress-section {
  margin-top: 1rem;
}

.progress-bar {
  width: 100%;
  height: 6px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: #2ecc71;
  transition: width 0.3s ease;
}

.progress-message {
  margin: 0.5rem 0 0 0;
  font-size: 0.8rem;
  color: rgba(255, 255, 255, 0.7);
  text-align: center;
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  border-radius: 0.5rem;
  z-index: 10;
  pointer-events: none;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid rgba(255, 255, 255, 0.2);
  border-top: 3px solid #2ecc71;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.loading-overlay p {
  color: white;
  font-size: 0.95rem;
  margin: 0;
}
</style>