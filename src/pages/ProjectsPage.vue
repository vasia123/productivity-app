<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import { useRouter } from "vue-router";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useProjectsStore } from "@/stores/projects";
import { useTasksStore } from "@/stores/tasks";
import KanbanColumn from "@/components/KanbanColumn.vue";
import ProjectCard from "@/components/ProjectCard.vue";
import TaskCard from "@/components/TaskCard.vue";
import TaskInput from "@/components/TaskInput.vue";
import PrioritizationView from "@/components/PrioritizationView.vue";

const router = useRouter();
const projectsStore = useProjectsStore();
const tasksStore = useTasksStore();
const viewMode = ref<"kanban" | "prioritize">("kanban");
const showNewForm = ref(false);
const newProjectName = ref("");

// Task/window counts per project (loaded once)
const taskCounts = ref<Record<string, number>>({});
const windowCounts = ref<Record<string, number>>({});

let unlistenDesktop: UnlistenFn | null = null;

onMounted(async () => {
  await projectsStore.fetchProjects();
  await loadCounts();
  if (projectsStore.inProgressProject) {
    await tasksStore.fetchTasks(projectsStore.inProgressProject.id);
  }

  // Listen for external desktop switches (Win+Ctrl+Arrow, etc.)
  unlistenDesktop = await listen("desktop-changed", async () => {
    await projectsStore.fetchProjects();
    await loadCounts();
  });
});

onUnmounted(() => {
  unlistenDesktop?.();
});

watch(
  () => projectsStore.inProgressProject?.id,
  async (id) => {
    if (id) {
      await tasksStore.fetchTasks(id);
    } else {
      tasksStore.clear();
    }
  }
);

async function loadCounts() {
  for (const p of projectsStore.projects) {
    const tasks = await import("@/composables/useTauri").then((m) =>
      m.api.listTasks(p.id)
    );
    taskCounts.value[p.id] = tasks.length;
    const wins = await import("@/composables/useTauri").then((m) =>
      m.api.getProjectWindows(p.id)
    );
    windowCounts.value[p.id] = wins.length;
  }
}

// Project creation
async function createProject() {
  const name = newProjectName.value.trim();
  if (!name) return;
  await projectsStore.createProject(name);
  newProjectName.value = "";
  showNewForm.value = false;
  await loadCounts();
}

// Navigate to project detail (window management)
function openProject(id: string) {
  router.push({ name: "project-detail", params: { id } });
}

// Drag & Drop: project → in_progress
async function onDropInProgress(e: DragEvent) {
  const projectId = e.dataTransfer!.getData("application/x-project-id");
  if (projectId) {
    await projectsStore.setProjectBoardStatus(projectId, "in_progress");
    await loadCounts();
  }
}

// Drag & Drop: project → todo
async function activateProject(id: string) {
  await projectsStore.setProjectBoardStatus(id, "in_progress");
  await loadCounts();
}

async function onDropTodo(e: DragEvent) {
  const projectId = e.dataTransfer!.getData("application/x-project-id");
  if (projectId) {
    await projectsStore.setProjectBoardStatus(projectId, "todo");
  }
}

// Drag & Drop: task → done
async function onDropDone(e: DragEvent) {
  const taskId = e.dataTransfer!.getData("application/x-task-id");
  if (taskId) {
    await tasksStore.updateTaskStatus(taskId, "done");
  }
}

// Drag & Drop: task → back to todo (from done)
async function onDropInProgressTasks(e: DragEvent) {
  const taskId = e.dataTransfer!.getData("application/x-task-id");
  if (taskId) {
    await tasksStore.updateTaskStatus(taskId, "todo");
    return;
  }
  // Also handle project drop
  onDropInProgress(e);
}

// Task creation
async function onCreateTask(title: string) {
  if (!projectsStore.inProgressProject) return;
  await tasksStore.createTask(projectsStore.inProgressProject.id, title);
  taskCounts.value[projectsStore.inProgressProject.id] =
    (taskCounts.value[projectsStore.inProgressProject.id] || 0) + 1;
}

// Task deletion
async function onDeleteTask(taskId: string) {
  await tasksStore.deleteTask(taskId);
}

// Active project color
function activeColor(): string | undefined {
  return projectsStore.inProgressProject?.color || undefined;
}
</script>

<template>
  <div class="page">
    <header class="header">
      <h1>Board</h1>
      <div class="header-actions">
        <button
          class="btn-ghost"
          @click="viewMode = viewMode === 'kanban' ? 'prioritize' : 'kanban'"
        >
          {{ viewMode === "kanban" ? "Prioritize" : "Board" }}
        </button>
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

    <!-- Kanban View -->
    <div v-if="viewMode === 'kanban'" class="kanban-board">
      <KanbanColumn title="TODO" @drop="onDropTodo">
        <ProjectCard
          v-for="p in projectsStore.todoProjects"
          :key="p.id"
          :project="p"
          :task-count="taskCounts[p.id] || 0"
          :window-count="windowCounts[p.id] || 0"
          @click="activateProject(p.id)"
        />
        <div v-if="projectsStore.todoProjects.length === 0" class="empty-hint">
          No projects in queue
        </div>
      </KanbanColumn>

      <KanbanColumn
        title="In Progress"
        :color="activeColor()"
        @drop="onDropInProgressTasks"
      >
        <template v-if="projectsStore.inProgressProject">
          <div
            class="active-header"
            :style="{ color: activeColor() }"
            @dblclick="openProject(projectsStore.inProgressProject!.id)"
          >
            {{ projectsStore.inProgressProject.name }}
          </div>
          <TaskInput @create="onCreateTask" />
          <TaskCard
            v-for="t in tasksStore.todoTasks"
            :key="t.id"
            :task="t"
            @delete="onDeleteTask(t.id)"
          />
          <div v-if="tasksStore.todoTasks.length === 0" class="empty-hint">
            No active tasks
          </div>
        </template>
        <div v-else class="empty-hint">
          Drag a project here to start working
        </div>
      </KanbanColumn>

      <KanbanColumn title="Done" @drop="onDropDone">
        <template v-if="projectsStore.inProgressProject">
          <TaskCard
            v-for="t in tasksStore.doneTasks"
            :key="t.id"
            :task="t"
            :draggable="true"
            @delete="onDeleteTask(t.id)"
          />
          <div v-if="tasksStore.doneTasks.length === 0" class="empty-hint">
            Completed tasks appear here
          </div>
        </template>
        <div v-else class="empty-hint">
          &mdash;
        </div>
      </KanbanColumn>
    </div>

    <!-- Prioritization View -->
    <PrioritizationView v-else />
  </div>
</template>

<style scoped>
.page {
  padding: 24px;
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  flex-shrink: 0;
}

.header h1 {
  font-size: 22px;
  font-weight: 600;
}

.header-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}


.new-project-form {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
  flex-shrink: 0;
}

.new-project-form input {
  flex: 1;
}

.kanban-board {
  display: grid;
  grid-template-columns: 1fr 1.5fr 1fr;
  gap: 16px;
  flex: 1;
  min-height: 0;
}

.active-header {
  font-size: 16px;
  font-weight: 700;
  margin-bottom: 8px;
  cursor: pointer;
}

.active-header:hover {
  text-decoration: underline;
}

.empty-hint {
  text-align: center;
  padding: 24px 12px;
  color: var(--text-secondary);
  font-size: 13px;
  border: 1px dashed var(--border);
  border-radius: 6px;
}
</style>
