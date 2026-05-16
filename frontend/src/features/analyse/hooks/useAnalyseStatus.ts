import { useQueries, useQueryClient } from "@tanstack/react-query";
import { analyseRepository } from "../repository";
import type { Project } from "../../project";
import type { AnalyseResponse, AnalyseStatus, RunResponse } from "../types";

const ACTIVE: AnalyseStatus[] = ["pending", "running"];
const isActive = (s: AnalyseStatus) => ACTIVE.includes(s);

export default function useAnalyseStatus(project: Project | null) {
    const queryClient = useQueryClient();
    const activeRuns = (project?.runs ?? []).filter((r) => isActive(r.status));

    const queries = useQueries({
        queries: activeRuns.map((run) => ({
            queryKey: ["run", run.id] as const,
            queryFn: () => analyseRepository.getState(run.id),
            initialData: { message: "", run } as AnalyseResponse,
            refetchInterval: (q: { state: { data?: AnalyseResponse } }) => {
                const s = q.state.data?.run.status;
                if (!s) return 60_000;
                if (!isActive(s)) {
                    queryClient.invalidateQueries({ queryKey: ["projects"] });
                    return false;
                }
                return 60_000;
            },
        })),
    });

    const polledById = new Map<string, RunResponse>(
        queries.map((q, i) => [activeRuns[i].id, q.data?.run ?? activeRuns[i]])
    );
    const runs = (project?.runs ?? []).map((r) => polledById.get(r.id) ?? r);

    return { runs, queries };
}
