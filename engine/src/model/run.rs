use std::fmt;
use std::str::FromStr;
use chrono::{DateTime, Utc};
use super::project::ProjectId;
use super::error::ServiceError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunId(String);

impl RunId {
    pub fn new(project_id: &ProjectId, commit: &str) -> Self {
        Self(format!("{}::{}", project_id, commit))
    }

    pub fn from_raw(raw: String) -> Self {
        Self(raw)
    }
}

impl fmt::Display for RunId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RunStatus {
    Pending,
    Processing,
    Done,
    Failed,
    PartialSuccess,
}

impl fmt::Display for RunStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunStatus::Pending => f.write_str("pending"),
            RunStatus::Processing => f.write_str("processing"),
            RunStatus::Done => f.write_str("done"),
            RunStatus::Failed => f.write_str("failed"),
            RunStatus::PartialSuccess => f.write_str("partial_success"),
        }
    }
}

impl FromStr for RunStatus {
    type Err = ServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(RunStatus::Pending),
            "processing" => Ok(RunStatus::Processing),
            "done" => Ok(RunStatus::Done),
            "failed" => Ok(RunStatus::Failed),
            "partial_success" => Ok(RunStatus::PartialSuccess),
            other => Err(ServiceError::DatabaseRequestFailed(
                format!("Unknown run status: {}", other),
            )),
        }
    }
}

#[derive(Debug)]
pub struct Run {
    pub id: RunId,
    pub project_id: ProjectId,
    pub branch: String,
    pub commit: String,
    pub status: RunStatus,
    pub error_message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl Run {
    pub fn new(project_id: ProjectId, branch: String, commit: String) -> Self {
        let id = RunId::new(&project_id, &commit);
        Self {
            id,
            project_id,
            branch,
            commit,
            status: RunStatus::Pending,
            error_message: None,
            started_at: None,
            finished_at: None,
        }
    }

    pub fn succeed(&mut self) {
        self.status = RunStatus::Done;
        self.finished_at = Some(Utc::now());
    }

    pub fn fail(&mut self, error: String) {
        self.status = RunStatus::Failed;
        self.error_message = Some(error);
        self.finished_at = Some(Utc::now());
    }

    /// Use this when the run completes with retryable issues
    /// It will used when implementing the "retry" feature. 
    pub fn partial_success(&mut self) {
        self.status = RunStatus::PartialSuccess;
        self.finished_at = Some(Utc::now());
    }
}
