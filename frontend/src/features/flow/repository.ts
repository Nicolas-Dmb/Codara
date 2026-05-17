import { api } from "../../lib/api";
import type {GraphResponse} from "./types.ts";

export const projectsRepository = {

    getGraph: async (run_id: string): Promise<GraphResponse> => {
        const { data } = await api.get<GraphResponse>(`/graph/${run_id}`);
        return data;
    }

};
