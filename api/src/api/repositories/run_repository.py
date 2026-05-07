from ..core import get_db
from asyncpg import Connection
from fastapi import Depends
from ..models import Run, RunId


class RunRepository:

    def __init__(self, db: Connection):
        self.db = db

    async def save(self, run: Run):
        query = """
        INSERT INTO analysis_run (id, project_id, branch, commit, status, error_message)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (id) DO NOTHING
        """
        await self.db.execute(
            query,
            run.id,
            run.project_id,
            run.branch,
            run.commit,
            run.status.value,
            run.error_message,
        )
    
    async def get_run(self, run_id: RunId) -> Run | None:
        query = "SELECT project_id, branch, commit, status, error_message, started_at, finished_at FROM analysis_run WHERE id = $1"
        row = await self.db.fetchrow(query, run_id)
        if row is None:
            return None
        return Run.from_db_row(run_id, row)
        

def get_run_repository(db: Connection = Depends(get_db))-> RunRepository:
    return RunRepository(db)