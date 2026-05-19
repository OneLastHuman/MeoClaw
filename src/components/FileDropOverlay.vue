<script setup lang="ts">
import { computed } from 'vue';

interface DroppedFile {
  name: string;
  icon: string;
}

const props = defineProps<{
  files: DroppedFile[];
}>();

const emit = defineEmits<{
  remove: [index: number];
}>();

const MAX_FILES = 3;

const displayFiles = computed(() => props.files.slice(0, MAX_FILES));

function truncateName(name: string, maxLen = 10): string {
  if (name.length <= maxLen) return name;
  const ext = name.includes('.') ? '.' + name.split('.').pop() : '';
  const base = name.slice(0, name.length - ext.length);
  const truncated = base.slice(0, maxLen - 3 - ext.length);
  return truncated + '...' + ext;
}

function onRemove(index: number) {
  emit('remove', index);
}
</script>

<template>
  <div class="file-drop-overlay">
    <div
      v-for="(file, idx) in displayFiles"
      :key="idx"
      class="file-item"
    >
      <div class="icon-wrapper">
        <img :src="`/ui/${file.icon}.png`" :alt="file.icon" class="file-icon" />
        <button class="close-btn" @click="onRemove(idx)">×</button>
      </div>
      <span class="file-name">{{ truncateName(file.name) }}</span>
    </div>
  </div>
</template>

<style scoped>
.file-drop-overlay {
  position: fixed;
  top: var(--file-drop-top, 70px);
  left: 50%;
  transform: translateX(-50%);
  z-index: 15;
  display: flex;
  gap: var(--file-drop-gap, 8px);
  align-items: center;
  pointer-events: none;
}

.file-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  pointer-events: auto;
}

.icon-wrapper {
  position: relative;
  display: inline-block;
}

.file-icon {
  max-width: var(--file-icon-size, 40px);
  max-height: var(--file-icon-size, 40px);
  image-rendering: pixelated;
  display: block;
}

.close-btn {
  position: absolute;
  top: var(--close-button-offset, -4px);
  right: var(--close-button-offset, -4px);
  width: var(--close-button-size, 16px);
  height: var(--close-button-size, 16px);
  border-radius: 50%;
  background: rgba(180, 180, 180, 0.85);
  border: none;
  color: #fff;
  font-size: calc(var(--close-button-size, 16px) * 0.75);
  line-height: 1;
  cursor: pointer;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-btn:active {
  background: rgba(150, 150, 150, 0.9);
}

.file-name {
  font-size: var(--file-name-font-size, 10px);
  font-family: sans-serif;
  color: #555;
  max-width: var(--file-name-max-width, 56px);
  text-align: center;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
