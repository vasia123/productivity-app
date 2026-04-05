import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { api } from "@/composables/useTauri";
import type { Project } from "@/types";

export const useProjectsStore = defineStore("projects", () => {
  const projects = ref<Project[]>([]);
  const loading = ref(false);

  const todoProjects = computed(() =>
    projects.value
      .filter((p) => p.board_status === "todo")
      .sort((a, b) => a.sort_order - b.sort_order)
  );

  const inProgressProject = computed(() =>
    projects.value.find((p) => p.board_status === "in_progress") ?? null
  );

  async function fetchProjects() {
    loading.value = true;
    try {
      projects.value = await api.listProjects();
    } finally {
      loading.value = false;
    }
  }

  async function createProject(name: string, color?: string) {
    const project = await api.createProject(name, color);
    projects.value.push(project);
    return project;
  }

  async function deleteProject(id: string) {
    await api.deleteProject(id);
    projects.value = projects.value.filter((p) => p.id !== id);
  }

  async function renameProject(id: string, name: string) {
    const updated = await api.renameProject(id, name);
    const index = projects.value.findIndex((p) => p.id === id);
    if (index !== -1) {
      projects.value[index] = updated;
    }
  }

  async function switchProject(id: string) {
    await api.switchProject(id);
  }

  async function importDesktops() {
    loading.value = true;
    try {
      projects.value = await api.importDesktops();
    } finally {
      loading.value = false;
    }
  }

  async function setProjectBoardStatus(id: string, boardStatus: string) {
    await api.setProjectBoardStatus(id, boardStatus);
    await fetchProjects();
  }

  async function reorderProjects(projectIds: string[]) {
    await api.reorderProjects(projectIds);
    await fetchProjects();
  }

  return {
    projects,
    loading,
    todoProjects,
    inProgressProject,
    fetchProjects,
    createProject,
    deleteProject,
    renameProject,
    switchProject,
    importDesktops,
    setProjectBoardStatus,
    reorderProjects,
  };
});
