import logging

from fastapi import FastAPI
from .config import setup_cors
from .router import router

logging.basicConfig(level=logging.INFO)
logging.info("Starting the application...")

app = FastAPI()

setup_cors(app)
app.include_router(router)