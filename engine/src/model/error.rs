#[derive(Debug, PartialEq)]
pub enum ProjectError {
    MissingRepositoryName,
    MissingNamespace,
    InvalidRepositoryUrl,
}

#[derive(Debug, PartialEq)]
pub enum RunError {
    EmptyRepository,
    ReadRepositoryFailed(String),
    CloneFailed(String),
}
