from fastapi import APIRouter, Depends, status

from api.src.api.schemas.graph import SymbolGraph

from .schemas import AnalyseRequest, AnalyseResponse, RunResponse, SymbolResponse, RelationResponse
from .services import AnalyseService, get_analyse_service, GraphService, get_graph_service
from .models import RunId

router = APIRouter()


@router.post(
    "/analyse",
    response_model=AnalyseResponse,
    status_code=status.HTTP_202_ACCEPTED,
)
async def analyse(
    request: AnalyseRequest,
    service: AnalyseService = Depends(get_analyse_service),
) -> AnalyseResponse:
    """
        Endpoint to initiate an run and project if not already existing.
        else return the existing run and project.
    """
    run = await service.analyse(request)
    return AnalyseResponse(
        message="Analysis request received.",
        run=RunResponse.model_validate(run),
    )

@router.get(
    "/graph/{run_id}",
    response_model=SymbolGraph,
    status_code=status.HTTP_200_OK,
)
async def get_symbols(
    run_id: str,
    service: GraphService = Depends(get_graph_service),
) -> SymbolGraph:
    """
        Endpoint to get the symbols and relations of a run.
    """
    symbols, relations = await service.build_module_graph(RunId.from_str(run_id))
    return SymbolGraph(
        symbols=[SymbolResponse.model_validate(symbol) for symbol in symbols], 
        relations=[RelationResponse.model_validate(relation) for relation in relations],
    )
    