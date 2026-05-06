from typing import Literal
from pydantic import BaseModel

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


class AnalyseResponse(BaseModel):
    message: str
    status: Status