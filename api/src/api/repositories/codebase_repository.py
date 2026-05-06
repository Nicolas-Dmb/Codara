import logging
from urllib.parse import quote
import httpx

logger = logging.getLogger(__name__)


class CodebaseRepository:

    async def get_last_commit(
        self, provider: str, namespace: str, project_name: str, branch: str
    ) -> str | None:
        match provider:
            case "github":
                url = f"https://api.github.com/repos/{namespace}/{project_name}/branches/{branch}"
                commit_path = ("commit", "sha")
            case "gitlab":
                project_id = quote(f"{namespace}/{project_name}", safe="")
                url = f"https://gitlab.com/api/v4/projects/{project_id}/repository/branches/{quote(branch, safe='')}"
                commit_path = ("commit", "id")
            case "bitbucket":
                url = f"https://api.bitbucket.org/2.0/repositories/{namespace}/{project_name}/refs/branches/{branch}"
                commit_path = ("target", "hash")
            case _:
                return None

        try:
            async with httpx.AsyncClient(timeout=10.0) as client:
                response = await client.get(url)
        except httpx.HTTPError as e:
            logger.error(f"Failed to fetch last commit from {url}: {e}")
            return None

        if response.status_code != 200:
            logger.error(
                f"Failed to fetch last commit from {url}: "
                f"{response.status_code} - {response.text}"
            )
            return None

        data = response.json()
        outer, inner = commit_path
        sha = data.get(outer, {}).get(inner)
        return sha or None


def get_codebase_repository() -> CodebaseRepository:
    return CodebaseRepository()