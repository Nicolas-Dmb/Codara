from dataclasses import dataclass
from typing import TYPE_CHECKING

from .run import Run
from .project_id import ProjectId

if TYPE_CHECKING:
    from ..schemas import AnalyseRequest


@dataclass
class Project:
    id: ProjectId
    name: str
    repo_url: str
    branch: str
    runs: list[Run]

    @classmethod
    def from_request(cls, request: "AnalyseRequest", url: str) -> "Project":
        return cls(
            id=ProjectId(request.namespace, request.project_name),
            name=request.project_name,
            repo_url=url,
            branch=request.branch,
            runs=[]
        )
    
    def add_run(self, run: Run):
        self.runs.append(run)