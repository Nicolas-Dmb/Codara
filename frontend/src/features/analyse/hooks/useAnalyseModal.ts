import { useState } from "react";
import useCreateAnalyse from "./useCreateAnalyse";
import type { AnalyseRequest } from "../types";
import {
    validateAnalyseRequest,
    hasValidationErrors,
    type AnalyseValidationErrors,
} from "../validation";
import { useProjects } from "../../project";

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

export default function useAnalyseModal() {
    const [isOpen, setIsOpen] = useState(false);
    const [analyse, setAnalyse] = useState<AnalyseRequest>(initialAnalyse);
    const [validationErrors, setValidationErrors] = useState<AnalyseValidationErrors>({});
    const mutation = useCreateAnalyse();
    const { data: projects } = useProjects();

    const open = () => setIsOpen(true);

    const close = () => {
        setIsOpen(false);
        setAnalyse(initialAnalyse);
        setValidationErrors({});
        mutation.reset();
    };

    const updateField: UpdateAnalyseField = (key, value) => {
        setAnalyse((prev) => ({ ...prev, [key]: value }));
        setValidationErrors((prev) => {
            if (!prev[key]) return prev;
            const next = { ...prev };
            delete next[key];
            return next;
        });
    };

    const isAlreadyAnalysing = () => {
        for (const project of projects ?? []) {
            if (
                project.repo_url.includes(analyse.provider) &&
                project.repo_url.includes(analyse.project_name) &&
                project.repo_url.includes(analyse.namespace)
            ) {
                const run = project.runs.find(
                    (r) =>
                        r.branch === analyse.branch &&
                        (r.status === "pending" || r.status === "processing")
                );
                if (run) return true;
            }
        }
        return false;
    };

    const submit = () => {
        if (isAlreadyAnalysing()) {
            setValidationErrors({ branch: "This branch is already being analysed." });
            return;
        }
        const errors = validateAnalyseRequest(analyse);
        if (hasValidationErrors(errors)) {
            setValidationErrors(errors);
            return;
        }
        setValidationErrors({});
        mutation.mutate(analyse, { onSuccess: close });
    };

    return {
        isOpen,
        open,
        close,
        analyse,
        updateField,
        submit,
        isPending: mutation.isPending,
        error: mutation.error,
        validationErrors,
    };
}