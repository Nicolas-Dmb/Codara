from ..repositories import RunRepository, ProjectRepository, CodebaseRepository
from ..schemas import AnalyseRequest, RepositoryNotFoundError, UnsupportedRepositoryProvider, RegisterNewRunError
from ..models import Run, Project, RunId, Status
from typing import Tuple

class AnalyseService:
    def __init__(self, run_repository: RunRepository, project_repository: ProjectRepository, codebase_repository: CodebaseRepository):
        self.run_repository = run_repository
        self.project_repository = project_repository
        self.codebase_repository = codebase_repository

    async def analyse(self, analyse_request: AnalyseRequest) -> Status:
        """Generate Run and Project models"""
        url = analyse_request.build_repo_url()
        if not url:
            raise UnsupportedRepositoryProvider(f"Unsupported repository provider: {analyse_request.provider}")
        
        commit = await self.get_last_commit(url, analyse_request.branch)
        if not commit:
            raise RepositoryNotFoundError("Failed to fetch the last commit from the codebase repository.")
        
        project, run = self.create_run(analyse_request, url, commit)

        if not await self.run_is_already_register(run.id):
            try:
                return await self.register_run(run, project)
            except Exception as e:
                raise RegisterNewRunError(f"Failed to register new run: {str(e)}")

        
        # TODO: check status then return the appropriate response (pending, processing, done, failed, partial_success)
        raise Exception("Run already exists. Status checking is not implemented yet.")

    async def get_last_commit(self, url: str, branch: str) -> str | None:
        commit = await self.codebase_repository.get_last_commit(url, branch)
        return commit
    
    def create_run(self, analyse_request: AnalyseRequest, url: str, commit: str) -> Tuple[Project, Run]:
        project = Project(analyse_request, url=url)  
        run = Run(project.id, analyse_request.branch, commit)
        return project, run
    
    async def run_is_already_register(self, run_id: RunId) -> bool:
        run_result= await self.run_repository.is_already_register(run_id)
        return run_result
    
    async def project_is_already_register(self, project_id: str) -> bool:
        project_result = await self.project_repository.is_already_register(project_id)
        return project_result
    
    async def register_run(self, run: Run, project: Project):
        if not await self.project_is_already_register(project.id):
            await self.project_repository.save(project)
        await self.run_repository.save(run)
        return run.status