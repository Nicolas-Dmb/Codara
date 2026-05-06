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
    
    @staticmethod
    def from_str(status_str: str) -> 'Status|None':
        match status_str.lower():
            case "pending":
                return Status.Pending
            case "processing":
                return Status.Processing
            case "done":
                return Status.Done
            case "failed":
                return Status.Failed
            case "partial_success":
                return Status.PartialSuccess
            case _:
                return None

@dataclass
class Run:
    id: RunId
    project_id: ProjectId
    branch: str
    commit: str
    status: Status
    error_message: Optional[str]
    started_at: Optional[datetime] 
    finished_at: Optional[datetime]

    def __init__(self, project_id: ProjectId, branch: str, commit: str):
        self.id = RunId(project_id, commit)
        self.project_id = project_id
        self.branch = branch
        self.commit = commit
        self.status = Status.Pending
        self.error_message = None
        self.started_at = None
        self.finished_at = None
