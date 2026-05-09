from fastapi import APIRouter, Depends, status

from .schemas import AnalyseRequest, AnalyseResponse, RunResponse
from .services import AnalyseService, get_analyse_service

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
    "/graph/{run_id}/symbols",
    response_model=RunResponse,
    status_code=status.HTTP_200_OK,
)
async def get_symbols(
    run_id: str,
    service: AnalyseService = Depends(get_analyse_service),
) -> RunResponse:
    """
        Endpoint to get a run by its id.
    """
    run = await service.get_run(run_id)
    return RunResponse.model_validate(run)