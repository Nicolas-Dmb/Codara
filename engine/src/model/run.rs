


#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RunId(String);

impl RunId {
    pub fn new(project_id: &str, commit: &str) -> Self {
        Self(format!("{}_{}", project_id, commit))
    }

    pub fn value(&self) -> &str {
        &self.0
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
        let id = RunId::new(&project_id.value(), &commit);
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

    pub fn complete(&mut self, success: bool, error_message: Option<String>) {
        self.status = if success {
            RunStatus::Success
        } else {
            RunStatus::Failed
        };
        self.error_message = error_message;
        self.finished_at = Some(Utc::now());
    }
}