from dataclasses import dataclass
from typing import TYPE_CHECKING

if TYPE_CHECKING:
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

    @classmethod
    def from_request(cls, request: "AnalyseRequest", url: str) -> "Project":
        return cls(
            id=ProjectId(request.namespace, request.project_name),
            name=request.project_name,
            repo_url=url,
            branch=request.branch,
        )