import columnArrow from "../assets/images/columnArrow.svg";
import useColumn from "../hooks/useColumn";
import { AnalyseModal, useAnalyseModal, useAnalyseStatus } from "../features/analyse";
import type { RunResponse } from "../features/analyse/types";
import {useProjects} from "../features/project";
import type {Project} from "../features/project";

export default function LeftColumn() {
    const queryProjects = useProjects();
    const analyseModal = useAnalyseModal();
    const { isOpen, toggleColumn, selectedProject, setSelectedProject } = useColumn();
    const { runs } = useAnalyseStatus(selectedProject);
    
    return (
        <div
            className={
                "h-screen overflow-hidden bg-white border-r border-gray-200 transition-[width] duration-300 ease-in-out " +
                (isOpen ? "w-1/5" : "w-16")
            }
        >
            {topColumn({ isOpen, toggleColumn })}
            {ProjectsPart({ isOpen, projects: queryProjects.data, setSelectedProject })}
            {AnalysisPart({ isOpen, onAddClick: analyseModal.open, runs })}

            <AnalyseModal
                isOpen={analyseModal.isOpen}
                onClose={analyseModal.close}
                onSubmit={analyseModal.submit}
                analyse={analyseModal.analyse}
                updateField={analyseModal.updateField}
                isPending={analyseModal.isPending}
                error={analyseModal.error}
                validationErrors={analyseModal.validationErrors}
            />
        </div>
    );
}


interface TopColumnProps {
    isOpen: boolean;
    toggleColumn: () => void;
}
function topColumn({ isOpen, toggleColumn }: TopColumnProps) {
    return (
         <div
                className={
                    "flex items-center px-4 " +
                    (isOpen ? "justify-between" : "justify-center")
                }
            >
                <a
                    href="/"
                    className={
                        "inline-block transition-opacity duration-200 " +
                        (isOpen ? "opacity-100" : "pointer-events-none w-0 opacity-0")
                    }
                >
                    <img src="/icons.svg" alt="Aterminal" className="h-14 w-32" />
                </a>

                <img
                    onClick={toggleColumn}
                    src={columnArrow}
                    alt="Toggle"
                    className={
                        "h-4 w-4 shrink-0 cursor-pointer transition-transform duration-300 ease-in-out hover:text-primary" +
                    (isOpen ? "rotate-0" : "rotate-180")
                }
                />
            </div>
    )
}

interface SubColumnProps {
    isOpen: boolean;
    projects: Project[] | undefined;
    setSelectedProject: (project: Project | null) => void;
}

function ProjectsPart({ isOpen, projects, setSelectedProject }: SubColumnProps) {
    return (
        <div
            className={
                "px-4 py-2 flex flex-col space-y-4 border-t border-gray-200 " +
                (isOpen ? "opacity-100" : "pointer-events-none opacity-0")
            }
        >   
            <div className="flex justify-between items-center">
                <div className="flex justify-between items-center gap-2">
                    <img
                        src={columnArrow}
                        alt="Toggle"
                        className={
                            "h-2 w-2 shrink-0 cursor-pointer transition-transform duration-300 ease-in-out rotate-180"
                        }
                    />
                    <h2 className="font-bold text-sm">Projects</h2>
                </div>
                <button className="text-sm font-bold text-black hover:text-primary">+</button>
            </div>
            {projects && projects.map((project) => (
                <div onClick={() => setSelectedProject(project)} className="flex justify-between align-baseline p-2 cursor-pointer hover:bg-gray-100" key={project.id}>
                    <p className="text-sm">{project.name}</p>
                </div>
            ))}
        </div>
    )
}

interface AnalysisPartProps {
    isOpen: boolean;
    onAddClick: () => void;
    runs: RunResponse[];
}

const statusColor: Record<string, string> = {
    pending: "text-yellow-500",
    processing: "text-blue-500",
    done: "text-green-500",
    failed: "text-red-500",
    partial_success: "text-orange-500",
};

function AnalysisPart({ isOpen, onAddClick, runs }: AnalysisPartProps) {
    return (
        <div
            className={
                "px-4 py-2 flex flex-col space-y-4 border-t border-gray-200 " +
                (isOpen ? "opacity-100" : "pointer-events-none opacity-0")
            }
        >
            <div className="flex justify-between items-center">
                <div className="flex justify-between items-center gap-2">
                    <img
                        src={columnArrow}
                        alt="Toggle"
                        className={
                            "h-2 w-2 shrink-0 cursor-pointer transition-transform duration-300 ease-in-out rotate-180"
                        }
                    />
                    <h2 className="font-bold text-sm">Analyses</h2>
                </div>
                <button
                    onClick={onAddClick}
                    className="text-sm font-bold text-black hover:text-primary cursor-pointer"
                >
                    +
                </button>
            </div>
            <div className="flex flex-col gap-2">
                {runs.map((run) => (
                    <div className="flex justify-between p-2 cursor-pointer hover:bg-gray-100" key={run.id}>
                        <p className="text-xs">{run.branch}</p>
                        <p className={`text-xs font-medium ${statusColor[run.status] ?? "text-gray-500"}`}>
                            {run.status}
                        </p>
                    </div>
                ))}
            </div>
        </div>
    )
}