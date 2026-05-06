
from contextlib import asynccontextmanager
from fastapi import FastAPI, Request
import asyncpg
from asyncpg import Connection
from typing import AsyncGenerator
from ..settings import settings

@asynccontextmanager
async def lifespan(app: FastAPI):
    app.state.db_pool = await asyncpg.create_pool(
        user=settings.db_user,
        password=settings.db_password,
        database=settings.db_name,
        host=settings.db_host,
        port=settings.db_port,
        min_size=1,
        max_size=5,
    )
    yield
    await app.state.db_pool.close()


async def get_db(request: Request) -> AsyncGenerator[Connection, None]:
    async with request.app.state.db_pool.acquire() as connection:
        yield connection