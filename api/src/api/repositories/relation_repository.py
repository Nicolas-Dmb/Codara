

from typing import List

from asyncpg import Connection
from fastapi import Depends

from ..core import get_db

from ..models import RunId, Relation



class RelationRepository:
    
    def __init__(self, db: Connection):
        self.db = db

    async def get_module_relations(self, run_id: RunId) -> List[Relation]:
        query = "SELECT id, module_id, parent_symbol_id, imported_name, source_path, target_symbol_id, kind, line FROM relation WHERE run_id = $1 AND parent_symbol_id IS NULL"
        rows = await self.db.fetch(query, run_id)
        return [Relation.create(run_id=run_id, row=row) for row in rows]


def get_relation_repository(db: Connection = Depends(get_db)) -> RelationRepository:
    return RelationRepository(db)