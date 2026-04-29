-- Add migration script here

-- analysis_warning: non-blocking errors collected during analysis
CREATE TABLE analysis_warning (
    id SERIAL PRIMARY KEY,
    analysis_run_id INTEGER NOT NULL REFERENCES analysis_run(id) ON DELETE CASCADE,
    scope TEXT NOT NULL CHECK (scope IN ('module', 'symbol', 'relation')),
    module_path TEXT NOT NULL,
    symbol_name TEXT,
    error_kind TEXT NOT NULL,
    message TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_warning_analysis_run ON analysis_warning(analysis_run_id);