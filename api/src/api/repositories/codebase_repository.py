import requests
import logging

class CodebaseRepository:

    async def get_last_commit(self, url: str, branch: str) -> str|None:
        response = requests.get(f"{url}/branches/{branch}")
        return dezerialize_commit(response)

def dezerialize_commit(response: requests.Response) -> str|None:
    if response.status_code == 200:
        data = response.json()
        return data.get("commit", {}).get("sha", "")
    
    logging.error(f"Failed to fetch last commit: {response.status_code} - {response.text}")
    return None

def get_codebase_repository()-> CodebaseRepository:
    return CodebaseRepository()