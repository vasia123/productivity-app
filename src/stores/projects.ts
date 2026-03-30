import { defineStore } from "pinia";
import { ref } from "vue";
import { api } from "@/composables/useTauri";
import type { Project } from "@/types";

export const useProjectsStore = defineStore("projects", () => {
  const projects = ref<Project[]>([]);
  const currentProjectId = ref<string | null>(null);
  const loading = ref(false);

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
    if (currentProjectId.value === id) {
      currentProjectId.value = null;
    }
  }

  async function renameProject(id: string, name: string) {
    const updated = await api.renameProject(id, name);
    const index = projects.value.findIndex((p) => p.id === id);
    if (index !== -1) {
      projects.value[index] = updated;
    }
  }

  async function importDesktops() {
    loading.value = true;
    try {
      projects.value = await api.importDesktops();
    } finally {
      loading.value = false;
    }
  }

  async function switchProject(id: string) {
    await api.switchProject(id);
    currentProjectId.value = id;
  }

  return {
    projects,
    currentProjectId,
    loading,
    fetchProjects,
    importDesktops,
    createProject,
    deleteProject,
    renameProject,
    switchProject,
  };
});
