<script setup lang="ts">
import { ref } from "vue";

defineProps<{
  title: string;
  color?: string | null;
}>();

const emit = defineEmits<{
  drop: [event: DragEvent];
}>();

const dragOver = ref(false);

function onDragEnter() {
  dragOver.value = true;
}
function onDragLeave(e: DragEvent) {
  const target = e.currentTarget as HTMLElement;
  if (!target.contains(e.relatedTarget as Node)) {
    dragOver.value = false;
  }
}
function onDrop(e: DragEvent) {
  dragOver.value = false;
  emit("drop", e);
}
</script>

<template>
  <div
    class="column"
    :class="{ 'drag-over': dragOver }"
    :style="color ? { borderTopColor: color } : {}"
    @dragover.prevent
    @dragenter="onDragEnter"
    @dragleave="onDragLeave"
    @drop="onDrop"
  >
    <h2 class="column-title" :style="color ? { color } : {}">{{ title }}</h2>
    <div class="column-content">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.column {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-top: 3px solid var(--border);
  border-radius: 10px;
  padding: 16px;
  min-height: 200px;
  display: flex;
  flex-direction: column;
  transition: border-color 0.15s, background 0.15s;
}

.column.drag-over {
  border-color: var(--accent);
  background: var(--bg-hover);
}

.column-title {
  font-size: 13px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 1px;
  color: var(--text-secondary);
  margin-bottom: 12px;
}

.column-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
}
</style>
