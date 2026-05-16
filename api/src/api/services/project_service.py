from fastapi import Depends

from ..repositories import (
    ProjectRepository,
    RunRepository,
    get_project_repository,
    get_run_repository,
)

class ProjectService:
    def __init__(
        self,
        run_repository: RunRepository,
        project_repository: ProjectRepository,
    ):
        self.run_repository = run_repository
        self.project_repository = project_repository

    
    async def get_projects_with_runs(self):
        projects = await self.project_repository.get_projects()
        for project in projects:
            runs = await self.run_repository.get_runs_by_project_id(project.id)
            project.runs = runs
        return projects



def get_project_service(
    run_repo: RunRepository = Depends(get_run_repository),
    project_repo: ProjectRepository = Depends(get_project_repository),
) -> ProjectService:
    return ProjectService(run_repo, project_repo)