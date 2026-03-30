<script setup lang="ts">
import { onMounted, computed } from "vue";
import { useRouter } from "vue-router";
import { useProjectsStore } from "@/stores/projects";
import { useWindowsStore } from "@/stores/windows";

const props = defineProps<{ id: string }>();
const router = useRouter();
const projectsStore = useProjectsStore();
const windowsStore = useWindowsStore();

const project = computed(() =>
  projectsStore.projects.find((p) => p.id === props.id)
);

const assignedHandles = computed(
  () => new Set(windowsStore.projectWindows.map((w) => w.window_handle))
);

const availableWindows = computed(() =>
  windowsStore.openWindows.filter((w) => !assignedHandles.value.has(w.handle))
);

onMounted(async () => {
  if (projectsStore.projects.length === 0) {
    await projectsStore.fetchProjects();
  }
  await windowsStore.fetchProjectWindows(props.id);
  await windowsStore.fetchOpenWindows();
});

async function assignWindow(handle: number) {
  await windowsStore.assignWindow(props.id, handle);
  await windowsStore.fetchOpenWindows();
}

async function unassignWindow(handle: number) {
  await windowsStore.unassignWindow(handle, props.id);
  await windowsStore.fetchOpenWindows();
}

async function switchHere() {
  await projectsStore.switchProject(props.id);
}

function refreshWindows() {
  windowsStore.fetchOpenWindows();
  windowsStore.fetchProjectWindows(props.id);
}

function goBack() {
  router.push({ name: "projects" });
}
</script>

<template>
  <div class="page">
    <header class="header">
      <div class="header-left">
        <button class="btn-ghost" @click="goBack">&larr; Back</button>
        <h1>{{ project?.name || "Project" }}</h1>
      </div>
      <div class="header-actions">
        <button class="btn-ghost" @click="refreshWindows">Refresh</button>
        <button class="btn-primary" @click="switchHere">
          Switch to Desktop
        </button>
      </div>
    </header>

    <div class="columns">
      <section class="column">
        <h2>Assigned Windows ({{ windowsStore.projectWindows.length }})</h2>
        <div
          v-if="windowsStore.projectWindows.length === 0"
          class="empty-column"
        >
          No windows assigned yet
        </div>
        <div
          v-for="win in windowsStore.projectWindows"
          :key="win.window_handle"
          class="window-item assigned"
        >
          <div class="window-info">
            <span class="window-title">{{ win.window_title || "Untitled" }}</span>
            <span class="window-exe">{{ win.exe_name }}</span>
          </div>
          <button class="btn-ghost btn-small" @click="unassignWindow(win.window_handle)">
            Remove
          </button>
        </div>
      </section>

      <section class="column">
        <h2>Available Windows ({{ availableWindows.length }})</h2>
        <div v-if="availableWindows.length === 0" class="empty-column">
          No other windows found
        </div>
        <div
          v-for="win in availableWindows"
          :key="win.handle"
          class="window-item"
        >
          <div class="window-info">
            <span class="window-title">{{ win.title || "Untitled" }}</span>
            <span class="window-exe">{{ win.exe_name }}</span>
          </div>
          <button class="btn-primary btn-small" @click="assignWindow(win.handle)">
            Assign
          </button>
        </div>
      </section>
    </div>
  </div>
</template>

<style scoped>
.page {
  padding: 24px;
  max-width: 1100px;
  margin: 0 auto;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.header-left h1 {
  font-size: 22px;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.columns {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
}

.column h2 {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 12px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.empty-column {
  text-align: center;
  padding: 32px;
  color: var(--text-secondary);
  border: 1px dashed var(--border);
  border-radius: 8px;
}

.window-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 14px;
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 8px;
  margin-bottom: 8px;
  transition: border-color 0.15s;
}

.window-item:hover {
  border-color: var(--accent);
}

.window-item.assigned {
  border-left: 3px solid var(--accent);
}

.window-info {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.window-title {
  font-size: 14px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 300px;
}

.window-exe {
  font-size: 12px;
  color: var(--text-secondary);
}

.btn-small {
  padding: 4px 12px;
  font-size: 12px;
  flex-shrink: 0;
}
</style>
