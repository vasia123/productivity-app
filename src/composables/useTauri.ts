import { invoke } from "@tauri-apps/api/core";
import type {
  Project,
  Task,
  WindowInfo,
  WindowAssignment,
  OtherWindowInfo,
  DesktopInfo,
} from "@/types";

export const api = {
  // Projects
  listProjects: () => invoke<Project[]>("list_projects"),
  createProject: (name: string, color?: string) =>
    invoke<Project>("create_project", { name, color }),
  deleteProject: (projectId: string) =>
    invoke<void>("delete_project", { projectId }),
  renameProject: (projectId: string, name: string) =>
    invoke<Project>("rename_project", { projectId, name }),
  switchProject: (projectId: string) =>
    invoke<void>("switch_project", { projectId }),
  importDesktops: () => invoke<Project[]>("import_desktops"),

  // Board
  setProjectBoardStatus: (projectId: string, boardStatus: string) =>
    invoke<Project>("set_project_board_status", { projectId, boardStatus }),
  reorderProjects: (projectIds: string[]) =>
    invoke<void>("reorder_projects", { projectIds }),

  // Tasks
  createTask: (projectId: string, title: string) =>
    invoke<Task>("create_task", { projectId, title }),
  deleteTask: (taskId: string) => invoke<void>("delete_task", { taskId }),
  updateTaskStatus: (taskId: string, status: string) =>
    invoke<Task>("update_task_status", { taskId, status }),
  listTasks: (projectId: string) =>
    invoke<Task[]>("list_tasks", { projectId }),

  // Windows
  listOpenWindows: () => invoke<WindowInfo[]>("list_open_windows"),
  assignWindow: (projectId: string, windowHandle: number) =>
    invoke<void>("assign_window_to_project", { projectId, windowHandle }),
  unassignWindow: (windowHandle: number) =>
    invoke<void>("unassign_window", { windowHandle }),
  getProjectWindows: (projectId: string) =>
    invoke<WindowAssignment[]>("get_project_windows", { projectId }),
  killWindowProcess: (windowHandle: number) =>
    invoke<void>("kill_window_process", { windowHandle }),
  getOtherProjectWindows: (excludeProjectId: string) =>
    invoke<OtherWindowInfo[]>("get_other_project_windows", { excludeProjectId }),
  listAllWindows: () => invoke<WindowInfo[]>("list_all_windows"),

  // Desktops
  listDesktops: () => invoke<DesktopInfo[]>("list_desktops"),
  getCurrentDesktop: () => invoke<DesktopInfo>("get_current_desktop"),
};
