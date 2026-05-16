import columnArrow from "../assets/images/columnArrow.svg";
import useColumn from "../hooks/useColumn";
import { AnalyseModal, useAnalyseModal, useAnalyseStatus } from "../features/analyse";
import type { UseQueryResult } from "@tanstack/react-query";
import type { AnalyseResponse } from "../features/analyse/types";

export default function LeftColumn() {
    const { isOpen, toggleColumn } = useColumn();

    const analyseModal = useAnalyseModal();
    const { analysisQuery } = useAnalyseStatus(analyseModal.currentRunId);

    return (
        <div
            className={
                "h-screen overflow-hidden bg-white border-r border-gray-200 transition-[width] duration-300 ease-in-out " +
                (isOpen ? "w-1/5" : "w-16")
            }
        >
            {topColumn({ isOpen, toggleColumn })}
            {ProjectsPart({ isOpen })}
            {AnalysisPart({ isOpen, onAddClick: analyseModal.open, analysisQuery })}

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
}

function ProjectsPart({ isOpen }: SubColumnProps) {
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
        </div>
    )
}

interface AnalysisPartProps extends SubColumnProps {
    onAddClick: () => void;
    analysisQuery: UseQueryResult<AnalyseResponse, Error>;
}

function AnalysisPart({ isOpen, onAddClick, analysisQuery }: AnalysisPartProps) {
    const run = analysisQuery.data?.run;
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
                {run && (
                    <div className="flex justify-between p-2" key={run.id}>
                        <p className="font-small">{run.branch}</p>
                        {run.status === "pending" && (
                            <p className="text-yellow-500 font-medium">{run.status}</p>
                        )}
                        {run.status === "running" && (
                            <p className="text-blue-500 font-medium">{run.status}</p>
                        )}
                        {run.status === "done" && (  
                            <p className="text-green-500 font-medium">{run.status}</p>
                        )}{ run.status === "failed" && 
                            <p className="text-red-500 font-medium">{run.status}</p>
                        }
                    </div>
                )}
            </div>
        </div>
    )
}