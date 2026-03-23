CREATE TABLE project (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    repo_url TEXT NOT NULL,
    default_branch TEXT DEFAULT 'main',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE analysis_run (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL REFERENCES project(id) ON DELETE CASCADE,
    analyzed_branch TEXT NOT NULL,
    analyzed_commit TEXT,
    status TEXT NOT NULL CHECK (status IN ('pending', 'processing', 'done', 'failed')),
    error_message TEXT,
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    finished_at TIMESTAMP
);

CREATE TABLE module (
    id SERIAL PRIMARY KEY,
    analysis_run_id INTEGER NOT NULL REFERENCES analysis_run(id) ON DELETE CASCADE,
    path TEXT NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE symbol (
    id SERIAL PRIMARY KEY,
    analysis_run_id INTEGER NOT NULL REFERENCES analysis_run(id) ON DELETE CASCADE,
    module_id INTEGER NOT NULL REFERENCES module(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('class', 'function', 'method')),
    doc TEXT,
    location TEXT,
    parent_symbol_id INTEGER REFERENCES symbol(id),
    start_line INTEGER,
    end_line INTEGER
);

CREATE TABLE relation (
    id SERIAL PRIMARY KEY,
    analysis_run_id INTEGER NOT NULL REFERENCES analysis_run(id) ON DELETE CASCADE,
    module_id INTEGER NOT NULL REFERENCES module(id) ON DELETE CASCADE,
    imported_name TEXT NOT NULL,
    source_path TEXT NOT NULL,
    target_symbol_id INTEGER REFERENCES symbol(id),
    kind TEXT NOT NULL CHECK (kind IN ('import'))
);

CREATE INDEX idx_project_repo_url ON project(repo_url);
CREATE INDEX idx_analysis_project_id ON analysis_run(project_id);
CREATE INDEX idx_analysis_status ON analysis_run(status);
CREATE INDEX idx_module_analysis_run ON module(analysis_run_id);
CREATE INDEX idx_symbol_analysis_run ON symbol(analysis_run_id);
CREATE INDEX idx_symbol_module_id ON symbol(module_id);
CREATE INDEX idx_relation_analysis_run ON relation(analysis_run_id);
CREATE INDEX idx_relation_module_id ON relation(module_id);