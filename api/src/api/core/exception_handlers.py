import logging

from fastapi import FastAPI, Request, status
from fastapi.responses import JSONResponse

from ..schemas import (
    RegisterNewRunError,
    RepositoryNotFoundError,
    UnsupportedRepositoryProvider,
)

logger = logging.getLogger(__name__)


def _error(status_code: int, message: str) -> JSONResponse:
    return JSONResponse(status_code=status_code, content={"detail": message})


async def unsupported_provider_handler(_: Request, exc: Exception) -> JSONResponse:
    return _error(status.HTTP_422_UNPROCESSABLE_ENTITY, str(exc))


async def repository_not_found_handler(_: Request, exc: Exception) -> JSONResponse:
    return _error(status.HTTP_404_NOT_FOUND, str(exc))


async def run_already_exists_handler(_: Request, exc: Exception) -> JSONResponse:
    return _error(status.HTTP_409_CONFLICT, str(exc))


async def register_new_run_handler(_: Request, exc: Exception) -> JSONResponse:
    logger.exception("Failed to register new run", exc_info=exc)
    return _error(status.HTTP_500_INTERNAL_SERVER_ERROR, str(exc))


def register_exception_handlers(app: FastAPI) -> None:
    app.add_exception_handler(UnsupportedRepositoryProvider, unsupported_provider_handler)
    app.add_exception_handler(RepositoryNotFoundError, repository_not_found_handler)
    app.add_exception_handler(RegisterNewRunError, register_new_run_handler)