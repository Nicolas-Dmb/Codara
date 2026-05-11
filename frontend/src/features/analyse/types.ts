export type Provider = "github" | "gitlab" | "bitbucket";

export interface AnalyseRequest {
    provider: Provider;
    namespace: string;
    project_name: string;
    branch: string;
}

export type AnalyseStatus = "PENDING" | "RUNNING" | "SUCCESS" | "FAILED";

export interface RunResponse {
    id: string;
    project_id: string;
    branch: string;
    commit: string;
    status: AnalyseStatus;
    error_message: string | null;
    started_at: string | null;
    finished_at: string | null;
}

export interface AnalyseResponse {
    message: string;
    run: RunResponse;
}
