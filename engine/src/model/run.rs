use std::fmt;
use chrono::{DateTime, Utc};
use super::project::ProjectId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunId(String);

impl RunId {
    pub fn new(project_id: &ProjectId, commit: &str) -> Self {
        Self(format!("{}::{}", project_id, commit))
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
    Running,
    Success,
    Failed,
}

#[derive(Debug)]
pub struct Run {
    pub id: RunId,
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
            branch,
            commit,
            status: RunStatus::Pending,
            error_message: None,
            started_at: None,
            finished_at: None,
        }
    }

    pub fn start(&mut self) {
        self.status = RunStatus::Running;
        self.started_at = Some(Utc::now());
    }

    pub fn succeed(&mut self) {
        self.status = RunStatus::Success;
        self.finished_at = Some(Utc::now());
    }

    pub fn fail(&mut self, error: String) {
        self.status = RunStatus::Failed;
        self.error_message = Some(error);
        self.finished_at = Some(Utc::now());
    }
}
