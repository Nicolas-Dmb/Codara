import logging

from fastapi import FastAPI

from .core import configure_logging, lifespan, setup_cors
from .router import router

configure_logging()
logger = logging.getLogger(__name__)
logger.info("Starting the application...")

app = FastAPI(lifespan=lifespan, title="Aterminal API", version="1.0.0")

setup_cors(app)
app.include_router(router)