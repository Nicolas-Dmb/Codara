from fastapi import APIRouter, Depends, status

from .schemas import AnalyseRequest, AnalyseResponse, RunResponse, RelationResponse, ProjectsResponse, ProjectResponse, ModuleGraph, ModuleResponse
from .services import AnalyseService, get_analyse_service, GraphService, get_graph_service, ProjectService, get_project_service
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
    "/analyse/{run_id}",
    response_model=AnalyseResponse,
    status_code=status.HTTP_200_OK,
)
async def get_analyse_status(
    run_id: str,
    service: AnalyseService = Depends(get_analyse_service),
) -> AnalyseResponse:
    """
        Endpoint to get the status of an analysis run.
    """
    run = await service.get_run(RunId.from_str(run_id))
    return AnalyseResponse(
        message="Analysis status retrieved.",
        run=RunResponse.model_validate(run),
    )

@router.get(
    "/graph/{run_id}",
    response_model=ModuleGraph,
    status_code=status.HTTP_200_OK,
)
async def module_graph(
    run_id: str,
    service: GraphService = Depends(get_graph_service),
) -> ModuleGraph:
    """
        Endpoint to get the modules and relations of a run.
    """
    modules, relations = await service.build_module_graph(RunId.from_str(run_id))
    return ModuleGraph(
        modules=[ModuleResponse.model_validate(module) for module in modules],
        relations=[RelationResponse.model_validate(relation) for relation in relations],
    )
    

@router.get(
    "/projects",
    status_code=status.HTTP_200_OK,
)
async def get_projects(
    service: ProjectService = Depends(get_project_service),
) -> ProjectsResponse:
    projects = await service.get_projects_with_runs()
    return ProjectsResponse(
        message="Projects retrieved.",
        projects=[ProjectResponse.model_validate(project) for project in projects]
    )