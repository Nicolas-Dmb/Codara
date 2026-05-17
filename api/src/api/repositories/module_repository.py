

from asyncpg import Connection
from fastapi import Depends

from ..core import get_db

from ..models import RunId, Module


class ModuleRepository:
    def __init__(self, db:Connection):
        self.db = db

    async def get_modules(self, run_id: RunId):
        query = "SELECT id, relative_path, name FROM module WHERE run_id = $1"
        rows = await self.db.fetch(query, run_id)
        return [Module.from_db_row(row, run_id) for row in rows]
    

def get_module_repository(db: Connection = Depends(get_db)) -> ModuleRepository:
    return ModuleRepository(db)