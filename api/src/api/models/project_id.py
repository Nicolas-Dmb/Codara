class ProjectId(str):

    def __new__(cls, namespace: str, project_name: str):
        return super().__new__(cls, f"{namespace}:{project_name}")
    