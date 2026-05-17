import { useQuery } from "@tanstack/react-query";
import { projectsRepository } from "../repository";


export default function useFlowDiagram(runId: string|null) {
    return useQuery({
        enabled: !!runId,
        queryKey: ["flowDiagram", runId],
        queryFn: () => projectsRepository.getGraph(runId!),
        select: (data) => data,
    });
}