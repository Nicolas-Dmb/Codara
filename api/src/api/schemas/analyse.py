from datetime import datetime
from typing import Literal

from pydantic import BaseModel, ConfigDict

from ..models import Status

Provider = Literal["github", "gitlab", "bitbucket"]


class AnalyseRequest(BaseModel):
    provider: Provider
    namespace: str
    project_name: str
    branch: str

    def build_clone_url(self) -> str:
        match self.provider:
            case "github":
                return f"https://github.com/{self.namespace}/{self.project_name}.git"
            case "gitlab":
                return f"https://gitlab.com/{self.namespace}/{self.project_name}.git"
            case "bitbucket":
                return f"https://bitbucket.org/{self.namespace}/{self.project_name}.git"


class RunResponse(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: str
    project_id: str
    branch: str
    commit: str
    status: Status
    error_message: str | None = None
    started_at: datetime | None = None
    finished_at: datetime | None = None


class AnalyseResponse(BaseModel):
    message: str
    run: RunResponse
