from typing import Tuple

from fastapi import Depends

from ..models import Project, ProjectId, Run, RunId
from ..repositories import (
    CodebaseRepository,
    ProjectRepository,
    RunRepository,
    get_codebase_repository,
    get_project_repository,
    get_run_repository,
)
from ..schemas import (
    AnalyseRequest,
    RegisterNewRunError,
    RepositoryNotFoundError,
    RunAlreadyExistsError,
)


class AnalyseService:
    def __init__(
        self,
        run_repository: RunRepository,
        project_repository: ProjectRepository,
        codebase_repository: CodebaseRepository,
    ):
        self.run_repository = run_repository
        self.project_repository = project_repository
        self.codebase_repository = codebase_repository

    async def analyse(self, analyse_request: AnalyseRequest) -> Run:
        """Generate Run and Project models"""
        url = analyse_request.build_clone_url()

        commit = await self.get_last_commit(analyse_request)
        if not commit:
            raise RepositoryNotFoundError(
                "Failed to fetch the last commit from the codebase repository."
            )

        project, run = self.initialize_run(analyse_request, url, commit)

        if await self.run_is_already_register(run.id):
            # TODO: check status then return the appropriate response
            #       (pending, processing, done, failed, partial_success)
            raise RunAlreadyExistsError(
                f"Run {run.id} already exists. Status checking is not implemented yet."
            )

        try:
            return await self.register_run(run, project)
        except Exception as e:
            raise RegisterNewRunError(f"Failed to register new run: {e}") from e

    async def get_last_commit(self, request: AnalyseRequest) -> str | None:
        return await self.codebase_repository.get_last_commit(
            request.provider, request.namespace, request.project_name, request.branch
        )

    def initialize_run(
        self, analyse_request: AnalyseRequest, url: str, commit: str
    ) -> Tuple[Project, Run]:
        project = Project.from_request(analyse_request, url)
        run = Run.create(project.id, analyse_request.branch, commit)
        return project, run

    async def run_is_already_register(self, run_id: RunId) -> bool:
        return await self.run_repository.is_already_register(run_id)

    async def project_is_already_register(self, project_id: ProjectId) -> bool:
        return await self.project_repository.is_already_register(project_id)

    async def register_run(self, run: Run, project: Project) -> Run:
        if not await self.project_is_already_register(project.id):
            await self.project_repository.save(project)
        await self.run_repository.save(run)
        return run


def get_analyse_service(
    run_repo: RunRepository = Depends(get_run_repository),
    project_repo: ProjectRepository = Depends(get_project_repository),
    codebase_repo: CodebaseRepository = Depends(get_codebase_repository),
) -> AnalyseService:
    return AnalyseService(run_repo, project_repo, codebase_repo)