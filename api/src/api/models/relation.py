from dataclasses import dataclass
from typing import Optional, Dict, Any

from .symbol import SymbolId
from .run import RunId
from .module import ModuleId

from enum import Enum


class RelationKind(Enum): 
    Import = "import"

    def __str__(self):
        return self.value

class RelationId(str):

    def __new__(cls, module_id: ModuleId, kind: RelationKind, imported_name: str, source_path: str, line: int):
        return super().__new__(cls, f"{module_id}:{kind}:{imported_name}:{source_path}:{line}")
    
    @classmethod
    def from_db_row(cls, symbol_id: str) -> "RelationId":
        module_id, kind, imported_name, source_path, line = symbol_id.rsplit(":", 4)
        return cls(ModuleId.from_db_row(module_id), RelationKind(kind), imported_name, source_path, int(line))


@dataclass
class Relation:
    id: RelationId
    run_id: RunId
    module_id: ModuleId
    parent_symbol_id: Optional[SymbolId]
    imported_name: str
    source_path: str
    target_symbol_id: Optional[SymbolId]
    kind: RelationKind
    line: int


    @classmethod
    def create(cls,run_id: RunId, row: Dict[str, Any]) -> "Relation":
        return cls(
            id=RelationId.from_db_row(symbol_id=row['id']),
            run_id=run_id,
            module_id=ModuleId.from_db_row(module_id=row['module_id']),
            parent_symbol_id=SymbolId.from_db_row(symbol_id=row['parent_symbol_id']) if row['parent_symbol_id'] else None,
            imported_name=row['imported_name'],
            source_path=row['source_path'],
            target_symbol_id=SymbolId.from_db_row(symbol_id=row['target_symbol_id']) if row['target_symbol_id'] else None,
            kind=RelationKind(row['kind']),
            line=row['line']
        )