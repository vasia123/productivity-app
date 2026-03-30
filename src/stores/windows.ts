import { defineStore } from "pinia";
import { ref } from "vue";
import { api } from "@/composables/useTauri";
import type { WindowInfo, WindowAssignment } from "@/types";

export const useWindowsStore = defineStore("windows", () => {
  const openWindows = ref<WindowInfo[]>([]);
  const projectWindows = ref<WindowAssignment[]>([]);

  async function fetchOpenWindows() {
    openWindows.value = await api.listOpenWindows();
  }

  async function fetchProjectWindows(projectId: string) {
    projectWindows.value = await api.getProjectWindows(projectId);
  }

  async function assignWindow(projectId: string, windowHandle: number) {
    await api.assignWindow(projectId, windowHandle);
    await fetchProjectWindows(projectId);
  }

  async function unassignWindow(windowHandle: number, projectId: string) {
    await api.unassignWindow(windowHandle);
    await fetchProjectWindows(projectId);
  }

  return {
    openWindows,
    projectWindows,
    fetchOpenWindows,
    fetchProjectWindows,
    assignWindow,
    unassignWindow,
  };
});
