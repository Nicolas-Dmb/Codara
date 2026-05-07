

class RepositoryError(Exception):
    """Base class for repository errors."""
    pass

class RepositoryNotFoundError(RepositoryError):
    """Raised when the repository is not found.
        it could be raised if repository url is invalid 
        or if the repository is private and the credentials are not provided.
    """
    pass

class UnsupportedRepositoryProvider(RepositoryError):
    """Raised when the repository provider is not supported."""
    pass

class RunError(Exception):
    """Base class for run errors."""
    pass

class StatusError(RunError):
    """Raised when there is an error with the run status."""
    pass

class RegisterNewRunError(RunError):
    """Raised when there is an error while registering a new run or new project."""
    pass