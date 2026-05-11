import { useMutation } from "@tanstack/react-query";
import { analyseRepository } from "../repository";
import type { AnalyseRequest } from "../types";

export default function useCreateAnalyse() {
    return useMutation({
        mutationFn: (payload: AnalyseRequest) => analyseRepository.create(payload),
    });
}
