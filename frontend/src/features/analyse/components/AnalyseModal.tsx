import Modal from "../../../components/modal";
import type { UpdateAnalyseField } from "../hooks/useAnalyseModal";
import type { AnalyseRequest } from "../types";
import type { AnalyseValidationErrors } from "../validation";

interface AnalyseModalProps {
    isOpen: boolean;
    onClose: () => void;
    onSubmit: () => void;
    analyse: AnalyseRequest;
    updateField: UpdateAnalyseField;
    isPending: boolean;
    error: Error | null;
    validationErrors: AnalyseValidationErrors;
}

const fieldClass = (hasError: boolean) =>
    "border rounded px-3 py-2 focus:outline-none " +
    (hasError ? "border-red-500 focus:border-red-500" : "border-gray-300 focus:border-primary");

export default function AnalyseModal({
    isOpen,
    onClose,
    onSubmit,
    analyse,
    updateField,
    isPending,
    error,
    validationErrors,
}: AnalyseModalProps) {
    return (
        <Modal isOpen={isOpen} onClose={onClose} title="Nouvelle analyse">
            <form
                onSubmit={(e) => {
                    e.preventDefault();
                    onSubmit();
                }}
                noValidate
                className="flex flex-col space-y-4"
            >
                <label className="flex flex-col space-y-1 text-sm">
                    <span className="font-bold">Provider</span>
                    <select
                        value={analyse.provider}
                        onChange={(e) =>
                            updateField("provider", e.target.value as AnalyseRequest["provider"])
                        }
                        className={fieldClass(false) + " bg-white"}
                        disabled={isPending}
                    >
                        <option value="github">GitHub</option>
                        <option value="gitlab">GitLab</option>
                        <option value="bitbucket">Bitbucket</option>
                    </select>
                </label>

                <label className="flex flex-col space-y-1 text-sm">
                    <span className="font-bold">Namespace</span>
                    <input
                        type="text"
                        value={analyse.namespace}
                        onChange={(e) => updateField("namespace", e.target.value)}
                        className={fieldClass(!!validationErrors.namespace)}
                        placeholder="organisation ou utilisateur"
                        autoFocus
                        disabled={isPending}
                    />
                    {validationErrors.namespace && (
                        <span className="text-red-600 text-xs">{validationErrors.namespace}</span>
                    )}
                </label>

                <label className="flex flex-col space-y-1 text-sm">
                    <span className="font-bold">Nom du projet</span>
                    <input
                        type="text"
                        value={analyse.project_name}
                        onChange={(e) => updateField("project_name", e.target.value)}
                        className={fieldClass(!!validationErrors.project_name)}
                        placeholder="nom-du-repo"
                        disabled={isPending}
                    />
                    {validationErrors.project_name && (
                        <span className="text-red-600 text-xs">{validationErrors.project_name}</span>
                    )}
                </label>

                <label className="flex flex-col space-y-1 text-sm">
                    <span className="font-bold">Branche</span>
                    <input
                        type="text"
                        value={analyse.branch}
                        onChange={(e) => updateField("branch", e.target.value)}
                        className={fieldClass(!!validationErrors.branch)}
                        placeholder="main"
                        disabled={isPending}
                    />
                    {validationErrors.branch && (
                        <span className="text-red-600 text-xs">{validationErrors.branch}</span>
                    )}
                </label>

                {error && (
                    <p className="text-sm text-red-600">
                        {error.message ?? "Erreur lors de la création de l'analyse."}
                    </p>
                )}

                <div className="flex justify-end gap-2 pt-2">
                    <button
                        type="button"
                        onClick={onClose}
                        disabled={isPending}
                        className="px-4 py-2 text-sm rounded border border-gray-300 hover:bg-gray-50 disabled:opacity-50"
                    >
                        Annuler
                    </button>
                    <button
                        type="submit"
                        disabled={isPending}
                        className="px-4 py-2 text-sm rounded bg-primary text-white hover:opacity-90 disabled:opacity-50"
                    >
                        {isPending ? "Création..." : "Créer"}
                    </button>
                </div>
            </form>
        </Modal>
    );
}
