-- Align database schema with Rust models (pre-MVP)

-- project: rename default_branch -> branch
ALTER TABLE project RENAME COLUMN default_branch TO branch;

-- analysis_run: rename columns to match Run model
ALTER TABLE analysis_run RENAME COLUMN analyzed_branch TO branch;
ALTER TABLE analysis_run RENAME COLUMN analyzed_commit TO commit;

-- analysis_run: align status values (processing -> running, done -> success)
ALTER TABLE analysis_run DROP CONSTRAINT analysis_run_status_check;
UPDATE analysis_run SET status = 'running' WHERE status = 'processing';
UPDATE analysis_run SET status = 'success' WHERE status = 'done';
ALTER TABLE analysis_run ADD CONSTRAINT analysis_run_status_check
    CHECK (status IN ('pending', 'running', 'success', 'failed'));

-- analysis_run: started_at should not default to CURRENT_TIMESTAMP (set on start(), not on creation)
ALTER TABLE analysis_run ALTER COLUMN started_at DROP DEFAULT;

-- module: rename path -> relative_path
ALTER TABLE module RENAME COLUMN path TO relative_path;

-- symbol: align kind values with SymbolKind enum (class, function, method)
ALTER TABLE symbol DROP CONSTRAINT symbol_kind_check;
UPDATE symbol SET kind = 'method' WHERE kind = 'method';
ALTER TABLE symbol ADD CONSTRAINT symbol_kind_check
    CHECK (kind IN ('class', 'function', 'method'));

-- relation: add line column
ALTER TABLE relation ADD COLUMN line INTEGER;

-- relation: kind stays 'import' only (already the case in init schema)
