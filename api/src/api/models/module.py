from ..models import ProjectId, RunId
from typing import Dict, Any
from dataclasses import dataclass


class ModuleId(str):
    
    def __new__(cls, project_id: ProjectId, relative_path: str):
        return super().__new__(cls, f"{project_id}::{relative_path}")
    
    @classmethod
    def from_db_row(cls, module_id: str) -> "ModuleId":
        project_id, relative_path = module_id.split("::", 1)
        namespace, project_name = project_id.split(":", 1)  
        return cls(ProjectId(namespace, project_name), relative_path)

@dataclass
class Module:
    id: ModuleId
    run_id: RunId
    relative_path: str
    name: str

    @classmethod    
    def from_db_row(cls, row: Dict[str, Any], run_id: RunId) -> "Module":
        return cls(
            id=ModuleId.from_db_row(row["id"]),
            run_id=run_id,
            relative_path=row["relative_path"],
            name=row["name"]
        )