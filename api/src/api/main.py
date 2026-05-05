import logging

from fastapi import FastAPI
from .router import router
from .core import lifespan, setup_cors

logging.basicConfig(level=logging.INFO)
logging.info("Starting the application...")

app = FastAPI(lifespan=lifespan, title="Aterminal API", version="1.0.0")

setup_cors(app)
app.include_router(router)