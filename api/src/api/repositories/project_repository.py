
from ..core import get_db
from fastapi import Depends
from asyncpg import Connection
from ..models import Project


class ProjectRepository:

    def __init__(self, db:Connection):
        self.db = db

    async def is_already_register(self, project_id: str) -> bool:
        query = "SELECT EXISTS(SELECT 1 FROM project WHERE id = $1)"
        return bool(await self.db.fetchval(query, project_id))
    
    async def save(self, project: Project):
        query = """
        INSERT INTO project (id, name, repo_url, branch)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (id) DO NOTHING
        """
        await self.db.execute(query, project.id, project.name, project.repo_url, project.branch)

    async def get_projects(self):
        query = "SELECT id, name, repo_url, branch FROM project"
        rows = await self.db.fetch(query)
        return [Project(**row, runs=[]) for row in rows]



def get_project_repository(db: Connection = Depends(get_db)) -> ProjectRepository:
    return ProjectRepository(db)