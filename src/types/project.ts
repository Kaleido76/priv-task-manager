export interface Project {
  id: number;
  name: string;
  description: string;
  color: string | null;
  sort_order: number;
  create_time: string;
  update_time: string | null;
}
