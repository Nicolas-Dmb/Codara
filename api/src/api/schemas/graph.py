

from ..models import SymbolKind, RelationKind
from pydantic import BaseModel, ConfigDict


class ModuleResponse(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: str
    run_id: str
    relative_path: str
    name: str

class SymbolResponse(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: str
    run_id: str
    module_id: str
    parent_symbol_id: str | None
    name: str
    kind: SymbolKind
    doc: str
    location: str
    start_line: int
    end_line: int


class RelationResponse(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: str
    run_id: str
    module_id: str
    parent_symbol_id: str | None
    imported_name: str
    source_path: str
    target_symbol_id: str | None
    kind: RelationKind
    line: int



class SymbolGraph(BaseModel):
    symbols: list[SymbolResponse]
    relations: list[RelationResponse]

class ModuleGraph(BaseModel):
    modules: list[ModuleResponse]
    relations: list[RelationResponse]