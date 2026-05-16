import type {RunResponse} from "../analyse/types.ts"

export interface Project {
    id: string;
    name: string;
    repo_url: string;
    branch: string;
    started_at: string;
    finished_at: string;
    runs : RunResponse[];
}

export interface ProjectsResponse {
    projects: Project[];
} 