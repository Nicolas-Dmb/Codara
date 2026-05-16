import { useMutation, useQueryClient } from "@tanstack/react-query";
import { analyseRepository } from "../repository";
import type { AnalyseRequest } from "../types";

export default function useCreateAnalyse() {
    const queryClient = useQueryClient();
    return useMutation({
        mutationFn: (payload: AnalyseRequest) => analyseRepository.create(payload),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ["projects"] });
        },
    });
}
