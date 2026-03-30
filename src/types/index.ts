export interface Project {
  id: string;
  name: string;
  desktop_guid: string | null;
  desktop_name: string | null;
  color: string | null;
  created_at: string;
  updated_at: string;
}

export interface WindowAssignment {
  project_id: string;
  window_handle: number;
  window_title: string;
  exe_name: string;
  assigned_at: string;
}

export interface WindowInfo {
  handle: number;
  title: string;
  exe_name: string;
  desktop_id: string | null;
  is_visible: boolean;
}

export interface DesktopInfo {
  guid: string;
  name: string;
  index: number;
  is_current: boolean;
}
