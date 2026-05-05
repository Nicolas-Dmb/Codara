use crate::persistence::{AnalysisRepository, RunRepository, ProjectRepository};
use crate::services::Cloner;

pub struct Context<A: AnalysisRepository, R: RunRepository, P: ProjectRepository, C: Cloner> {
    pub analysis_repo: A,
    pub run_repo: R,
    pub project_repo: P,
    pub cloner: C,
}

impl<A: AnalysisRepository, R: RunRepository, P: ProjectRepository, C: Cloner> Context<A, R, P, C> {
    pub fn new(analysis_repo: A, run_repo: R, project_repo: P, cloner: C) -> Self {
        Self { analysis_repo, run_repo, project_repo, cloner }
    }
}

