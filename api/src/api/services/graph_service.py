from ..repositories import SymbolRepository

class GraphService:
       def __init__(
        self,
        project_repository: SymbolRepository,
    ):
        self.project_repository = project_repository