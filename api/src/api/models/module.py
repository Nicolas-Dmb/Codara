from api.src.api.models.project import ProjectId


class ModuleId(str):
    
    def __new__(cls, project_id: ProjectId, relative_path: str):
        return super().__new__(cls, f"{project_id}::{relative_path}")
    
    @classmethod
    def from_db_row(cls, module_id: str) -> "ModuleId":
        project_id, relative_path = module_id.split("::", 1)
        namespace, project_name = project_id.split(":", 1)  
        return cls(ProjectId(namespace, project_name), relative_path)