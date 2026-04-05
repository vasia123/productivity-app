<script setup lang="ts">
import type { Project } from "@/types";

const props = defineProps<{
  project: Project;
  taskCount: number;
  windowCount: number;
}>();

const emit = defineEmits<{
  dragstart: [event: DragEvent];
}>();

const defaultColors = ["#4fc3f7", "#66bb6a", "#ffa726", "#ef5350", "#ab47bc", "#26c6da"];

function getColor(): string {
  return props.project.color || defaultColors[props.project.sort_order % defaultColors.length];
}

function onDragStart(e: DragEvent) {
  e.dataTransfer!.setData("application/x-project-id", props.project.id);
  e.dataTransfer!.effectAllowed = "move";
  emit("dragstart", e);
}
</script>

<template>
  <div class="project-card" draggable="true" @dragstart="onDragStart">
    <div class="card-accent" :style="{ background: getColor() }" />
    <div class="card-body">
      <div class="card-name">{{ project.name }}</div>
      <div class="card-stats">
        <span class="stat" title="Tasks">{{ taskCount }} tasks</span>
        <span class="stat" title="Windows">{{ windowCount }} windows</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.project-card {
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
  cursor: grab;
  transition: border-color 0.15s, transform 0.1s;
}

.project-card:hover {
  border-color: var(--accent);
}

.project-card:active {
  cursor: grabbing;
  transform: scale(0.98);
}

.card-accent {
  height: 3px;
}

.card-body {
  padding: 10px 12px;
}

.card-name {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 4px;
}

.card-stats {
  display: flex;
  gap: 12px;
}

.stat {
  font-size: 11px;
  color: var(--text-secondary);
}
</style>
