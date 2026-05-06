from dataclasses import dataclass
from ..schemas import AnalyseRequest

class ProjectId(str):
    
    def __new__(cls, namespace: str, project_name: str):
        return super().__new__(cls, f"{namespace}:{project_name}")



@dataclass
class Project:
    id: ProjectId
    name: str
    repo_url: str
    branch: str

    def __init__(self, request: AnalyseRequest, url:str):
        self.id = ProjectId(request.namespace, request.project_name)
        self.name = request.project_name
        self.repo_url = url
        self.branch = request.branch
    