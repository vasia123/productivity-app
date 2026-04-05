<script setup lang="ts">
import type { Task } from "@/types";

const props = defineProps<{
  task: Task;
  draggable?: boolean;
}>();

const emit = defineEmits<{
  delete: [];
}>();

function onDragStart(e: DragEvent) {
  e.dataTransfer!.setData("application/x-task-id", props.task.id);
  e.dataTransfer!.effectAllowed = "move";
}
</script>

<template>
  <div
    class="task-card"
    :class="{ done: task.status === 'done' }"
    :draggable="draggable !== false"
    @dragstart="onDragStart"
  >
    <span class="task-title">{{ task.title }}</span>
    <button class="task-delete" @click="emit('delete')" title="Delete">&times;</button>
  </div>
</template>

<style scoped>
.task-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  cursor: grab;
  transition: border-color 0.15s;
}

.task-card:hover {
  border-color: var(--accent);
}

.task-card:active {
  cursor: grabbing;
}

.task-card.done {
  opacity: 0.6;
}

.task-card.done .task-title {
  text-decoration: line-through;
}

.task-title {
  font-size: 13px;
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.task-delete {
  background: transparent;
  border: none;
  color: var(--text-secondary);
  font-size: 16px;
  padding: 0 4px;
  cursor: pointer;
  flex-shrink: 0;
}

.task-delete:hover {
  color: var(--danger);
}
</style>
