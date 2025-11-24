<script setup lang="ts">
import { ref, onMounted } from 'vue';
import type { KiCadInstance } from '../types/pywebview';

const instances = ref<KiCadInstance[]>([]);
const selectedInstance = ref<KiCadInstance | null>(null);
const loading = ref(false);
const error = ref<string>('');

async function waitForAPI(maxAttempts = 20, delayMs = 100): Promise<boolean> {
  for (let i = 0; i < maxAttempts; i++) {
    if (window.pywebview?.api) {
      return true;
    }
    await new Promise(resolve => setTimeout(resolve, delayMs));
  }
  return false;
}

async function detectInstances() {
  loading.value = true;
  error.value = '';
  
  try {
    // Wait for API to be available (up to 2 seconds)
    const apiAvailable = await waitForAPI();
    
    if (!apiAvailable) {
      error.value = 'pywebview API not available';
      return;
    }
    
    const detected = await window.pywebview!.api.detect_kicad_instances();
    instances.value = detected;
    
    // Auto-select if only one instance is found
    if (detected.length === 1) {
      selectedInstance.value = detected[0];
    } else if (detected.length === 0) {
      error.value = 'No KiCAD instances detected. Please make sure KiCAD is running.';
    }
  } catch (err) {
    error.value = `Error detecting KiCAD instances: ${err}`;
    console.error('Error detecting KiCAD instances:', err);
  } finally {
    loading.value = false;
  }
}

function copyError() {
  if (error.value) {
    navigator.clipboard.writeText(error.value);
  }
}

// Auto-detect on component mount
onMounted(() => {
  detectInstances();
});
</script>

<template>
  <div class="kicad-selector">
    <div class="selector-header">
      <h3>KiCAD Connection</h3>
      <button @click="detectInstances" :disabled="loading" class="refresh-btn">
        {{ loading ? 'Detecting...' : '🔄 Refresh' }}
      </button>
    </div>

    <div v-if="error" class="error-message">
      <span class="error-icon">⚠️</span>
      <span class="error-text">{{ error }}</span>
      <button @click="copyError" class="copy-btn" title="Copy error message">Copy</button>
    </div>

    <div v-else-if="loading" class="loading-message">
      <div class="spinner"></div>
      <span>Detecting KiCAD instances...</span>
    </div>

    <div v-else-if="instances.length === 0" class="no-instances">
      <p>No KiCAD instances found.</p>
      <p class="hint">Please start KiCAD and click Refresh.</p>
    </div>

    <div v-else-if="instances.length === 1" class="single-instance">
      <div class="instance-card selected">
        <div class="instance-icon">✓</div>
        <div class="instance-info">
          <div class="instance-name">{{ instances[0].display_name }}</div>
          <div class="instance-project">{{ instances[0].project_name }}</div>
        </div>
      </div>
      <div class="selected-info">
        <div class="info-row" v-if="instances[0].project_path">
          <span class="label">Project:</span>
          <span class="value" :title="instances[0].project_path">{{ instances[0].project_path }}</span>
        </div>
        <div class="info-row" v-if="instances[0].pcb_path">
          <span class="label">PCB:</span>
          <span class="value" :title="instances[0].pcb_path">{{ instances[0].pcb_path }}</span>
        </div>
        <div class="info-row" v-if="instances[0].schematic_path">
          <span class="label">Schematic:</span>
          <span class="value" :title="instances[0].schematic_path">{{ instances[0].schematic_path }}</span>
        </div>
      </div>
      <p class="connection-status">Connected to KiCAD</p>
    </div>

    <div v-else class="multiple-instances">
      <label for="instance-select">Select KiCAD Instance:</label>
      <select 
        id="instance-select" 
        v-model="selectedInstance" 
        class="instance-dropdown"
      >
        <option :value="null" disabled>-- Choose an instance --</option>
        <option 
          v-for="instance in instances" 
          :key="instance.socket_path" 
          :value="instance"
        >
          {{ instance.display_name }}
        </option>
      </select>

      <div v-if="selectedInstance" class="selected-info">
        <div class="info-row" v-if="selectedInstance.project_path">
          <span class="label">Project:</span>
          <span class="value" :title="selectedInstance.project_path">{{ selectedInstance.project_path }}</span>
        </div>
        <div class="info-row" v-if="selectedInstance.pcb_path">
          <span class="label">PCB:</span>
          <span class="value" :title="selectedInstance.pcb_path">{{ selectedInstance.pcb_path }}</span>
        </div>
        <div class="info-row" v-if="selectedInstance.schematic_path">
          <span class="label">Schematic:</span>
          <span class="value" :title="selectedInstance.schematic_path">{{ selectedInstance.schematic_path }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.kicad-selector {
  padding: 1rem;
  background-color: var(--bg-tertiary);
  border-radius: 8px;
  margin-bottom: 1rem;
}

.selector-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.selector-header h3 {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-primary);
}

