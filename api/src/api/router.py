from fastapi import APIRouter, Depends, status

from .schemas import AnalyseRequest, AnalyseResponse
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
    run_status = await service.analyse(request)
    return AnalyseResponse(message="Analysis request received.", status=run_status)