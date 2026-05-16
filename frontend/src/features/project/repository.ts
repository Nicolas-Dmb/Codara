import { api } from "../../lib/api";
import type {ProjectsResponse} from "./types.ts";

export const projectsRepository = {

    getProjects: async (): Promise<ProjectsResponse> => {
        const { data } = await api.get<ProjectsResponse>("/projects");
        return data;
    }

};
