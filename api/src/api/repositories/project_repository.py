
from ..core import get_db
from fastapi import Depends
from asyncpg import Connection
from ..models import Project


class ProjectRepository:

    def __init__(self, db:Connection):
        self.db = db

    async def is_already_register(self, project_id: str) -> bool:
        query = "SELECT EXISTS(SELECT 1 FROM projects WHERE id = $1)"
        return bool(await self.db.fetchval(query, project_id))
    
    async def save(self, project: Project):
        query = """
        INSERT INTO projects (id, name, repo_url, branch)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (id) DO NOTHING
        """
        await self.db.execute(query, project.id, project.name, project.repo_url, project.branch)





def get_project_repository(db: Connection = Depends(get_db)) -> ProjectRepository:
    return ProjectRepository(db)