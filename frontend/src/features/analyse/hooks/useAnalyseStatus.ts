import { useQuery } from "@tanstack/react-query";
import { analyseRepository } from "../repository";

export default function useAnalyseStatus(runId: string| undefined) {

    const analysisQuery = useQuery({
        queryKey: ["runId",  runId],

        queryFn: async () =>  await analyseRepository.getState(runId!),

        enabled: !!runId,

        refetchInterval: (query) => {
            const status = query.state.data?.run.status;

            if (
            status === "done" ||
            status === "failed" || 
            status === "partial_success"
            ) {
            return false;
            }

            return 60_000;
        },
    });

    return { analysisQuery };
}
