from asyncpg import Connection
from fastapi import Depends
from typing import List

from ..core import get_db

from ..models import Symbol, RunId


class SymbolRepository:
    def __init__(self, db: Connection):
        self.db = db

    async def get_root_symbols(self, run_id: RunId) -> List[Symbol]:
        """Fetch symbols that are modules as parents"""
        query = "SELECT id, parent_symbol_id, module_id, name, kind, doc, location, start_line, end_line FROM symbol WHERE run_id = $1 AND parent_symbol_id IS NULL"
        rows = await self.db.fetch(query, run_id)
        return [Symbol.create(run_id=run_id, row=row) for row in rows]



def get_symbol_repository(db: Connection = Depends(get_db)) -> SymbolRepository:
    return SymbolRepository(db)