import logging
from fastapi import APIRouter, Depends
from fastapi.responses import JSONResponse 
from .schemas import AnalyseRequest
from .repositories import get_run_repository, get_project_repository, get_codebase_repository, RunRepository, ProjectRepository, CodebaseRepository
from .services import AnalyseService
from .models import Status

router = APIRouter()
logger = logging.getLogger(__name__)


@router.post("/analyse")
async def analyse(analyse: AnalyseRequest, run_repo : RunRepository = Depends(get_run_repository), project_repo : ProjectRepository = Depends(get_project_repository), codebase_repo : CodebaseRepository = Depends(get_codebase_repository))-> JSONResponse:
    try: 
        service = AnalyseService(run_repo, project_repo, codebase_repo)
        status = await service.analyse(analyse)
        if status == Status.Pending or status == Status.Processing:
            return JSONResponse(content={"message": f"Analysis request received." , "status": str(status)}, status_code=202)
        else :
            # TODO : implemented other status case (failed, partial success, done)
            return JSONResponse(content={"message": f"Analysis request already exists with status: {status}."}, status_code=200)
    except Exception as e:
        logger.error(f"Error processing analysis request: {e}")
        return JSONResponse(content={"message": f"Error processing analysis request: {str(e)}"}, status_code=400)