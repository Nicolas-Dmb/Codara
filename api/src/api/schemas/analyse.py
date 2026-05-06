from pydantic import BaseModel


class AnalyseRequest(BaseModel):
    provider: str
    namespace: str
    project_name: str
    branch: str

    def build_repo_url(self) -> str|None:
        match self.provider:
            case "github":
                return f"https://github.com/{self.namespace}/{self.project_name}.git"
            case "gitlab":
                return f"https://gitlab.com/{self.namespace}/{self.project_name}.git"
            case "bitbucket":
                return f"https://bitbucket.org/{self.namespace}/{self.project_name}.git"
            case _:
                return None