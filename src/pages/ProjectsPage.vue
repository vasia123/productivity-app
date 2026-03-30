<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useProjectsStore } from "@/stores/projects";

const store = useProjectsStore();
const router = useRouter();
const newProjectName = ref("");
const showNewForm = ref(false);

onMounted(() => {
  store.fetchProjects();
});

async function createProject() {
  const name = newProjectName.value.trim();
  if (!name) return;
  await store.createProject(name);
  newProjectName.value = "";
  showNewForm.value = false;
}

async function switchProject(id: string) {
  await store.switchProject(id);
}

function openProject(id: string) {
  router.push({ name: "project-detail", params: { id } });
}

async function importDesktops() {
  await store.importDesktops();
}

async function deleteProject(id: string) {
  await store.deleteProject(id);
}

const defaultColors = ["#4fc3f7", "#66bb6a", "#ffa726", "#ef5350", "#ab47bc", "#26c6da"];

function getColor(index: number, color: string | null): string {
  return color || defaultColors[index % defaultColors.length];
}
</script>

<template>
  <div class="page">
    <header class="header">
      <h1>Projects</h1>
      <div class="header-actions">
        <button class="btn-ghost" @click="importDesktops">Import Desktops</button>
        <button class="btn-primary" @click="showNewForm = !showNewForm">
          {{ showNewForm ? "Cancel" : "+ New Project" }}
        </button>
      </div>
    </header>

    <div v-if="showNewForm" class="new-project-form">
      <input
        v-model="newProjectName"
        type="text"
        placeholder="Project name..."
        @keyup.enter="createProject"
        autofocus
      />
      <button class="btn-primary" @click="createProject">Create</button>
    </div>

    <div v-if="store.loading" class="loading">Loading...</div>

    <div v-else-if="store.projects.length === 0" class="empty">
      <p>No projects yet. Create one to get started!</p>
    </div>

    <div v-else class="project-grid">
      <div
        v-for="(project, index) in store.projects"
        :key="project.id"
        class="project-card"
        @click="openProject(project.id)"
      >
        <div
          class="card-accent"
          :style="{ background: getColor(index, project.color) }"
        />
        <div class="card-body">
          <h3>{{ project.name }}</h3>
          <p class="card-meta">
            {{ project.desktop_name || "No desktop" }}
          </p>
          <div class="card-actions" @click.stop>
            <button
              class="btn-primary btn-small"
              @click="switchProject(project.id)"
              :class="{ active: store.currentProjectId === project.id }"
            >
              {{ store.currentProjectId === project.id ? "Active" : "Switch" }}
            </button>
            <button
              class="btn-danger btn-small"
              @click="deleteProject(project.id)"
            >
              Delete
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page {
  padding: 24px;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.header h1 {
  font-size: 24px;
  font-weight: 600;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.new-project-form {
  display: flex;
  gap: 12px;
  margin-bottom: 24px;
}

.new-project-form input {
  flex: 1;
}

.loading,
.empty {
  text-align: center;
  padding: 48px;
  color: var(--text-secondary);
}

.project-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
}

.project-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: 10px;
  overflow: hidden;
  cursor: pointer;
  transition: border-color 0.15s, transform 0.1s;
}

.project-card:hover {
  border-color: var(--accent);
  transform: translateY(-2px);
}

.card-accent {
  height: 4px;
}

.card-body {
  padding: 12px;
}

.card-body h3 {
  font-size: 16px;
  margin-bottom: 4px;
}

.card-meta {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 12px;
}

.card-actions {
  display: flex;
  gap: 8px;
}

.btn-small {
  padding: 4px 12px;
  font-size: 12px;
}

.btn-primary.active {
  background: var(--success);
}
</style>
