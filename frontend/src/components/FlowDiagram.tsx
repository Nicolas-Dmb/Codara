import { useMemo } from "react";
import type { RunResponse } from "../features/analyse";
import {useFlowDiagram, getLayoutedElements} from "../features/flow";
import { type Node, type Edge, ReactFlow } from "@xyflow/react";
import '@xyflow/react/dist/style.css';

interface FlowDiagramProps {
    selectedAnalysis: RunResponse | null;
}
export default function FlowDiagram({ selectedAnalysis }: FlowDiagramProps) {
    const {data: graphResponse} = useFlowDiagram(selectedAnalysis?.id || "");

    const { nodes, edges } = useMemo(() => {
        if (!graphResponse) {
            return { nodes: [], edges: [] };
        }
        
        const moduleIdOf = (symbolId: string): string => {
            const parts = symbolId.split("::");
            return parts.slice(0, parts.length - 3).join("::");
        };

        const moduleIds = new Set<string>();
        graphResponse.modules.forEach((m) => moduleIds.add(m.id));
        
        const rawNodes: Node[] = graphResponse.modules.map((module) => ({
            id: module.id,
            data: { label: module.name },
            position: { x: 0, y: 0 },
        }));

        const seenEdges = new Set<string>();
        const rawEdges: Edge[] = [];
        for (const relation of graphResponse.relations) {
            if (!relation.target_symbol_id) continue;
            const targetModuleId = moduleIdOf(relation.target_symbol_id);
            if (targetModuleId === relation.module_id) continue;
            const key = `${relation.module_id}->${targetModuleId}`;
            if (seenEdges.has(key)) continue;
            seenEdges.add(key);
            rawEdges.push({
                id: key,
                source: relation.module_id,
                target: targetModuleId,
            });
        }

        return getLayoutedElements(rawNodes, rawEdges);
    }, [graphResponse]);

    return (
        <div style={{ width: '100%', height: '100%' }}>
            <ReactFlow nodes={nodes} edges={edges} fitView />
        </div>
    );
}