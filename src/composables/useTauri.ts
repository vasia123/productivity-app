import { invoke } from "@tauri-apps/api/core";
import type {
  Project,
  WindowInfo,
  WindowAssignment,
  DesktopInfo,
} from "@/types";

export const api = {
  listProjects: () => invoke<Project[]>("list_projects"),

  createProject: (name: string, color?: string) =>
    invoke<Project>("create_project", { name, color }),

  deleteProject: (projectId: string) =>
    invoke<void>("delete_project", { projectId }),

  renameProject: (projectId: string, name: string) =>
    invoke<Project>("rename_project", { projectId, name }),

  switchProject: (projectId: string) =>
    invoke<void>("switch_project", { projectId }),

  listOpenWindows: () => invoke<WindowInfo[]>("list_open_windows"),

  assignWindow: (projectId: string, windowHandle: number) =>
    invoke<void>("assign_window_to_project", { projectId, windowHandle }),

  unassignWindow: (windowHandle: number) =>
    invoke<void>("unassign_window", { windowHandle }),

  getProjectWindows: (projectId: string) =>
    invoke<WindowAssignment[]>("get_project_windows", { projectId }),

  listDesktops: () => invoke<DesktopInfo[]>("list_desktops"),

  getCurrentDesktop: () => invoke<DesktopInfo>("get_current_desktop"),
};
