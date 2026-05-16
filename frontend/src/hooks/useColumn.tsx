import { useState } from "react";
import type {Project} from "../features/project";

export default function useColumn() {
    const [isOpen, setIsOpen] = useState(true);
    const [selectedProject, setSelectedProject] = useState<Project | null>(null);

    const toggleColumn = () => {
        setIsOpen(!isOpen);
    };

    return { isOpen, toggleColumn, selectedProject, setSelectedProject };
}
