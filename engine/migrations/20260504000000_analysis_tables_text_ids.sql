-- Align module, symbol, relation tables with Rust models.
-- IDs are now application-managed composite TEXT strings (e.g. "project_id::relative_path")
-- instead of DB-managed SERIAL integers.

-- Drop old indexes on these tables
DROP INDEX IF EXISTS idx_module_analysis_run;
DROP INDEX IF EXISTS idx_symbol_analysis_run;
DROP INDEX IF EXISTS idx_symbol_module_id;
DROP INDEX IF EXISTS idx_relation_analysis_run;
DROP INDEX IF EXISTS idx_relation_module_id;

-- Drop tables in dependency order (relation → symbol → module)
DROP TABLE IF EXISTS relation;
DROP TABLE IF EXISTS symbol;
DROP TABLE IF EXISTS module;

-- Recreate with TEXT composite IDs matching Rust models

CREATE TABLE module (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    relative_path TEXT NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE symbol (
    id TEXT PRIMARY KEY,
    module_id TEXT NOT NULL REFERENCES module(id) ON DELETE CASCADE,
    run_id TEXT NOT NULL,
    parent_symbol_id TEXT REFERENCES symbol(id),
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('class', 'function', 'method')),
    doc TEXT NOT NULL DEFAULT '',
    location TEXT NOT NULL DEFAULT '',
    start_line INTEGER NOT NULL,
    end_line INTEGER NOT NULL
);

CREATE TABLE relation (
    id TEXT PRIMARY KEY,
    module_id TEXT NOT NULL REFERENCES module(id) ON DELETE CASCADE,
    run_id TEXT NOT NULL,
    parent_symbol_id TEXT REFERENCES symbol(id),
    imported_name TEXT NOT NULL,
    source_path TEXT NOT NULL,
    target_symbol_id TEXT REFERENCES symbol(id),
    kind TEXT NOT NULL CHECK (kind IN ('import')),
    line INTEGER NOT NULL
);

-- Indexes
CREATE INDEX idx_module_run_id ON module(run_id);
CREATE INDEX idx_symbol_module_id ON symbol(module_id);
CREATE INDEX idx_symbol_run_id ON symbol(run_id);
CREATE INDEX idx_symbol_parent ON symbol(parent_symbol_id);
CREATE INDEX idx_relation_module_id ON relation(module_id);
CREATE INDEX idx_relation_run_id ON relation(run_id);
CREATE INDEX idx_relation_target ON relation(target_symbol_id);