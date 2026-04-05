<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { useProjectsStore } from "@/stores/projects";
import { api } from "@/composables/useTauri";
import type { Task } from "@/types";

const projectsStore = useProjectsStore();
const projectTasks = ref<Record<string, Task[]>>({});
const sortedIds = ref<string[]>([]);
const draggedId = ref<string | null>(null);

onMounted(async () => {
  if (projectsStore.projects.length === 0) {
    await projectsStore.fetchProjects();
  }
  refreshOrder();
  await loadAllTasks();
});

watch(() => projectsStore.projects, () => refreshOrder(), { deep: true });

function refreshOrder() {
  sortedIds.value = [...projectsStore.projects]
    .sort((a, b) => a.sort_order - b.sort_order)
    .map((p) => p.id);
}

async function loadAllTasks() {
  for (const p of projectsStore.projects) {
    projectTasks.value[p.id] = await api.listTasks(p.id);
  }
}

// Drag & drop
function onDragStart(e: DragEvent, id: string) {
  draggedId.value = id;
  e.dataTransfer!.setData("text/plain", id);
  e.dataTransfer!.effectAllowed = "move";
}

function onDragOver(_e: DragEvent, targetIndex: number) {
  if (!draggedId.value) return;
  const currentIndex = sortedIds.value.indexOf(draggedId.value);
  if (currentIndex === -1 || currentIndex === targetIndex) return;

  const ids = [...sortedIds.value];
  ids.splice(currentIndex, 1);
  ids.splice(targetIndex, 0, draggedId.value);
  sortedIds.value = ids;
}

function onDragEnd() {
  if (draggedId.value) {
    projectsStore.reorderProjects([...sortedIds.value]);
    draggedId.value = null;
  }
}

function getProject(id: string) {
  return projectsStore.projects.find((p) => p.id === id);
}

const defaultColors = ["#4fc3f7", "#66bb6a", "#ffa726", "#ef5350", "#ab47bc", "#26c6da"];
function getColor(id: string, index: number): string {
  return getProject(id)?.color || defaultColors[index % defaultColors.length];
}
</script>

<template>
  <div class="prioritization">
    <p class="hint">Drag columns to set priority. Leftmost = highest.</p>
    <div class="columns-row">
      <div
        v-for="(id, index) in sortedIds"
        :key="id"
        class="prio-column"
        :class="{ dragging: draggedId === id }"
        draggable="true"
        @dragstart="onDragStart($event, id)"
        @dragover.prevent="onDragOver($event, index)"
        @drop.prevent="onDragEnd"
        @dragend="onDragEnd"
      >
        <div class="prio-header" :style="{ borderBottomColor: getColor(id, index) }">
          <span class="prio-rank">#{{ index + 1 }}</span>
          {{ getProject(id)?.name }}
        </div>
        <div class="prio-tasks">
          <div
            v-for="task in (projectTasks[id] || [])"
            :key="task.id"
            class="prio-task"
            :class="{ done: task.status === 'done' }"
          >
            {{ task.title }}
          </div>
          <div v-if="!(projectTasks[id] || []).length" class="prio-empty">
            No tasks
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.hint {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 16px;
}

.columns-row {
  display: flex;
  gap: 12px;
  overflow-x: auto;
  padding-bottom: 8px;
}

.prio-column {
  min-width: 180px;
  max-width: 240px;
  flex-shrink: 0;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  cursor: grab;
  transition: border-color 0.15s, opacity 0.15s;
}

.prio-column:hover {
  border-color: var(--accent);
}

.prio-column:active {
  cursor: grabbing;
}

.prio-column.dragging {
  opacity: 0.4;
}

.prio-header {
  padding: 10px 12px;
  font-size: 14px;
  font-weight: 600;
  border-bottom: 3px solid var(--border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.prio-rank {
  color: var(--text-secondary);
  margin-right: 6px;
  font-size: 12px;
}

.prio-tasks {
  padding: 8px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.prio-task {
  font-size: 12px;
  padding: 4px 8px;
  background: var(--bg);
  border-radius: 4px;
}

.prio-task.done {
  opacity: 0.5;
  text-decoration: line-through;
}

.prio-empty {
  font-size: 12px;
  color: var(--text-secondary);
  padding: 8px;
  text-align: center;
}
</style>
