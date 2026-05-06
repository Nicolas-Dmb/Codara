from dataclasses import dataclass
from .project import ProjectId
from enum import Enum
from datetime import datetime
from typing import Optional


class RunId(str):
    "project_id: &ProjectId, commit: &str"

    def __new__(cls, project_id: ProjectId, commit: str):
        return super().__new__(cls, f"{project_id}:{commit}")


class Status(Enum):
    Pending = "pending"
    Processing = "processing"
    Done = "done"
    Failed = "failed"
    PartialSuccess = "partial_success"

    def __str__(self):
        return self.value


@dataclass
class Run:
    id: RunId
    project_id: ProjectId
    branch: str
    commit: str
    status: Status = Status.Pending
    error_message: Optional[str] = None
    started_at: Optional[datetime] = None
    finished_at: Optional[datetime] = None

    @classmethod
    def create(cls, project_id: ProjectId, branch: str, commit: str) -> "Run":
        return cls(
            id=RunId(project_id, commit),
            project_id=project_id,
            branch=branch,
            commit=commit,
        )
