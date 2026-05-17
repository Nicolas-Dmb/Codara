from pydantic import BaseModel, ConfigDict
from typing import List

from .analyse import RunResponse


class ProjectResponse(BaseModel):
    model_config = ConfigDict(from_attributes=True)

    id: str
    name: str
    repo_url: str
    branch: str
    runs: List[RunResponse]

class ProjectsResponse(BaseModel):
    message: str
    projects: List[ProjectResponse]