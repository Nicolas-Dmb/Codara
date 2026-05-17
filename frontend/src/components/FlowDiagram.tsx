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

        const moduleLabel = (moduleId: string): string => {
            const idx = moduleId.indexOf("::");
            return idx === -1 ? moduleId : moduleId.slice(idx + 2);
        };

        const moduleIds = new Set<string>();
        graphResponse.symbols.forEach((s) => moduleIds.add(s.module_id));
        graphResponse.relations.forEach((r) => moduleIds.add(r.module_id));

        const rawNodes: Node[] = Array.from(moduleIds).map((mid) => ({
            id: mid,
            data: { label: moduleLabel(mid) },
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