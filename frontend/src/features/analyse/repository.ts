import { api } from "../../lib/api";
import type { AnalyseRequest, AnalyseResponse } from "./types";

export const analyseRepository = {
    create: async (payload: AnalyseRequest): Promise<AnalyseResponse> => {
        const { data } = await api.post<AnalyseResponse>("/analyse", payload);
        return data;
    },
};
