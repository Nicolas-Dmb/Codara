from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from ..settings import settings


def setup_cors(app: FastAPI) -> None:

    app.add_middleware(
        CORSMiddleware,
        allow_origins=settings.cors_origins,
        allow_credentials=False,
        allow_methods=["GET", "POST", "OPTIONS"],
        allow_headers=["Content-Type", "Authorization"],
    )
