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