.refresh-btn {
  padding: 0.5rem 1rem;
  background-color: var(--bg-input);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  color: var(--text-primary);
  transition: background-color 0.2s;
}

.refresh-btn:hover:not(:disabled) {
  background-color: var(--bg-secondary);
}

.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.error-message {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem;
  background-color: #fee;
  border: 1px solid #fcc;
  border-radius: 6px;
  color: #c33;
}

.error-icon {
  font-size: 1.2rem;
  flex-shrink: 0;
}

.error-text {
  flex: 1;
}

.copy-btn {
  padding: 0.25rem 0.5rem;
  background-color: rgba(0, 0, 0, 0.05);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.75rem;
  transition: background-color 0.2s;
  flex-shrink: 0;
}

.copy-btn:hover {
  background-color: rgba(0, 0, 0, 0.1);
}

.copy-btn:active {
  background-color: rgba(0, 0, 0, 0.15);
}

.loading-message {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
  justify-content: center;
  color: var(--text-secondary);
}

.spinner {
  width: 20px;
  height: 20px;
  border: 3px solid var(--border-color);
  border-top-color: #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.no-instances {
  text-align: center;
  padding: 2rem 1rem;
  color: var(--text-secondary);
}

.no-instances p:first-child {
  font-weight: 500;
  margin-bottom: 0.5rem;
}

.hint {
  font-size: 0.9rem;
  opacity: 0.8;
}

.single-instance {
  text-align: center;
}

.instance-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background-color: var(--bg-primary);
  border: 2px solid var(--border-color);
  border-radius: 8px;
  margin-bottom: 0.5rem;
}

.instance-card.selected {
  border-color: #667eea;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
}

.instance-icon {
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  flex-shrink: 0;
}

.instance-info {
  flex: 1;
  text-align: left;
}

.instance-name {
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.25rem;
}

.instance-project {
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.connection-status {
  color: #5a5;
  font-weight: 500;
  margin-top: 0.5rem;
}

.multiple-instances label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: var(--text-primary);
}

.instance-dropdown {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background-color: var(--bg-input);
  color: var(--text-primary);
  font-size: 1rem;
  cursor: pointer;
  transition: border-color 0.2s;
}

.instance-dropdown:focus {
  outline: none;
  border-color: #667eea;
}

.selected-info {
  margin-top: 1rem;
  padding: 0.75rem;
  background-color: var(--bg-primary);
  border-radius: 6px;
  border: 1px solid var(--border-color);
}

.info-row {
  display: flex;
  justify-content: space-between;
  padding: 0.5rem 0;
}

.info-row:not(:last-child) {
  border-bottom: 1px solid var(--border-color);
}

.info-row .label {
  font-weight: 500;
  color: var(--text-secondary);
  min-width: 80px;
  flex-shrink: 0;
}

.info-row .value {
  color: var(--text-primary);
  font-weight: 400;
  word-break: break-all;
  text-align: right;
  font-size: 0.85rem;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
}
</style>
