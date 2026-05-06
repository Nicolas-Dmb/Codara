from ..core import get_db
from asyncpg import Connection
from fastapi import Depends
from ..models import Run, RunId, Status
from typing import Tuple, Optional


class RunRepository:

    def __init__(self, db: Connection):
        self.db = db

    async def is_already_register(self, run_id: RunId) -> bool:
        query = "SELECT EXISTS(SELECT 1 FROM analysis_run WHERE id = $1)"
        return bool(await self.db.fetchval(query, run_id))

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
    
    async def get_status(self, run_id: RunId) -> Tuple[Status, Optional[str]] | None:
        query = "SELECT status, error_message FROM analysis_run WHERE id = $1"
        row = await self.db.fetchrow(query, run_id)
        if row is None:
            return None
        try:
            status = Status(row['status'])
        except ValueError:
            return None
        return status, row['error_message']


def get_run_repository(db: Connection = Depends(get_db))-> RunRepository:
    return RunRepository(db)