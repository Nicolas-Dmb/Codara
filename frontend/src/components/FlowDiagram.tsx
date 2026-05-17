import type { RunResponse } from "../features/analyse";
import {useFlowDiagram} from "../features/flow";

interface FlowDiagramProps {
    selectedAnalysis: RunResponse | null;
}
export default function FlowDiagram({ selectedAnalysis }: FlowDiagramProps) {
    const {data: graphResponse} = useFlowDiagram(selectedAnalysis?.id || "");
    console.log(graphResponse);
    return (
        <div className="h-full w-full">
            
        </div>
    );
}