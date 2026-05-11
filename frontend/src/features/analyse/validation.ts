import type { AnalyseRequest } from "./types";

export type AnalyseValidationErrors = Partial<Record<keyof AnalyseRequest, string>>;

const NAMESPACE_RE = /^[a-zA-Z0-9._/-]+$/;
const PROJECT_RE = /^[a-zA-Z0-9._-]+$/;
const INVALID_BRANCH_RE = /[\s~^:?*[\\]/;

export function validateAnalyseRequest(req: AnalyseRequest): AnalyseValidationErrors {
    const errors: AnalyseValidationErrors = {};

    if (!req.namespace.trim()) {
        errors.namespace = "Namespace requis";
    } else if (!NAMESPACE_RE.test(req.namespace)) {
        errors.namespace = "Caractères autorisés : lettres, chiffres, . _ - /";
    }

    if (!req.project_name.trim()) {
        errors.project_name = "Nom du projet requis";
    } else if (!PROJECT_RE.test(req.project_name)) {
        errors.project_name = "Caractères autorisés : lettres, chiffres, . _ -";
    }

    if (!req.branch.trim()) {
        errors.branch = "Branche requise";
    } else if (INVALID_BRANCH_RE.test(req.branch)) {
        errors.branch = "Nom de branche invalide";
    }

    return errors;
}

export function hasValidationErrors(errors: AnalyseValidationErrors): boolean {
    return Object.keys(errors).length > 0;
}
