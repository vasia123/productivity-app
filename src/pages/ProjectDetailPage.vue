<script setup lang="ts">
import { onMounted, computed, ref, nextTick } from "vue";
import { useRouter } from "vue-router";
import { useProjectsStore } from "@/stores/projects";
import { useWindowsStore } from "@/stores/windows";
import { api } from "@/composables/useTauri";
import type { WindowInfo } from "@/types";

const props = defineProps<{ id: string }>();
const router = useRouter();
const projectsStore = useProjectsStore();
const windowsStore = useWindowsStore();

const editing = ref(false);
const editName = ref("");
const nameInput = ref<HTMLInputElement | null>(null);
const search = ref("");

const project = computed(() =>
  projectsStore.projects.find((p) => p.id === props.id)
);

function matchesSearch(title: string, exe: string): boolean {
  const q = search.value.toLowerCase().trim();
  if (!q) return true;
  return title.toLowerCase().includes(q) || exe.toLowerCase().includes(q);
}

const filteredProjectWindows = computed(() =>
  windowsStore.projectWindows.filter((w) => matchesSearch(w.window_title, w.exe_name))
);

const assignedHandles = computed(
  () => new Set(windowsStore.projectWindows.map((w) => w.window_handle))
);

const availableWindows = computed(() =>
  windowsStore.openWindows.filter((w) => !assignedHandles.value.has(w.handle) && matchesSearch(w.title, w.exe_name))
);

const allWindows = ref<WindowInfo[]>([]);

interface WindowGroup {
  projectName: string;
  desktopId: string | null;
  windows: WindowInfo[];
}

const groupedOtherWindows = computed(() => {
  const other = allWindows.value.filter(
    (w) => !assignedHandles.value.has(w.handle) && w.desktop_id && matchesSearch(w.title, w.exe_name)
  );

  // Build desktop_guid → project name map
  const desktopToProject: Record<string, string> = {};
  for (const p of projectsStore.projects) {
    if (p.desktop_guid && p.id !== props.id) {
      desktopToProject[p.desktop_guid] = p.name;
    }
  }

  // Group by desktop
  const groups: Record<string, WindowGroup> = {};
  for (const w of other) {
    const key = w.desktop_id || "__unassigned__";
    if (!groups[key]) {
      groups[key] = {
        projectName: (w.desktop_id && desktopToProject[w.desktop_id]) || "Unassigned",
        desktopId: w.desktop_id,
        windows: [],
      };
    }
    groups[key].windows.push(w);
  }

  return Object.values(groups).sort((a, b) => a.projectName.localeCompare(b.projectName));
});

onMounted(async () => {
  if (projectsStore.projects.length === 0) {
    await projectsStore.fetchProjects();
  }
  await windowsStore.fetchProjectWindows(props.id);
  await windowsStore.fetchOpenWindows();
  allWindows.value = await api.listAllWindows();
});

async function assignWindow(handle: number) {
  await windowsStore.assignWindow(props.id, handle);
  await windowsStore.fetchOpenWindows();
}

async function unassignWindow(handle: number) {
  await windowsStore.unassignWindow(handle, props.id);
  await windowsStore.fetchOpenWindows();
}

async function killProcess(handle: number, exeName: string) {
  if (!confirm(`Kill process "${exeName}"?`)) return;
  const { api } = await import("@/composables/useTauri");
  await api.killWindowProcess(handle);
  await windowsStore.fetchProjectWindows(props.id);
  await windowsStore.fetchOpenWindows();
}

async function switchHere() {
  await projectsStore.switchProject(props.id);
}

async function refreshWindows() {
  await windowsStore.fetchOpenWindows();
  await windowsStore.fetchProjectWindows(props.id);
  allWindows.value = await api.listAllWindows();
}

async function reassignWindow(handle: number) {
  await windowsStore.assignWindow(props.id, handle);
  await refreshWindows();
}

function goBack() {
  router.push({ name: "projects" });
}

