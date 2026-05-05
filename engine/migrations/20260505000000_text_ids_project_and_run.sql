-- Align project and analysis_run tables with Rust models.
-- IDs are now application-managed composite TEXT strings
-- (e.g. "namespace::project_name" for project, "project_id::commit" for run)
-- Also adds created_at to analysis_run for polling ORDER BY.

-- Drop in dependency order
DROP TABLE IF EXISTS analysis_run;
DROP TABLE IF EXISTS project;

-- Recreate project with TEXT id
CREATE TABLE project (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    repo_url TEXT NOT NULL,
    branch TEXT NOT NULL DEFAULT 'main',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Recreate analysis_run with TEXT id and TEXT project_id FK
CREATE TABLE analysis_run (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES project(id) ON DELETE CASCADE,
    branch TEXT NOT NULL,
    commit TEXT,
    status TEXT NOT NULL CHECK (status IN ('pending', 'processing', 'done', 'failed', 'partial_success')),
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ
);

CREATE INDEX idx_project_repo_url ON project(repo_url);
CREATE INDEX idx_analysis_run_project_id ON analysis_run(project_id);
CREATE INDEX idx_analysis_run_status ON analysis_run(status);
CREATE INDEX idx_analysis_run_created_at ON analysis_run(created_at);