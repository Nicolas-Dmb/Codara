import { useQuery } from "@tanstack/react-query";
import { projectsRepository } from "../repository";

export default function useProjects() {
    return useQuery({
        queryKey: ["projects"],
        queryFn: () => projectsRepository.getProjects(),
        select: (data) => data.projects,
    });
}