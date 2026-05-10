import { useState } from "react";

export type Provider = "github" | "gitlab" | "bitbucket";

export interface AnalyseRequest {
    provider: Provider;
    namespace: string;
    project_name: string;
    branch: string;
}

export type UpdateAnalyseField = <K extends keyof AnalyseRequest>(
    key: K,
    value: AnalyseRequest[K]
) => void;

const initialAnalyse: AnalyseRequest = {
    provider: "github",
    namespace: "",
    project_name: "",
    branch: "",
};

interface UseAnalyseModalOptions {
    onSubmit?: (analyse: AnalyseRequest) => void;
}

export default function useAnalyseModal({ onSubmit }: UseAnalyseModalOptions = {}) {
    const [isOpen, setIsOpen] = useState(false);
    const [analyse, setAnalyse] = useState<AnalyseRequest>(initialAnalyse);

    const open = () => setIsOpen(true);

    const close = () => {
        setIsOpen(false);
        setAnalyse(initialAnalyse);
    };

    const updateField: UpdateAnalyseField = (key, value) => {
        setAnalyse((prev) => ({ ...prev, [key]: value }));
    };

    const submit = () => {
        onSubmit?.(analyse);
        close();
    };

    return { isOpen, open, close, analyse, updateField, submit };
}
