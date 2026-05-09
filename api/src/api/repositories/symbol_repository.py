from asyncpg import Connection
from fastapi import Depends
from typing import List

from api.src.api.core.db import get_db
from api.src.api.models.run import RunId

from ..models import Symbol


class SymbolRepository:
    def __init__(self, db: Connection):
        self.db = db

    def get_parents_symbols(self, run_id: RunId) -> List[Symbol]:
        """Fetch symbols that are modules as parents"""
        query = "SELECT id, module_id, name, kind, doc, location, start_line, end_line FROM symbol WHERE run_id = $1 AND parent_symbol_id IS NULL"
        rows = self.db.fetch(query, run_id)
        return [Symbol.create(run_id=run_id, row=row) for row in rows]



def get_symbol_repository(db: Connection = Depends(get_db)) -> SymbolRepository:
    return SymbolRepository(db)