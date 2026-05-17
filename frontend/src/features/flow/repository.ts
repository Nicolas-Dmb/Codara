import { api } from "../../lib/api";
import type {ModuleGraphResponse} from "./types.ts";

export const projectsRepository = {

    getGraph: async (run_id: string): Promise<ModuleGraphResponse> => {
        const { data } = await api.get<ModuleGraphResponse>(`/graph/${run_id}`);
        return data;
    }

};
