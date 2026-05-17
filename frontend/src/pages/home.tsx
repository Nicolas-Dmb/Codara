import { useState } from "react";
import LeftColumn from "../components/leftColumn";
import type { RunResponse } from "../features/analyse";
import FlowDiagram from "../components/flowDiagram";

export default function Home() {
  const [selectedAnalysis, setSelectedAnalysis] = useState<RunResponse | null>(null);
  return (
    <div className="min-h-screen">
        <LeftColumn setSelectedAnalysis={setSelectedAnalysis} />
        <FlowDiagram selectedAnalysis={selectedAnalysis} />
    </div>
  );
}
