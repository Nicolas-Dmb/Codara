use crate::persistence::{AnalysisRepository, RunRepository, ProjectRepository};

pub struct Context<A: AnalysisRepository, R: RunRepository, P: ProjectRepository> {
    pub analysis_repo: A,
    pub run_repo: R,
    pub project_repo: P
}

impl<A: AnalysisRepository, R: RunRepository, P: ProjectRepository> Context<A, R, P> {
    pub fn new(analysis_repo: A, run_repo: R, project_repo: P) -> Self {
        Self { analysis_repo, run_repo, project_repo }
    }
}

