-- Align analysis_run.status CHECK constraint with Rust RunStatus enum values
ALTER TABLE analysis_run
    DROP CONSTRAINT IF EXISTS analysis_run_status_check;

ALTER TABLE analysis_run
    ADD CONSTRAINT analysis_run_status_check
    CHECK (status IN ('pending', 'processing', 'done', 'failed', 'partial_success'));