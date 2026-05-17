import { useState } from "react";
import LeftColumn from "../components/leftColumn";
import type { RunResponse } from "../features/analyse";
import FlowDiagram from "../components/flowDiagram";

export default function Home() {
  const [selectedAnalysis, setSelectedAnalysis] = useState<RunResponse | null>(null);
  return (
    <div className="flex h-screen">
        <LeftColumn setSelectedAnalysis={setSelectedAnalysis} />
        <div className="flex-1 h-full">
            <FlowDiagram selectedAnalysis={selectedAnalysis} />
        </div>
    </div>
  );
}
