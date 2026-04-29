use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ProjectError {
    #[error("missing repository name in URL")]
    MissingRepositoryName,
    #[error("missing namespace in URL")]
    MissingNamespace,
    #[error("invalid repository URL")]
    InvalidRepositoryUrl,
}

#[derive(Debug, Error, PartialEq)]
pub enum RunError {
    #[error("repository is empty")]
    EmptyRepository,
    #[error("failed to read repository: {0}")]
    ReadRepositoryFailed(String),
    #[error("clone failed: {0}")]
    CloneFailed(String),
}
