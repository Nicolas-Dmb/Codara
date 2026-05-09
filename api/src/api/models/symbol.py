from dataclasses import dataclass
from typing import Optional, Dict, Any

from .run import RunId
from enum import Enum
from .module import ModuleId

class SymbolKind(Enum): 
    Class = "class"
    Function = "function"
    Method = "method"



class SymbolId(str):
    "module_id: &ModuleId, kind: &SymbolKind, name: &str, start_line: usize"

    def __new__(cls, module_id: ModuleId, kind: SymbolKind, name: str, start_line: int):
        return super().__new__(cls, f"{module_id}:{kind}:{name}:{start_line}")
    
    @classmethod
    def from_db_row(cls, symbol_id: str) -> "SymbolId":
        module_id, kind, name, start_line = symbol_id.rsplit(":", 3)
        return cls(ModuleId.from_db_row(module_id), SymbolKind(kind), name, int(start_line))

@dataclass
class Symbol:
    id: SymbolId
    run_id: RunId
    module_id: ModuleId
    parent_symbol_id: Optional[SymbolId]
    name: str
    kind: SymbolKind
    doc: str
    location: str
    start_line: int
    end_line: int


    @classmethod
    def create(cls,run_id: RunId, row: Dict[str, Any]) -> "Symbol":
        return cls(
            id=SymbolId.from_db_row(symbol_id=row['id']),
            run_id=run_id,
            module_id=ModuleId.from_db_row(module_id=row['module_id']),
            parent_symbol_id=SymbolId.from_db_row(symbol_id=row['parent_symbol_id']) if row['parent_symbol_id'] else None,
            name=row['name'],
            kind=SymbolKind(row['kind']),
            doc=row['doc'],
            location=row['location'],
            start_line=row['start_line'],
            end_line=row['end_line']
        )