import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { api } from "@/composables/useTauri";
import type { Task } from "@/types";

export const useTasksStore = defineStore("tasks", () => {
  const tasks = ref<Task[]>([]);

  const todoTasks = computed(() =>
    tasks.value.filter((t) => t.status === "todo")
  );

  const doneTasks = computed(() =>
    tasks.value.filter((t) => t.status === "done")
  );

  async function fetchTasks(projectId: string) {
    tasks.value = await api.listTasks(projectId);
  }

  async function createTask(projectId: string, title: string) {
    const task = await api.createTask(projectId, title);
    tasks.value.push(task);
    return task;
  }

  async function deleteTask(taskId: string) {
    await api.deleteTask(taskId);
    tasks.value = tasks.value.filter((t) => t.id !== taskId);
  }

  async function updateTaskStatus(taskId: string, status: string) {
    const updated = await api.updateTaskStatus(taskId, status);
    const index = tasks.value.findIndex((t) => t.id === taskId);
    if (index !== -1) {
      tasks.value[index] = updated;
    }
  }

  function clear() {
    tasks.value = [];
  }

  return {
    tasks,
    todoTasks,
    doneTasks,
    fetchTasks,
    createTask,
    deleteTask,
    updateTaskStatus,
    clear,
  };
});
