from fastapi import Depends

from ..schemas import RunNotFoundError, RunNotDoneError
from ..repositories import RelationRepository, get_relation_repository, RunRepository, get_run_repository, ModuleRepository, get_module_repository
from ..models import RunId, Relation, Status, Module
from typing import Tuple, List


class GraphService:

    def __init__(
        self,
        module_repository: ModuleRepository,
        relation_repository: RelationRepository,
        run_repository: RunRepository,
    ) :
        self.module_repository = module_repository
        self.relation_repository = relation_repository
        self.run_repository = run_repository

    async def build_module_graph(self, run_id: RunId) -> Tuple[List[Module], List[Relation]]:
        run = await self.run_repository.get_run(run_id)
        if run is None:
            raise RunNotFoundError(f"Run with id {run_id} not found")
        if run.status not in (Status.Done, Status.PartialSuccess):
            raise RunNotDoneError(f"Run with id {run_id} is not done yet")

        modules = await self.module_repository.get_modules(run_id)
        relations = await self.relation_repository.get_module_relations(run_id)

        return modules, relations


def get_graph_service(
    module_repo: ModuleRepository = Depends(get_module_repository),
    relation_repo: RelationRepository = Depends(get_relation_repository),
    run_repo: RunRepository = Depends(get_run_repository)
) -> GraphService:
    return GraphService(module_repo, relation_repo, run_repo)