async function startRename() {
  editName.value = project.value?.name || "";
  editing.value = true;
  await nextTick();
  nameInput.value?.focus();
  nameInput.value?.select();
}

async function confirmRename() {
  const name = editName.value.trim();
  if (name && name !== project.value?.name) {
    await projectsStore.renameProject(props.id, name);
  }
  editing.value = false;
}

function cancelRename() {
  editing.value = false;
}
</script>

<template>
  <div class="page">
    <header class="header">
      <div class="header-left">
        <button class="btn-ghost" @click="goBack">&larr; Back</button>
        <input
          v-if="editing"
          ref="nameInput"
          v-model="editName"
          type="text"
          class="rename-input"
          @keyup.enter="confirmRename"
          @keyup.escape="cancelRename"
          @blur="confirmRename"
        />
        <template v-else>
          <h1>{{ project?.name || "Project" }}</h1>
          <button class="btn-icon" @click="startRename" title="Rename">&#9998;</button>
        </template>
      </div>
      <div class="header-actions">
        <button class="btn-ghost" @click="refreshWindows">Refresh</button>
        <button class="btn-primary" @click="switchHere">
          Switch to Desktop
        </button>
      </div>
    </header>

    <div class="search-bar">
      <input
        v-model="search"
        type="text"
        class="search-input"
        placeholder="Search windows by title or exe..."
      />
    </div>

    <div class="columns">
      <section class="column">
        <h2>Assigned Windows ({{ filteredProjectWindows.length }})</h2>
        <div
          v-if="filteredProjectWindows.length === 0"
          class="empty-column"
        >
          {{ search ? 'No matches' : 'No windows assigned yet' }}
        </div>
        <div
          v-for="win in filteredProjectWindows"
          :key="win.window_handle"
          class="window-item assigned"
        >
          <div class="window-info">
            <span class="window-title">{{ win.window_title || "Untitled" }}</span>
            <span class="window-exe">{{ win.exe_name }}</span>
          </div>
          <div class="window-actions">
            <button class="btn-ghost btn-small" @click="unassignWindow(win.window_handle)">
              Remove
            </button>
            <button class="btn-danger btn-small" @click="killProcess(win.window_handle, win.exe_name)">
              Kill
            </button>
          </div>
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

    <section v-if="groupedOtherWindows.length > 0" class="other-section">
      <h2>Other Windows</h2>
      <div
        v-for="group in groupedOtherWindows"
        :key="group.desktopId || 'none'"
        class="window-group"
      >
        <h3 class="group-title">{{ group.projectName }} ({{ group.windows.length }})</h3>
        <div
          v-for="win in group.windows"
          :key="win.handle"
          class="window-item"
        >
          <div class="window-info">
            <span class="window-title">{{ win.title || "Untitled" }}</span>
            <span class="window-exe">{{ win.exe_name }}</span>
          </div>
          <button class="btn-primary btn-small" @click="reassignWindow(win.handle)">
            Assign
          </button>
        </div>
      </div>
    </section>
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

.btn-icon {
  background: transparent;
  color: var(--text-secondary);
  font-size: 18px;
  padding: 4px 6px;
  line-height: 1;
}

.btn-icon:hover {
  color: var(--accent);
}

.rename-input {
  font-size: 22px;
  font-weight: bold;
  width: 300px;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.search-bar {
  margin-bottom: 16px;
}

.search-input {
  width: 100%;
  padding: 8px 12px;
  font-size: 14px;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: var(--bg-card);
  color: var(--text-primary);
}

.search-input:focus {
  outline: none;
  border-color: var(--accent);
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

.window-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.other-section {
  margin-top: 24px;
}

.other-section h2 {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 12px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.window-group {
  margin-bottom: 16px;
}

.group-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--accent);
  margin-bottom: 8px;
}

.btn-small {
  padding: 4px 12px;
  font-size: 12px;
  flex-shrink: 0;
}
</style>
