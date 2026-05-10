import asyncio

from fastapi import Depends

from api.src.api.repositories.run_repository import RunRepository, get_run_repository
from api.src.api.schemas import RunNotFoundError, RunNotDoneError
from ..repositories import SymbolRepository, RelationRepository, get_symbol_repository, get_relation_repository
from ..models import RunId, Symbol, Relation, Status
from typing import Tuple, List 


class GraphService:

    def __init__(
        self,
        symbol_repository: SymbolRepository,
        relation_repository: RelationRepository,
        run_repository: RunRepository,
    ) :
        self.symbol_repository = symbol_repository
        self.relation_repository = relation_repository
        self.run_repository = run_repository

    async def build_module_graph(self, run_id: RunId) -> Tuple[List[Symbol], List[Relation]]:
        run = await self.run_repository.get_run(run_id)
        if run is None:
            raise RunNotFoundError(f"Run with id {run_id} not found")
        if run.status not in (Status.Done, Status.PartialSuccess):
            raise RunNotDoneError(f"Run with id {run_id} is not done yet")
        
        symbols, relations = await asyncio.gather(                                              
            self.symbol_repository.get_root_symbols(run_id),                                  
            self.relation_repository.get_module_relations(run_id),  
        )    

        return symbols, relations


def get_graph_service(
    symbol_repo: SymbolRepository = Depends(get_symbol_repository),
    relation_repo: RelationRepository = Depends(get_relation_repository),
    run_repo: RunRepository = Depends(get_run_repository)
) -> GraphService:
    return GraphService(symbol_repo, relation_repo, run_repo)