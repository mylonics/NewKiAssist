<script setup lang="ts">
import ChatBox from './components/ChatBox.vue';
import KiCadInstanceSelector from './components/KiCadInstanceSelector.vue';
</script>

<template>
  <div class="app-container">
    <aside class="left-panel">
      <KiCadInstanceSelector />
    </aside>
    <main class="chat-panel">
      <ChatBox />
    </main>
  </div>
</template>

<style>
/* Default color variables (light mode) */
:root {
  --bg-primary: #f5f5f5;
  --bg-secondary: #e8e8e8;
  --bg-tertiary: #dcdcdc;
  --bg-input: #ffffff;
  --text-primary: #2d2d2d;
  --text-secondary: #666666;
  --border-color: #c0c0c0;
}

/* Dark mode support */
@media (prefers-color-scheme: dark) {
  :root {
    --bg-primary: #1e1e1e;
    --bg-secondary: #252525;
    --bg-tertiary: #2d2d2d;
    --bg-input: #1e1e1e;
    --text-primary: #e4e4e4;
    --text-secondary: #a0a0a0;
    --border-color: #404040;
  }
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  height: 100%;
  overflow: hidden;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  background-color: var(--bg-secondary);
}

#app {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

.app-container {
  display: flex;
  flex-direction: row;
  height: 100%;
  width: 100%;
}

.left-panel {
  width: 280px;
  min-width: 280px;
  max-width: 280px;
  height: 100%;
  background-color: var(--bg-primary);
  border-right: 1px solid var(--border-color);
  overflow-y: auto;
  flex-shrink: 0;
}

.chat-panel {
  flex: 1;
  height: 100%;
  min-width: 0;
  display: flex;
  flex-direction: column;
}
</style>