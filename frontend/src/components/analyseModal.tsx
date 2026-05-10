import type { AnalyseRequest, UpdateAnalyseField } from "../hooks/useAnalyseModal";
import Modal from "./modal";

interface AnalyseModalProps {
    isOpen: boolean;
    onClose: () => void;
    onSubmit: () => void;
    analyse: AnalyseRequest;
    updateField: UpdateAnalyseField;
}

export default function AnalyseModal({
    isOpen,
    onClose,
    onSubmit,
    analyse,
    updateField,
}: AnalyseModalProps) {
    return (
        <Modal isOpen={isOpen} onClose={onClose} title="Nouvelle analyse">
            <form
                onSubmit={(e) => {
                    e.preventDefault();
                    onSubmit();
                }}
                className="flex flex-col space-y-4"
            >
                <label className="flex flex-col space-y-1 text-sm">
                    <span className="font-bold">Provider</span>
                    <select
                        value={analyse.provider}
                        onChange={(e) =>
                            updateField("provider", e.target.value as AnalyseRequest["provider"])
                        }
                        className="border border-gray-300 rounded px-3 py-2 focus:outline-none focus:border-primary bg-white"
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
                        className="border border-gray-300 rounded px-3 py-2 focus:outline-none focus:border-primary"
                        placeholder="organisation ou utilisateur"
                        required
                        autoFocus
                    />
                </label>

                <label className="flex flex-col space-y-1 text-sm">
                    <span className="font-bold">Nom du projet</span>
                    <input
                        type="text"
                        value={analyse.project_name}
                        onChange={(e) => updateField("project_name", e.target.value)}
                        className="border border-gray-300 rounded px-3 py-2 focus:outline-none focus:border-primary"
                        placeholder="nom-du-repo"
                        required
                    />
                </label>

                <label className="flex flex-col space-y-1 text-sm">
                    <span className="font-bold">Branche</span>
                    <input
                        type="text"
                        value={analyse.branch}
                        onChange={(e) => updateField("branch", e.target.value)}
                        className="border border-gray-300 rounded px-3 py-2 focus:outline-none focus:border-primary"
                        placeholder="main"
                        required
                    />
                </label>

                <div className="flex justify-end gap-2 pt-2">
                    <button
                        type="button"
                        onClick={onClose}
                        className="px-4 py-2 text-sm rounded border border-gray-300 hover:bg-gray-50"
                    >
                        Annuler
                    </button>
                    <button
                        type="submit"
                        className="px-4 py-2 text-sm rounded bg-primary text-white hover:opacity-90"
                    >
                        Créer
                    </button>
                </div>
            </form>
        </Modal>
    );
}
